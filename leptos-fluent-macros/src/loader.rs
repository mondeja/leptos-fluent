use crate::{
    build_fluent_resources_and_file_paths,
    cookie::validate_cookie_attrs,
    languages::{read_languages_file, read_locales_folder},
    FluentFilePaths, ParsedLanguage,
};
use crate::{
    fluent_entries::build_fluent_entries,
    tr_macros::{
        gather_tr_macro_defs_from_globstr, gather_tr_macro_defs_from_workspace,
    },
};
use quote::ToTokens;
use std::path::PathBuf;
use std::rc::Rc;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token, Result,
};

fn parse_litstr_or_expr_param(
    input: ParseStream,
    strlit: &mut Option<syn::LitStr>,
    expr: &mut Option<TokenStreamStr>,
    param_name: &'static str,
) -> Result<()> {
    if input.peek(syn::LitStr) {
        let lit = input.parse::<syn::LitStr>()?;
        *strlit = Some(lit);
        return Ok(());
    }

    let span = input.span();

    if input.peek(syn::LitBool) {
        let value = input.parse::<syn::LitBool>()?.value.to_string();
        return Err(syn::Error::new(
            span,
            format!(
                concat!(
                    "Invalid value for '{}' of leptos_fluent! macro.",
                    " Expected literal string or valid expression.",
                    " Found a literal boolean `{}`.",
                ),
                param_name, &value,
            ),
        ));
    }

    match input.parse::<syn::Expr>() {
        Ok(e) => {
            *expr = Some(TokenStreamStr::from(
                e.to_token_stream().to_string().as_str(),
            ));
            Ok(())
        }
        Err(_) => Err(syn::Error::new(
            span,
            format!(
                concat!(
                    "Invalid value for '{}' of leptos_fluent! macro.",
                    " Expected literal string or valid expression.",
                ),
                param_name,
            ),
        )),
    }
}

fn parse_litstr_or_expr_param_noop(
    input: ParseStream,
    param_name: &'static str,
) -> Result<()> {
    parse_litstr_or_expr_param(input, &mut None, &mut None, param_name)
}

macro_rules! parse_litstr_or_expr_param_with_maybe_comptime_exprpath {
    ($exprpath:ident, $k:ident, $input:ident, $param:ident, $param_name:literal) => {
        if let Some(ref e) = $exprpath {
            let evaluated_exprpath = $crate::evaluate_exprpath(e);
            if !evaluated_exprpath.supported {
                return Err(syn::Error::new(
                    e.span(),
                    exprpath_not_supported_error_message(
                        e.to_string().as_str(),
                        &$k,
                    ),
                ));
            } else if !evaluated_exprpath.result {
                parse_litstr_or_expr_param_noop(&$input, $param_name)?;
            } else {
                parse_litstr_or_expr_param(
                    &$input,
                    &mut $param.lit,
                    &mut $param.expr,
                    $param_name,
                )?;
            }
        } else {
            parse_litstr_or_expr_param(
                &$input,
                &mut $param.lit,
                &mut $param.expr,
                $param_name,
            )?;
        }
    };
}

fn parse_litbool_or_expr_param(
    input: ParseStream,
    expr: &mut Option<TokenStreamStr>,
    param_name: &'static str,
) -> Result<()> {
    let span = input.span();
    match input.parse::<syn::LitBool>() {
        Ok(lit) => {
            *expr = Some(TokenStreamStr::from(lit.value.to_string().as_str()));
            Ok(())
        }
        Err(_) => match input.parse::<syn::Expr>() {
            Ok(e) => {
                *expr = Some(TokenStreamStr::from(
                    e.to_token_stream().to_string().as_str(),
                ));
                Ok(())
            }
            Err(_) => Err(syn::Error::new(
                span,
                format!(
                    concat!(
                        "Invalid value for '{}' of leptos_fluent! macro.",
                        " Expected literal boolean or valid expression.",
                    ),
                    param_name,
                ),
            )),
        },
    }
}

macro_rules! parse_struct_field_init_shorthand {
    ($shorthand:ident, $param:ident, $k_token_stream_str:ident) => {
        if $shorthand {
            $param.expr = Some($k_token_stream_str);
            continue;
        }
    };
    ($shorthand:ident, $param:ident, $k_token_stream_str:ident, $vec:ident) => {
        if $shorthand {
            $param.expr = Some($k_token_stream_str);
            $vec.push($param);
            continue;
        }
    };
}

/// A syntax part consisting of a list of syn paths.
///
/// ```rust,ignore
/// translations: [loader1, loader2],
/// //            ^^^^^^^^^^^^^^^^^^
/// ```
///
/// Must not contain a [`Compound`] loader.
pub(crate) struct Simple(pub(crate) Vec<syn::Path>);

impl Parse for Simple {
    fn parse(input: ParseStream) -> Result<Self> {
        let bracketed;
        syn::bracketed!(bracketed in input);

        let list =
            bracketed.parse_terminated(syn::Path::parse, syn::Token![,])?;
        Ok(Self(list.into_iter().collect()))
    }
}

/// A syntax part consisting of a group of loaders passed as one.
///
/// Used to pack loaders and export them from a crate for example.
pub(crate) struct Compound(pub(crate) syn::Path);

impl Parse for Compound {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

/// Either a [`List`] or a [`Compound`].
pub(crate) enum SimpleOrCompound {
    Simple(Simple),
    Compound(Compound),
}

impl Parse for SimpleOrCompound {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(list) = Simple::parse(input) {
            Ok(Self::Simple(list))
        } else if let Ok(compound) = Compound::parse(input) {
            Ok(Self::Compound(compound))
        } else {
            Err(syn::Error::new(
                input.span(),
                "need to pass either a list of loaders or a compound loader",
            ))
        }
    }
}

/// A collection of loaders (both simple and compound ones) to use
/// for translating.
pub(crate) struct Translations(Rc<str>);

impl Parse for Translations {
    fn parse(input: ParseStream) -> Result<Self> {
        // example of input
        // [loader1, loader2] + loaders1 + loaders2 + [loader3]

        let loaders = syn::punctuated::Punctuated::<
            SimpleOrCompound,
            syn::Token![+],
        >::parse_separated_nonempty(input)?;

        if loaders.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "Need to pass at least one translations loader",
            ));
        }

        let mut translations_quote =
            "{let mut loaders = Vec::new();".to_string();
        for loader in loaders.into_iter() {
            match loader {
                SimpleOrCompound::Simple(x) => {
                    for loader in x.0.into_iter() {
                        translations_quote.push_str(&format!(
                            "loaders.push(&{});",
                            loader.to_token_stream()
                        ));
                    }
                }
                SimpleOrCompound::Compound(compound_loader) => {
                    translations_quote.push_str("loaders.extend(");
                    translations_quote.push_str(
                        &compound_loader.0.to_token_stream().to_string(),
                    );
                    translations_quote.push_str(");");
                }
            }
        }

        translations_quote.push_str("loaders}");
        Ok(Self(Rc::from(translations_quote)))
    }
}

impl ToTokens for Translations {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(proc_macro2::TokenStream::from_iter(
            self.0
                .to_string()
                .parse::<proc_macro2::TokenStream>()
                .unwrap(),
        ));
    }
}

fn exprpath_not_supported_error_message(expr: &str, k: &syn::Ident) -> String {
    format!(
        concat!(
            "The parameter '{}' of leptos_fluent! macro does not accept the",
            " expression path '{}'. Consider to move your configuration to a",
            " variable:\n\n",
            "```rust
{}
{{
    let {}_dyn = {{ ... }};
}}

leptos_fluent! {{
    // ...
    {}: {}_dyn,
}};
```
",
        ),
        k, expr, expr, k, k, k,
    )
}

fn compile_time_exprpath_not_supported_error_message(
    expr: &str,
    k: &syn::Ident,
) -> String {
    format!(
        concat!(
            "The parameter '{}' of leptos_fluent! macro does not accept the",
            " compile-time expression path '{}'."
        ),
        k, expr,
    )
}

macro_rules! evaluate_compile_time_exprpath_set_none {
    ($exprpath:ident, $k:ident, $field:ident) => {
        if let Some(ref e) = $exprpath {
            let evaluated_exprpath = $crate::evaluate_exprpath(e);
            if !evaluated_exprpath.supported {
                return Err(syn::Error::new(
                    e.span(),
                    compile_time_exprpath_not_supported_error_message(
                        e.to_string().as_str(),
                        &$k,
                    ),
                ));
            } else if !evaluated_exprpath.result {
                $field = None;
            }
        }
    };
}

macro_rules! parse_runtime_exprpath {
    ($exprpath:ident, $param:ident) => {
        if let Some(ref path) = $exprpath {
            $param.exprpath = Some(path.as_str().into());
        }
    };
}

fn check_struct_field_init_shorthand(
    struct_field_init_shorthand: bool,
    k: &syn::Ident,
) -> Result<()> {
    if struct_field_init_shorthand {
        Err(syn::Error::new(
            k.span(),
            format!(
                concat!(
                    "Struct field initialization shorthand is not supported",
                    " for the parameter '{}'.",
                ),
                k,
            ),
        ))
    } else {
        Ok(())
    }
}

/// Abstract implementation for token streams expressions.
///
/// This is used to parse expressions that are not string literals, like
/// expressions and literal booleans.
#[derive(Debug)]
pub(crate) struct TokenStreamStr(Rc<str>);

impl ToTokens for TokenStreamStr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token_stream =
            self.0.to_string().parse::<proc_macro2::TokenStream>();
        tokens.extend(token_stream);
    }
}

impl Parse for TokenStreamStr {
    fn parse(input: ParseStream) -> Result<Self> {
        let token_stream = input.parse::<proc_macro2::TokenStream>()?;
        Ok(Self(Rc::from(token_stream.to_string())))
    }
}

impl From<&str> for TokenStreamStr {
    fn from(s: &str) -> Self {
        Self(Rc::from(s))
    }
}

#[derive(Default)]
pub(crate) struct LitBoolExprOrIdent {
    pub expr: Option<TokenStreamStr>,
    pub exprpath: Option<TokenStreamStr>,
}

impl LitBoolExprOrIdent {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub(crate) struct LitStrExpr {
    pub lit: Option<syn::LitStr>,
    pub expr: Option<TokenStreamStr>,
}

impl LitStrExpr {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub(crate) struct LitBool {
    pub lit: Option<bool>,
    pub exprpath: Option<TokenStreamStr>,
}

impl LitBool {
    pub fn new() -> Self {
        Self::default()
    }
}

pub(crate) enum LitBoolOrStr {
    Bool(syn::LitBool),
    Str(syn::LitStr),
}

impl Parse for LitBoolOrStr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::LitBool) {
            Ok(LitBoolOrStr::Bool(input.parse()?))
        } else if input.peek(syn::LitStr) {
            Ok(LitBoolOrStr::Str(input.parse()?))
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected a literal boolean or a literal string",
            ))
        }
    }
}

impl LitBoolOrStr {
    fn span(&self) -> proc_macro2::Span {
        match self {
            LitBoolOrStr::Bool(lit) => lit.span(),
            LitBoolOrStr::Str(lit) => lit.span(),
        }
    }
}

pub(crate) struct I18nLoader {
    pub warnings: Vec<proc_macro_warning::Warning>,
    pub fluent_file_paths: FluentFilePaths,
    pub children: Vec<LitBoolExprOrIdent>,
    pub translations: Option<Translations>,
    pub languages: Vec<ParsedLanguage>,
    pub languages_path: Option<String>,
    pub default_language: Option<(String, usize)>,
    pub raw_languages_path: Option<String>,
    pub locales_path: String,
    pub core_locales_path: Option<String>,
    pub check_translations: Option<LitBoolOrStr>,
    pub fill_translations: Option<String>,
    pub customise: Option<syn::ExprClosure>,
    pub provide_meta_context: Vec<LitBool>,
    pub sync_html_tag_lang: Vec<LitBoolExprOrIdent>,
    pub sync_html_tag_dir: Vec<LitBoolExprOrIdent>,
    pub url_param: LitStrExpr,
    pub initial_language_from_url_param: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_param_to_local_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_param_to_session_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_param_to_cookie: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_param_to_server_function:
        Vec<LitBoolExprOrIdent>,
    pub set_language_to_url_param: Vec<LitBoolExprOrIdent>,
    pub local_storage_key: LitStrExpr,
    pub initial_language_from_local_storage: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_local_storage_to_cookie: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_local_storage_to_session_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_local_storage_to_server_function:
        Vec<LitBoolExprOrIdent>,
    pub set_language_to_local_storage: Vec<LitBoolExprOrIdent>,
    pub session_storage_key: LitStrExpr,
    pub initial_language_from_session_storage: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_session_storage_to_cookie:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_session_storage_to_local_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_session_storage_to_server_function:
        Vec<LitBoolExprOrIdent>,
    pub set_language_to_session_storage: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_navigator: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_navigator_to_local_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_navigator_to_session_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_navigator_to_cookie: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_navigator_to_server_function:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_accept_language_header: Vec<LitBoolExprOrIdent>,
    pub set_language_from_navigator: Vec<LitBoolExprOrIdent>,
    pub cookie_name: LitStrExpr,
    pub cookie_attrs: LitStrExpr,
    pub initial_language_from_cookie: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_cookie_to_local_storage: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_cookie_to_session_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_cookie_to_server_function:
        Vec<LitBoolExprOrIdent>,
    pub set_language_to_cookie: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_server_function: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_server_function_to_cookie:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_server_function_to_local_storage:
        Vec<LitBoolExprOrIdent>,
    pub set_language_to_server_function: Vec<LitBoolExprOrIdent>,
    pub url_path: Option<syn::Ident>,
    pub initial_language_from_url_path: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_path_to_cookie: Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_path_to_local_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_path_to_session_storage:
        Vec<LitBoolExprOrIdent>,
    pub initial_language_from_url_path_to_server_function:
        Vec<LitBoolExprOrIdent>,
    #[cfg(feature = "system")]
    pub initial_language_from_system: Vec<LitBoolExprOrIdent>,
    #[cfg(feature = "system")]
    pub initial_language_from_system_to_data_file: Vec<LitBoolExprOrIdent>,
    #[cfg(feature = "system")]
    pub set_language_to_data_file: Vec<LitBoolExprOrIdent>,
    #[cfg(feature = "system")]
    pub initial_language_from_data_file: Vec<LitBoolExprOrIdent>,
    #[cfg(feature = "system")]
    pub data_file_key: LitStrExpr,
}

impl Parse for I18nLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        let manifest_path = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".into()),
        );

        let mut warnings = Vec::new();
        let mut children: Vec<LitBoolExprOrIdent> = Vec::new();
        let mut locales_path: Option<syn::LitStr> = None;
        let mut languages_path: Option<syn::LitStr> = None;
        let mut core_locales_path: Option<syn::LitStr> = None;
        let mut default_language: Option<syn::LitStr> = None;
        let mut translations: Option<Translations> = None;
        let mut check_translations: Option<LitBoolOrStr> = None;
        let mut fill_translations: Option<syn::LitStr> = None;
        let mut customise: Option<syn::ExprClosure> = None;
        let mut provide_meta_context: Vec<LitBool> = Vec::new();
        let mut sync_html_tag_lang: Vec<LitBoolExprOrIdent> = Vec::new();
        let mut sync_html_tag_dir: Vec<LitBoolExprOrIdent> = Vec::new();
        let mut url_param = LitStrExpr::new();
        let mut initial_language_from_url_param: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_url_param_to_local_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_url_param_to_session_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_url_param_to_cookie: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_url_param_to_server_function: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut set_language_to_url_param: Vec<LitBoolExprOrIdent> = Vec::new();
        let mut local_storage_key = LitStrExpr::new();
        let mut initial_language_from_local_storage: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_local_storage_to_cookie: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_local_storage_to_session_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_local_storage_to_server_function: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut set_language_to_local_storage: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut session_storage_key = LitStrExpr::new();
        let mut initial_language_from_session_storage: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_session_storage_to_cookie: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_session_storage_to_local_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_session_storage_to_server_function: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut set_language_to_session_storage: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_navigator: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_navigator_to_local_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_navigator_to_session_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_navigator_to_cookie: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_navigator_to_server_function: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_accept_language_header: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut set_language_from_navigator: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut cookie_name = LitStrExpr::new();
        let mut cookie_attrs = LitStrExpr::new();
        let mut initial_language_from_cookie: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_cookie_to_local_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_cookie_to_session_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_cookie_to_server_function: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut set_language_to_cookie: Vec<LitBoolExprOrIdent> = Vec::new();
        let mut initial_language_from_server_function: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_server_function_to_cookie: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_server_function_to_local_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut set_language_to_server_function: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut url_path: Option<syn::Ident> = None;
        let mut initial_language_from_url_path: Vec<LitBoolExprOrIdent> =
            Vec::new();
        let mut initial_language_from_url_path_to_cookie: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_url_path_to_local_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_url_path_to_session_storage: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut initial_language_from_url_path_to_server_function: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();

        #[cfg(feature = "system")]
        let mut initial_language_from_system: Vec<LitBoolExprOrIdent> =
            Vec::new();
        #[cfg(feature = "system")]
        let mut initial_language_from_system_to_data_file: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        #[cfg(feature = "system")]
        let mut set_language_to_data_file: Vec<LitBoolExprOrIdent> = Vec::new();
        #[cfg(feature = "system")]
        let mut initial_language_from_data_file: Vec<
            LitBoolExprOrIdent,
        > = Vec::new();
        let mut data_file_key = LitStrExpr::new();

        while !input.is_empty() {
            let mut exprpath: Option<String> = None;
            let mut exprpath_token_stream: Option<proc_macro2::TokenStream> =
                None;
            let k: syn::Ident;
            if input.peek(syn::Ident)
                && (input.peek2(syn::Token![:]) || input.peek2(syn::Token![,]))
            {
                k = input.parse::<syn::Ident>()?;
                // `expression:` or `expression,`
            } else {
                let maybe_expr = input.parse::<syn::Expr>();
                if maybe_expr.is_err() {
                    return Err(syn::Error::new(
                        input.span(),
                        format!(
                            concat!(
                                "Expected expression with",
                                " `key: value`, `#[...] key: value`, `key,` or `#[...] key,` format.",
                                " Found:{}"
                            ),
                            if input.to_string().is_empty() {
                                concat!(
                                    " (empty).\n",
                                    "If you're using double curly braces syntax",
                                    " (`leptos_fluent! {{ ... }}`) make sure to",
                                    " use single curly braces syntax",
                                    " (`leptos_fluent! { ... }`)."
                                ).to_string()
                            } else {
                                format!("\n{}", &input.to_string())
                            }
                        ),
                    ));
                }
                let expr = maybe_expr.unwrap();
                if matches!(expr, syn::Expr::Path(_)) {
                    let string = expr.to_token_stream().to_string();
                    let splitter =
                        if string.contains('\n') { "\n" } else { "] " };
                    let ident = &string
                        .split(splitter)
                        .last()
                        .unwrap()
                        .replace('\n', " ");
                    k = syn::Ident::new(ident, expr.span());

                    let new_expr_stream =
                        expr.to_token_stream().into_iter().collect::<Vec<_>>();
                    // except last element
                    let except_last = new_expr_stream
                        .iter()
                        .take(new_expr_stream.len() - 1)
                        .cloned();
                    let stream = proc_macro2::TokenStream::from_iter(
                        except_last.into_iter(),
                    );
                    exprpath = Some(stream.to_string());
                    exprpath_token_stream = Some(stream);
                } else {
                    return Err(syn::Error::new(
                        expr.span(),
                        format!(
                            concat!(
                                "The line must be in the format 'key: value' or",
                                " contain some configuration macro like",
                                " '#[cfg(feature = \"nightly\")] key: value'.\n\n",
                                " Found expression: {:?}",
                            ),
                            expr.to_token_stream(),
                        )
                    ));
                }
            }

            let mut struct_field_init_shorthand = false;
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
                struct_field_init_shorthand = true;
            } else {
                input.parse::<syn::Token![:]>()?;
            }

            let k_token_stream_str =
                TokenStreamStr::from(k.to_string().as_str());
            if k == "children" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    children
                );
                let expr = input.parse::<syn::Expr>()?;
                param.expr =
                    Some(expr.to_token_stream().to_string().as_str().into());
                children.push(param);
            } else if k == "translations" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                translations = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    translations
                );
            } else if k == "locales" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                locales_path = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    locales_path
                );
            } else if k == "core_locales" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                core_locales_path = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    core_locales_path
                );
            } else if k == "default_language" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                default_language = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    default_language
                );
            } else if k == "languages" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                languages_path = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    languages_path
                );
            } else if k == "check_translations" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                check_translations = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    check_translations
                );
            } else if k == "fill_translations" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                fill_translations = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    fill_translations
                );
            } else if k == "customise" {
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                customise = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath_token_stream,
                    k,
                    fill_translations
                );
            } else if k == "sync_html_tag_lang" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    sync_html_tag_lang
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "sync_html_tag_lang",
                )?;
                sync_html_tag_lang.push(param);
            } else if k == "sync_html_tag_dir" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    sync_html_tag_dir
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "sync_html_tag_dir",
                )?;
                sync_html_tag_dir.push(param);
            } else if k == "url_param" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    url_param,
                    k_token_stream_str
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath_token_stream,
                    k,
                    input,
                    url_param,
                    "url_param"
                );
            } else if k == "initial_language_from_url_param" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_param
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_param",
                )?;
                initial_language_from_url_param.push(param);
            } else if k == "initial_language_from_url_param_to_localstorage"
                || k == "initial_language_from_url_param_to_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_param_to_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_param_to_local_storage",
                )?;
                initial_language_from_url_param_to_local_storage.push(param);
            } else if k == "initial_language_from_url_param_to_sessionstorage"
                || k == "initial_language_from_url_param_to_session_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_param_to_session_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_param_to_session_storage",
                )?;
                initial_language_from_url_param_to_session_storage.push(param);
            } else if k == "initial_language_from_url_param_to_cookie" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_param_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_param_to_cookie",
                )?;
                initial_language_from_url_param_to_cookie.push(param);
            } else if k == "initial_language_from_url_param_to_server_function"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                initial_language_from_url_param_to_server_function.push(param);
            } else if k == "set_language_to_url_param" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    set_language_to_url_param
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_to_url_param",
                )?;
                set_language_to_url_param.push(param);
            } else if k == "localstorage_key" || k == "local_storage_key" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    local_storage_key,
                    k_token_stream_str
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath_token_stream,
                    k,
                    input,
                    local_storage_key,
                    "local_storage_key"
                );
            } else if k == "initial_language_from_localstorage"
                || k == "initial_language_from_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_local_storage",
                )?;
                initial_language_from_local_storage.push(param);
            } else if k == "initial_language_from_localstorage_to_cookie"
                || k == "initial_language_from_local_storage_to_cookie"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_local_storage_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_local_storage_to_cookie",
                )?;
                initial_language_from_local_storage_to_cookie.push(param);
            } else if k
                == "initial_language_from_localstorage_to_sessionstorage"
                || k == "initial_language_from_local_storage_to_session_storage"
                || k == "initial_language_from_local_storage_to_sessionstorage"
                || k == "initial_language_from_localstorage_to_session_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_local_storage_to_session_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_local_storage_to_session_storage",
                )?;
                initial_language_from_local_storage_to_session_storage
                    .push(param);
            } else if k
                == "initial_language_from_localstorage_to_server_function"
                || k == "initial_language_from_local_storage_to_server_function"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                initial_language_from_local_storage_to_server_function
                    .push(param);
            } else if k == "set_language_to_localstorage"
                || k == "set_language_to_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    set_language_to_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_to_local_storage",
                )?;
                set_language_to_local_storage.push(param);
            } else if k == "session_storage_key" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    session_storage_key,
                    k_token_stream_str
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath_token_stream,
                    k,
                    input,
                    session_storage_key,
                    "session_storage_key"
                );
            } else if k == "initial_language_from_sessionstorage"
                || k == "initial_language_from_session_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_session_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_session_storage",
                )?;
                initial_language_from_session_storage.push(param);
            } else if k == "initial_language_from_sessionstorage_to_cookie"
                || k == "initial_language_from_session_storage_to_cookie"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_session_storage_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_session_storage_to_cookie",
                )?;
                initial_language_from_session_storage_to_cookie.push(param);
            } else if k
                == "initial_language_from_session_storage_to_localstorage"
                || k == "initial_language_from_session_storage_to_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_session_storage_to_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_session_storage_to_local_storage",
                )?;
                initial_language_from_session_storage_to_local_storage
                    .push(param);
            } else if k
                == "initial_language_from_session_storage_to_server_function"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                initial_language_from_session_storage_to_server_function
                    .push(param);
            } else if k == "set_language_to_sessionstorage"
                || k == "set_language_to_session_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    set_language_to_session_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_to_session_storage",
                )?;
                set_language_to_session_storage.push(param);
            } else if k == "initial_language_from_navigator" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_navigator
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_navigator",
                )?;
                initial_language_from_navigator.push(param);
            } else if k == "initial_language_from_navigator_to_localstorage"
                || k == "initial_language_from_navigator_to_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_navigator_to_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_navigator_to_local_storage",
                )?;
                initial_language_from_navigator_to_local_storage.push(param);
            } else if k == "initial_language_from_navigator_to_sessionstorage"
                || k == "initial_language_from_navigator_to_session_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_navigator_to_session_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_navigator_to_session_storage",
                )?;
                initial_language_from_navigator_to_session_storage.push(param);
            } else if k == "initial_language_from_navigator_to_cookie" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_navigator_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_navigator_to_cookie",
                )?;
                initial_language_from_navigator_to_cookie.push(param);
            } else if k == "initial_language_from_navigator_to_server_function"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                initial_language_from_navigator_to_server_function.push(param);
            } else if k == "initial_language_from_accept_language_header" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_accept_language_header
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_accept_language_header",
                )?;
                initial_language_from_accept_language_header.push(param);
            } else if k == "set_language_from_navigator" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    set_language_from_navigator
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_from_navigator",
                )?;
                set_language_from_navigator.push(param);
            } else if k == "cookie_name" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    cookie_name,
                    k_token_stream_str
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath_token_stream,
                    k,
                    input,
                    cookie_name,
                    "cookie_name"
                );
            } else if k == "cookie_attrs" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    cookie_attrs,
                    k_token_stream_str
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath_token_stream,
                    k,
                    input,
                    cookie_attrs,
                    "cookie_attrs"
                );
            } else if k == "initial_language_from_cookie" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_cookie",
                )?;
                initial_language_from_cookie.push(param);
            } else if k == "initial_language_from_cookie_to_localstorage"
                || k == "initial_language_from_cookie_to_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_cookie_to_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_cookie_to_local_storage",
                )?;
                initial_language_from_cookie_to_local_storage.push(param);
            } else if k == "initial_language_from_cookie_to_sessionstorage"
                || k == "initial_language_from_cookie_to_session_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_cookie_to_session_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_cookie_to_session_storage",
                )?;
                initial_language_from_cookie_to_session_storage.push(param);
            } else if k == "initial_language_from_cookie_to_server_function" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                initial_language_from_cookie_to_server_function.push(param);
            } else if k == "set_language_to_cookie" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    set_language_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_to_cookie",
                )?;
                set_language_to_cookie.push(param);
            } else if k == "initial_language_from_server_function" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                initial_language_from_server_function.push(param);
            } else if k == "initial_language_from_server_function_to_cookie" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_server_function_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_server_function_to_cookie",
                )?;
                initial_language_from_server_function_to_cookie.push(param);
            } else if k
                == "initial_language_from_server_function_to_localstorage"
                || k == "initial_language_from_server_function_to_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_server_function_to_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_server_function_to_local_storage",
                )?;
                initial_language_from_server_function_to_local_storage
                    .push(param);
            } else if k == "set_language_to_server_function" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                set_language_to_server_function.push(param);
            } else if k == "url_path" {
                if let Some(ref e) = exprpath_token_stream {
                    let evaluated_exprpath = crate::evaluate_exprpath(e);
                    if !evaluated_exprpath.supported {
                        return Err(syn::Error::new(
                            e.span(),
                            exprpath_not_supported_error_message(
                                e.to_string().as_str(),
                                &k,
                            ),
                        ));
                    } else if evaluated_exprpath.result {
                        if struct_field_init_shorthand {
                            url_path = Some(k.clone());
                        } else {
                            url_path = Some(input.parse()?);
                        }
                    }
                } else if struct_field_init_shorthand {
                    url_path = Some(k.clone());
                } else {
                    url_path = Some(input.parse()?);
                }
            } else if k == "initial_language_from_url_path" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_path
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_path",
                )?;
                initial_language_from_url_path.push(param);
            } else if k == "initial_language_from_url_path_to_cookie" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_path_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_path_to_cookie",
                )?;
                initial_language_from_url_path_to_cookie.push(param);
            } else if k == "initial_language_from_url_path_to_localstorage"
                || k == "initial_language_from_url_path_to_local_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_path_to_local_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_path_to_local_storage",
                )?;
                initial_language_from_url_path_to_local_storage.push(param);
            } else if k == "initial_language_from_url_path_to_sessionstorage"
                || k == "initial_language_from_url_path_to_session_storage"
            {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k_token_stream_str,
                    initial_language_from_url_path_to_session_storage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_path_to_session_storage",
                )?;
                initial_language_from_url_path_to_session_storage.push(param);
            } else if k == "initial_language_from_url_path_to_server_function" {
                let mut param = LitBoolExprOrIdent::new();
                parse_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.expr = Some(k_token_stream_str);
                } else {
                    let expr = input.parse::<syn::Expr>()?;
                    param.expr = Some(
                        expr.to_token_stream().to_string().as_str().into(),
                    );
                }
                initial_language_from_url_path_to_server_function.push(param);
            } else if k == "initial_language_from_system" {
                #[cfg(feature = "system")]
                {
                    let mut param = LitBoolExprOrIdent::new();
                    parse_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k_token_stream_str,
                        initial_language_from_system
                    );
                    parse_litbool_or_expr_param(
                        input,
                        &mut param.expr,
                        "initial_language_from_system",
                    )?;
                    initial_language_from_system.push(param);
                }

                #[cfg(not(feature = "system"))]
                {
                    return Err(syn::Error::new(
                        k.span(),
                        concat!(
                            "The parameter 'initial_language_from_system' of",
                            " leptos_fluent! macro requires the feature",
                            " 'system' enabled.",
                        ),
                    ));
                }
            } else if k == "initial_language_from_data_file" {
                #[cfg(feature = "system")]
                {
                    let mut param = LitBoolExprOrIdent::new();
                    parse_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k_token_stream_str,
                        initial_language_from_data_file
                    );
                    parse_litbool_or_expr_param(
                        input,
                        &mut param.expr,
                        "initial_language_from_data_file",
                    )?;
                    initial_language_from_data_file.push(param);
                }

                #[cfg(not(feature = "system"))]
                {
                    return Err(syn::Error::new(
                        k.span(),
                        concat!(
                            "The parameter 'initial_language_from_data_file' of",
                            " leptos_fluent! macro requires the feature",
                            " 'system' enabled.",
                        ),
                    ));
                }
            } else if k == "initial_language_from_system_to_data_file" {
                #[cfg(feature = "system")]
                {
                    let mut param = LitBoolExprOrIdent::new();
                    parse_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k_token_stream_str,
                        initial_language_from_system_to_data_file
                    );
                    parse_litbool_or_expr_param(
                        input,
                        &mut param.expr,
                        "initial_language_from_system_to_data_file",
                    )?;
                    initial_language_from_system_to_data_file.push(param);
                }

                #[cfg(not(feature = "system"))]
                {
                    return Err(syn::Error::new(
                        k.span(),
                        concat!(
                            "The parameter 'initial_language_from_system_to_data_file' of",
                            " leptos_fluent! macro requires the feature",
                            " 'system' enabled.",
                        ),
                    ));
                }
            } else if k == "set_language_to_data_file" {
                #[cfg(feature = "system")]
                {
                    let mut param = LitBoolExprOrIdent::new();
                    parse_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k_token_stream_str,
                        set_language_to_data_file
                    );
                    parse_litbool_or_expr_param(
                        input,
                        &mut param.expr,
                        "set_language_to_data_file",
                    )?;
                    set_language_to_data_file.push(param);
                }

                #[cfg(not(feature = "system"))]
                {
                    return Err(syn::Error::new(
                        k.span(),
                        concat!(
                            "The parameter 'set_language_to_data_file' of",
                            " leptos_fluent! macro requires the feature",
                            " 'system' enabled.",
                        ),
                    ));
                }
            } else if k == "data_file_key" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    data_file_key,
                    k_token_stream_str
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath_token_stream,
                    k,
                    input,
                    data_file_key,
                    "data_file_key"
                );
            } else if k == "provide_meta_context" {
                let mut param = LitBool::new();
                parse_runtime_exprpath!(exprpath, param);
                check_struct_field_init_shorthand(
                    struct_field_init_shorthand,
                    &k,
                )?;
                param.lit = Some(input.parse::<syn::LitBool>()?.value());
                provide_meta_context.push(param);
            } else {
                return Err(syn::Error::new(
                    k.span(),
                    format!(
                        "Invalid parameter '{k}' for leptos_fluent! macro."
                    ),
                ));
            }

            // Warnings for deprecated parameters that contain
            // "session_storage" or "local_storage" in their name.
            // TODO: remove in v0.3.0
            let k_str = k.to_string();
            if k_str.contains("sessionstorage")
                || k_str.contains("localstorage")
            {
                let new_k_str = k_str
                    .replace("sessionstorage", "session_storage")
                    .replace("localstorage", "local_storage");
                let warning = proc_macro_warning::Warning::new_deprecated(
                    k_str.to_string(),
                )
                .old("..")
                .new(format!("rename it as `{new_k_str}`"))
                .span(k.span())
                .build_or_panic();
                warnings.push(warning);
            }

            if input.is_empty() {
                break;
            }
            input.parse::<token::Comma>()?;
        }

        // languages
        if locales_path.is_none() {
            return Err(syn::Error::new(
                input.span(),
                "Missing `locales` parameter",
            ));
        }

        let languages;
        let mut languages_file_path = None;

        let languages_file = languages_path
            .as_ref()
            .map(|langs| manifest_path.join(langs.value()));

        let locales_folder_path = locales_path
            .as_ref()
            .map(|locales| manifest_path.join(locales.value()))
            .unwrap();

        if let Some(ref file) = languages_file {
            if std::fs::metadata(file).is_err() {
                let file_path =
                    std::path::absolute(file).unwrap_or(file.clone());

                return Err(syn::Error::new(
                    languages_path.as_ref().unwrap().span(),
                    format!(
                        concat!(
                            "Couldn't read languages file, this path should",
                            " be relative to your crate's `Cargo.toml`.",
                            " Looking for: {:?}",
                        ),
                        file_path,
                    ),
                ));
            } else {
                let langs_path = &languages_file.unwrap();
                let maybe_languages = read_languages_file(langs_path);
                if let Err(e) = maybe_languages {
                    return Err(syn::Error::new(
                        languages_path.as_ref().unwrap().span(),
                        e.to_string(),
                    ));
                }
                languages = maybe_languages.unwrap();
                languages_file_path =
                    Some(langs_path.as_path().to_str().unwrap().to_string());
            }
        } else if std::fs::metadata(&locales_folder_path).is_err() {
            let file_path = std::path::absolute(&locales_folder_path)
                .unwrap_or(locales_folder_path);

            return Err(syn::Error::new(
                locales_path.as_ref().unwrap().span(),
                format!(
                    concat!(
                        "Couldn't read locales folder. This path should",
                        " be relative to your crate's `Cargo.toml`.",
                        " Looking for: {:?}",
                    ),
                    file_path,
                ),
            ));
        } else {
            let (langs, read_locales_folder_errors) =
                read_locales_folder(&locales_folder_path);
            if !read_locales_folder_errors.is_empty() {
                return Err(syn::Error::new(
                    locales_path.as_ref().unwrap().span(),
                    format!(
                        "Errors while reading locales from {}:\n- {}",
                        locales_path.as_ref().unwrap().value(),
                        read_locales_folder_errors.join("\n- "),
                    ),
                ));
            }
            languages = langs;
        }

        let locales_path_str =
            locales_folder_path.as_path().to_str().unwrap().to_string();

        // core_locales
        let mut core_locales_content = None;
        let mut core_locales_path_str = None;
        if let Some(core_locales) = &core_locales_path {
            let core_locales = manifest_path.join(core_locales.value());
            if std::fs::metadata(&core_locales).is_err() {
                let file_path =
                    std::path::absolute(&core_locales).unwrap_or(core_locales);

                return Err(syn::Error::new(
                    core_locales_path.unwrap().span(),
                    format!(
                        concat!(
                            "Couldn't read core fluent resource. This path should",
                            " be relative to your crate's `Cargo.toml`.",
                            " Looking for: {:?}",
                        ),
                        file_path,
                    )
                ));
            } else {
                {
                    core_locales_content =
                        Some(std::fs::read_to_string(&core_locales).unwrap());
                }

                core_locales_path_str =
                    Some(core_locales.to_str().unwrap().to_string());
            }
        }

        let (fluent_resources_and_file_paths, resources_file_paths_errors) =
            build_fluent_resources_and_file_paths(&locales_path_str);
        if !resources_file_paths_errors.is_empty() {
            return Err(syn::Error::new(
                locales_path.unwrap().span(),
                format!(
                    "Errors while reading fluent resources from {}:\n- {}",
                    locales_path_str,
                    resources_file_paths_errors.join("\n- "),
                ),
            ));
        }

        if check_translations.is_some() || fill_translations.is_some() {
            let f_resources_and_file_paths =
                fluent_resources_and_file_paths.clone();
            let (ref fluent_resources, ref fluent_file_paths) =
                f_resources_and_file_paths;

            let mut errors: Vec<String> = Vec::new();
            let mut fluent_entries = build_fluent_entries(
                fluent_resources,
                fluent_file_paths,
                &manifest_path,
                &core_locales_path_str,
                &core_locales_content,
                &mut errors,
            );

            if !errors.is_empty() {
                let message = &format!(
                    "Unrecoverable errors:\n- {}",
                    errors.join("\n- "),
                );
                return Err(syn::Error::new(
                    if let Some(ref check_translations) = check_translations {
                        check_translations.span()
                    } else {
                        fill_translations.as_ref().unwrap().span()
                    },
                    message,
                ));
            }

            if let Some(ref fill_translations_globstr) = fill_translations {
                let fill_messages = crate::translations_filler::run(
                    &fill_translations_globstr.value(),
                    &manifest_path,
                    &fluent_entries,
                    fluent_file_paths,
                    fluent_resources,
                    &mut errors,
                );

                let mut report = String::new();
                if !fill_messages.is_empty() {
                    report.push_str("Translations filled by leptos-fluent:\n");
                    for (file_path, message_names) in fill_messages {
                        report.push_str(&format!("  {file_path}\n",));
                        for message_name in message_names {
                            report
                                .push_str(&format!("    - {message_name}\n",));
                        }
                    }
                }
                if !report.is_empty() {
                    report.push('\n');
                    eprintln!("{report}");

                    // resources must be recreated because new fluent entries
                    // have been added to them
                    let (
                        f_resources_and_file_paths_,
                        resources_file_paths_errors,
                    ) = build_fluent_resources_and_file_paths(
                        &locales_path_str,
                    );
                    if !resources_file_paths_errors.is_empty() {
                        return Err(syn::Error::new(
                            locales_path.unwrap().span(),
                            format!(
                                "Errors while reading fluent resources from {}:\n- {}",
                                locales_path_str,
                                resources_file_paths_errors.join("\n- "),
                            ),
                        ));
                    }

                    let (ref fluent_resources, ref fluent_file_paths) =
                        f_resources_and_file_paths_;

                    fluent_entries = build_fluent_entries(
                        fluent_resources,
                        fluent_file_paths,
                        &manifest_path,
                        &core_locales_path_str,
                        &core_locales_content,
                        &mut errors,
                    );
                }

                if !errors.is_empty() {
                    let message = &format!(
                        "Unrecoverable errors:\n- {}",
                        errors.join("\n- "),
                    );
                    return Err(syn::Error::new(
                        fill_translations_globstr.span(),
                        message,
                    ));
                }
            }

            if let Some(ref check_translations_enum) = check_translations {
                let maybe_tr_macros = match check_translations_enum {
                    LitBoolOrStr::Str(litstr) => {
                        gather_tr_macro_defs_from_globstr(
                            manifest_path.join(litstr.value()),
                            &mut errors,
                            #[cfg(not(test))]
                            &manifest_path,
                        )
                    }
                    LitBoolOrStr::Bool(litbool) => {
                        if litbool.value() {
                            gather_tr_macro_defs_from_workspace(
                                &manifest_path,
                                &mut errors,
                            )
                        } else {
                            Ok(Vec::new())
                        }
                    }
                };
                if let Ok(tr_macros) = maybe_tr_macros {
                    let check_messages = crate::translations_checker::run(
                        &fluent_entries,
                        &tr_macros,
                    );

                    let mut report = String::new();
                    if !check_messages.is_empty() {
                        report.push_str(&format!(
                            "Translations check failed:\n- {}",
                            check_messages.join("\n- "),
                        ));
                        if !errors.is_empty() {
                            report.push_str("\n\n");
                        }
                    }
                    if !errors.is_empty() {
                        report.push_str(&format!(
                            "Unrecoverable errors:\n- {}",
                            errors.join("\n- "),
                        ));
                    }
                    if !report.is_empty() {
                        return Err(syn::Error::new(
                            check_translations_enum.span(),
                            report,
                        ));
                    }
                }
            }
        }

        if let Some(ref attrs) = cookie_attrs.lit {
            let errors = validate_cookie_attrs(&attrs.value());
            if !errors.is_empty() {
                return Err(syn::Error::new(
                    cookie_attrs.lit.unwrap().span(),
                    format!(
                        "Invalid cookie attributes:\n- {}",
                        errors.join("\n- "),
                    ),
                ));
            }
        }

        let default_language_and_index = if let Some(ref default_language) =
            default_language
        {
            let value = &default_language.value();
            let languages_clone = languages.clone();
            let maybe_language_and_index = languages_clone
                .iter()
                .enumerate()
                .find(|(_, (code, ..))| code == value);
            if maybe_language_and_index.is_none() {
                let lang_codes = languages_clone
                    .into_iter()
                    .map(|(k, ..)| k)
                    .collect::<Vec<_>>();
                return Err(syn::Error::new(
                    default_language.span(),
                    format!(
                        "Default language '{}' not found in defined languages: {:?}",
                        value,
                        &lang_codes,
                    ),
                ));
            }
            let (index, (code, ..)) = maybe_language_and_index.unwrap();
            Some((code.to_string(), index))
        } else {
            None
        };

        let loader_ = Self {
            warnings,
            fluent_file_paths: fluent_resources_and_file_paths.1,
            children,
            translations,
            languages,
            languages_path: languages_file_path,
            raw_languages_path: languages_path.map(|x| x.value()),
            locales_path: locales_path.unwrap().value(),
            core_locales_path: core_locales_path_str,
            default_language: default_language_and_index,
            check_translations,
            fill_translations: fill_translations.map(|x| x.value()),
            customise,
            provide_meta_context,
            sync_html_tag_lang,
            sync_html_tag_dir,
            url_param,
            initial_language_from_url_param,
            initial_language_from_url_param_to_local_storage,
            initial_language_from_url_param_to_session_storage,
            initial_language_from_url_param_to_cookie,
            initial_language_from_url_param_to_server_function,
            set_language_to_url_param,
            local_storage_key,
            initial_language_from_local_storage,
            initial_language_from_local_storage_to_cookie,
            initial_language_from_local_storage_to_session_storage,
            initial_language_from_local_storage_to_server_function,
            set_language_to_local_storage,
            session_storage_key,
            initial_language_from_session_storage,
            initial_language_from_session_storage_to_cookie,
            initial_language_from_session_storage_to_local_storage,
            initial_language_from_session_storage_to_server_function,
            set_language_to_session_storage,
            initial_language_from_navigator,
            initial_language_from_navigator_to_local_storage,
            initial_language_from_navigator_to_session_storage,
            initial_language_from_navigator_to_cookie,
            initial_language_from_navigator_to_server_function,
            initial_language_from_accept_language_header,
            set_language_from_navigator,
            cookie_name,
            cookie_attrs,
            initial_language_from_cookie,
            initial_language_from_cookie_to_local_storage,
            initial_language_from_cookie_to_session_storage,
            initial_language_from_cookie_to_server_function,
            set_language_to_cookie,
            initial_language_from_server_function,
            initial_language_from_server_function_to_cookie,
            initial_language_from_server_function_to_local_storage,
            set_language_to_server_function,
            url_path,
            initial_language_from_url_path,
            initial_language_from_url_path_to_cookie,
            initial_language_from_url_path_to_local_storage,
            initial_language_from_url_path_to_session_storage,
            initial_language_from_url_path_to_server_function,
            #[cfg(feature = "system")]
            initial_language_from_system,
            #[cfg(feature = "system")]
            initial_language_from_system_to_data_file,
            #[cfg(feature = "system")]
            set_language_to_data_file,
            #[cfg(feature = "system")]
            initial_language_from_data_file,
            #[cfg(feature = "system")]
            data_file_key,
        };

        Ok(loader_)
    }
}
