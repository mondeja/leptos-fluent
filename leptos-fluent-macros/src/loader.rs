use crate::{
    build_fluent_resources_and_file_paths,
    cookie::validate_cookie_attrs,
    languages::{read_languages_file, read_locales_folder},
    FluentFilePaths, ParsedLanguage,
};
use quote::ToTokens;
use std::path::PathBuf;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token, Result,
};

fn parse_litstr_or_expr_param(
    input: ParseStream,
    strlit: &mut Option<syn::LitStr>,
    expr: &mut Option<syn::Expr>,
    param_name: &'static str,
) -> Result<()> {
    match input.parse::<syn::LitStr>() {
        Ok(lit) => {
            *strlit = Some(lit);
            Ok(())
        }
        Err(_) => match input.parse::<syn::Expr>() {
            Ok(e) => {
                *expr = Some(e);
                Ok(())
            }
            Err(_) => {
                let input_str = input.to_string();
                Err(syn::Error::new(
                    input.span(),
                    format!(
                        concat!(
                            "Not a valid value for '{}' of leptos_fluent! macro.",
                            " Must be a literal string or a valid expression.",
                            " Found {}",
                        ),
                        param_name,
                        match input_str.is_empty() {
                            true => "(empty)",
                            false => &input_str,
                        },
                    ),
                ))
            }
        },
    }
}

fn parse_litstr_or_expr_param_noop(
    input: ParseStream,
    param_name: &'static str,
) -> Result<()> {
    match input.parse::<syn::LitStr>() {
        Ok(_) => Ok(()),
        Err(_) => match input.parse::<syn::Expr>() {
            Ok(_) => Ok(()),
            Err(_) => {
                let input_str = input.to_string();
                Err(syn::Error::new(
                    input.span(),
                    format!(
                        concat!(
                            "Not a valid value for '{}' of leptos_fluent! macro.",
                            " Must be a literal string or a valid expression.",
                            " Found {}",
                        ),
                        param_name,
                        match input_str.is_empty() {
                            true => "(empty)",
                            false => &input_str,
                        },
                    ),
                ))
            }
        },
    }
}

macro_rules! parse_litstr_or_expr_param_with_maybe_comptime_exprpath {
    ($exprpath:ident, $k:ident, $input:ident, $param:ident, $param_name:literal) => {
        if let Some(ref e) = $exprpath {
            let evaluated_exprpath = $crate::evaluate_exprpath(e);
            if !evaluated_exprpath.supported {
                return Err(syn::Error::new(
                    e.span(),
                    exprpath_not_supported_error_message(e, &$k),
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
    expr: &mut Option<syn::Expr>,
    param_name: &'static str,
) -> Result<()> {
    match input.parse::<syn::LitBool>() {
        Ok(lit) => {
            *expr = Some(syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: syn::Lit::Bool(lit.clone()),
            }));
            Ok(())
        }
        Err(_) => match input.parse::<syn::Expr>() {
            Ok(e) => {
                *expr = Some(e);
                Ok(())
            }
            Err(_) => {
                let input_str = input.to_string();
                Err(syn::Error::new(
                    input.span(),
                    format!(
                        concat!(
                            "Not a valid value for '{}' of leptos_fluent! macro.",
                            " Must be a literal string or a valid expression.",
                            " Found {}",
                        ),
                        param_name,
                        match input_str.is_empty() {
                            true => "(empty)",
                            false => &input_str,
                        },
                    ),
                ))
            }
        },
    }
}

macro_rules! parse_struct_field_init_shorthand {
    ($shorthand:ident, $param:ident, $k:ident) => {
        if $shorthand {
            $param.expr =
                Some(syn::Expr::Verbatim($k.to_string().parse().unwrap()));
            continue;
        }
    };
    ($shorthand:ident, $param:ident, $k:ident, $vec:ident) => {
        if $shorthand {
            $param.expr =
                Some(syn::Expr::Verbatim($k.to_string().parse().unwrap()));
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
pub(crate) struct Translations {
    pub(crate) simple: Vec<syn::Path>,
    pub(crate) compound: Vec<syn::Path>,
}

impl Parse for Translations {
    fn parse(input: ParseStream) -> Result<Self> {
        // example of input
        // [loader1, loader2] + loaders1 + loaders2 + [loader3]
        let mut simple = Vec::new();
        let mut compound = Vec::new();

        let loaders = syn::punctuated::Punctuated::<
            SimpleOrCompound,
            syn::Token![+],
        >::parse_separated_nonempty(input)?;
        for loader in loaders.into_iter() {
            match loader {
                SimpleOrCompound::Simple(x) => {
                    for loader in x.0.into_iter() {
                        simple.push(loader);
                    }
                }
                SimpleOrCompound::Compound(compound_loader) => {
                    compound.push(compound_loader.0);
                }
            }
        }

        Ok(Self { simple, compound })
    }
}

fn exprpath_not_supported_error_message(
    expr: &proc_macro2::TokenStream,
    k: &syn::Ident,
) -> String {
    let exprpath_str = expr.to_string();
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
        k, exprpath_str, exprpath_str, k, k, k,
    )
}

macro_rules! evaluate_compile_time_exprpath_set_none {
    ($exprpath:ident, $k:ident, $field:ident) => {
        if let Some(ref e) = $exprpath {
            let evaluated_exprpath = $crate::evaluate_exprpath(e);
            if !evaluated_exprpath.supported {
                return Err(syn::Error::new(
                    e.span(),
                    exprpath_not_supported_error_message(e, &$k),
                ));
            } else if !evaluated_exprpath.result {
                $field = None;
            }
        }
    };
}

macro_rules! clone_runtime_exprpath {
    ($exprpath:ident, $field:ident) => {
        $field.exprpath.clone_from(&$exprpath);
    };
}

macro_rules! struct_field_init_shorthand_not_supported {
    ($struct_field_init_shorthand:ident, $k:ident) => {
        if $struct_field_init_shorthand {
            return Err(syn::Error::new(
                $k.span(),
                format!(
                    concat!(
                        "Struct field initialization shorthand is not supported",
                        " for the parameter '{}'.",
                    ),
                    $k,
                )
            ));
        }
    };
}

#[derive(Default)]
pub(crate) struct LitBoolExpr {
    pub expr: Option<syn::Expr>,
    pub exprpath: Option<proc_macro2::TokenStream>,
}

impl LitBoolExpr {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub(crate) struct LitStrExpr {
    pub lit: Option<syn::LitStr>,
    pub expr: Option<syn::Expr>,
}

impl LitStrExpr {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub(crate) struct Identifier {
    pub ident: Option<syn::Ident>,
    pub exprpath: Option<proc_macro2::TokenStream>,
}

impl Identifier {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub(crate) struct LitBool {
    pub lit: Option<syn::LitBool>,
    pub exprpath: Option<proc_macro2::TokenStream>,
}

impl LitBool {
    pub fn new() -> Self {
        Self::default()
    }
}

pub(crate) struct I18nLoader {
    pub fluent_file_paths: FluentFilePaths,
    pub translations: Translations,
    pub languages: Vec<ParsedLanguage>,
    pub languages_path: Option<String>,
    pub raw_languages_path: Option<String>,
    pub locales_path: String,
    pub core_locales_path: Option<String>,
    pub check_translations: Option<String>,
    pub provide_meta_context: Vec<LitBool>,
    pub sync_html_tag_lang: Vec<LitBoolExpr>,
    pub sync_html_tag_dir: Vec<LitBoolExpr>,
    pub url_param: LitStrExpr,
    pub initial_language_from_url_param: Vec<LitBoolExpr>,
    pub initial_language_from_url_param_to_localstorage: Vec<LitBoolExpr>,
    pub initial_language_from_url_param_to_cookie: Vec<LitBoolExpr>,
    pub initial_language_from_url_param_to_server_function: Vec<Identifier>,
    pub set_language_to_url_param: Vec<LitBoolExpr>,
    pub localstorage_key: LitStrExpr,
    pub initial_language_from_localstorage: Vec<LitBoolExpr>,
    pub initial_language_from_localstorage_to_cookie: Vec<LitBoolExpr>,
    pub initial_language_from_localstorage_to_server_function: Vec<Identifier>,
    pub set_language_to_localstorage: Vec<LitBoolExpr>,
    pub initial_language_from_navigator: Vec<LitBoolExpr>,
    pub initial_language_from_navigator_to_localstorage: Vec<LitBoolExpr>,
    pub initial_language_from_navigator_to_cookie: Vec<LitBoolExpr>,
    pub initial_language_from_navigator_to_server_function: Vec<Identifier>,
    pub initial_language_from_accept_language_header: Vec<LitBoolExpr>,
    pub set_language_from_navigator: Vec<LitBoolExpr>,
    pub cookie_name: LitStrExpr,
    pub cookie_attrs: LitStrExpr,
    pub initial_language_from_cookie: Vec<LitBoolExpr>,
    pub initial_language_from_cookie_to_localstorage: Vec<LitBoolExpr>,
    pub initial_language_from_cookie_to_server_function: Vec<Identifier>,
    pub set_language_to_cookie: Vec<LitBoolExpr>,
    pub initial_language_from_server_function: Vec<Identifier>,
    pub initial_language_from_server_function_to_cookie: Vec<LitBoolExpr>,
    pub initial_language_from_server_function_to_localstorage: Vec<LitBoolExpr>,
    pub set_language_to_server_function: Vec<Identifier>,
    pub url_path: Option<syn::Ident>,
    pub initial_language_from_url_path: Vec<LitBoolExpr>,
    pub initial_language_from_url_path_to_cookie: Vec<LitBoolExpr>,
    pub initial_language_from_url_path_to_localstorage: Vec<LitBoolExpr>,
    pub initial_language_from_url_path_to_server_function: Vec<Identifier>,
    #[cfg(feature = "system")]
    pub initial_language_from_system: Vec<LitBoolExpr>,
    #[cfg(feature = "system")]
    pub initial_language_from_system_to_data_file: Vec<LitBoolExpr>,
    #[cfg(feature = "system")]
    pub set_language_to_data_file: Vec<LitBoolExpr>,
    #[cfg(feature = "system")]
    pub initial_language_from_data_file: Vec<LitBoolExpr>,
    #[cfg(feature = "system")]
    pub data_file_key: LitStrExpr,
}

impl Parse for I18nLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        let workspace_path = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".into()),
        );

        let mut locales_path: Option<syn::LitStr> = None;
        let mut languages_path: Option<syn::LitStr> = None;
        let mut core_locales_path: Option<syn::LitStr> = None;
        let mut translations: Option<Translations> = None;
        let mut check_translations: Option<syn::LitStr> = None;
        let mut provide_meta_context: Vec<LitBool> = Vec::new();
        let mut sync_html_tag_lang: Vec<LitBoolExpr> = Vec::new();
        let mut sync_html_tag_dir: Vec<LitBoolExpr> = Vec::new();
        let mut url_param = LitStrExpr::new();
        let mut initial_language_from_url_param: Vec<LitBoolExpr> = Vec::new();
        let mut initial_language_from_url_param_to_localstorage: Vec<
            LitBoolExpr,
        > = Vec::new();
        let mut initial_language_from_url_param_to_cookie: Vec<LitBoolExpr> =
            Vec::new();
        let mut initial_language_from_url_param_to_server_function: Vec<
            Identifier,
        > = Vec::new();
        let mut set_language_to_url_param: Vec<LitBoolExpr> = Vec::new();
        let mut localstorage_key = LitStrExpr::new();
        let mut initial_language_from_localstorage: Vec<LitBoolExpr> =
            Vec::new();
        let mut initial_language_from_localstorage_to_cookie: Vec<LitBoolExpr> =
            Vec::new();
        let mut initial_language_from_localstorage_to_server_function: Vec<
            Identifier,
        > = Vec::new();
        let mut set_language_to_localstorage: Vec<LitBoolExpr> = Vec::new();
        let mut initial_language_from_navigator: Vec<LitBoolExpr> = Vec::new();
        let mut initial_language_from_navigator_to_localstorage: Vec<
            LitBoolExpr,
        > = Vec::new();
        let mut initial_language_from_navigator_to_cookie: Vec<LitBoolExpr> =
            Vec::new();
        let mut initial_language_from_navigator_to_server_function: Vec<
            Identifier,
        > = Vec::new();
        let mut initial_language_from_accept_language_header: Vec<LitBoolExpr> =
            Vec::new();
        let mut set_language_from_navigator: Vec<LitBoolExpr> = Vec::new();
        let mut cookie_name = LitStrExpr::new();
        let mut cookie_attrs = LitStrExpr::new();
        let mut initial_language_from_cookie: Vec<LitBoolExpr> = Vec::new();
        let mut initial_language_from_cookie_to_localstorage: Vec<LitBoolExpr> =
            Vec::new();
        let mut initial_language_from_cookie_to_server_function: Vec<
            Identifier,
        > = Vec::new();
        let mut set_language_to_cookie: Vec<LitBoolExpr> = Vec::new();
        let mut initial_language_from_server_function: Vec<Identifier> =
            Vec::new();
        let mut initial_language_from_server_function_to_cookie: Vec<
            LitBoolExpr,
        > = Vec::new();
        let mut initial_language_from_server_function_to_localstorage: Vec<
            LitBoolExpr,
        > = Vec::new();
        let mut set_language_to_server_function: Vec<Identifier> = Vec::new();
        let mut url_path: Option<syn::Ident> = None;
        let mut initial_language_from_url_path: Vec<LitBoolExpr> = Vec::new();
        let mut initial_language_from_url_path_to_cookie: Vec<LitBoolExpr> =
            Vec::new();
        let mut initial_language_from_url_path_to_localstorage: Vec<
            LitBoolExpr,
        > = Vec::new();
        let mut initial_language_from_url_path_to_server_function: Vec<
            Identifier,
        > = Vec::new();

        #[cfg(feature = "system")]
        let mut initial_language_from_system: Vec<LitBoolExpr> = Vec::new();
        #[cfg(feature = "system")]
        let mut initial_language_from_system_to_data_file: Vec<
            LitBoolExpr,
        > = Vec::new();
        #[cfg(feature = "system")]
        let mut set_language_to_data_file: Vec<LitBoolExpr> = Vec::new();
        #[cfg(feature = "system")]
        let mut initial_language_from_data_file: Vec<LitBoolExpr> = Vec::new();
        let mut data_file_key = LitStrExpr::new();

        while !input.is_empty() {
            let mut exprpath: Option<proc_macro2::TokenStream> = None;
            let k;
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
                                "Expected an expression with",
                                " 'key: value', '#[...] key: value', 'key,' or `#[...] key,` format.",
                                " Found:\n{}"
                            ),
                            input,
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
                    exprpath = Some(proc_macro2::TokenStream::from_iter(
                        except_last.into_iter(),
                    ));
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

            if k == "translations" {
                struct_field_init_shorthand_not_supported!(
                    struct_field_init_shorthand,
                    k
                );
                translations = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath,
                    k,
                    translations
                );
            } else if k == "locales" {
                struct_field_init_shorthand_not_supported!(
                    struct_field_init_shorthand,
                    k
                );
                locales_path = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath,
                    k,
                    locales_path
                );
            } else if k == "core_locales" {
                struct_field_init_shorthand_not_supported!(
                    struct_field_init_shorthand,
                    k
                );
                core_locales_path = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath,
                    k,
                    core_locales_path
                );
            } else if k == "languages" {
                struct_field_init_shorthand_not_supported!(
                    struct_field_init_shorthand,
                    k
                );
                languages_path = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath,
                    k,
                    languages_path
                );
            } else if k == "check_translations" {
                struct_field_init_shorthand_not_supported!(
                    struct_field_init_shorthand,
                    k
                );
                check_translations = Some(input.parse()?);
                evaluate_compile_time_exprpath_set_none!(
                    exprpath,
                    k,
                    check_translations
                );
            } else if k == "sync_html_tag_lang" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    sync_html_tag_lang
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "sync_html_tag_lang",
                )?;
                sync_html_tag_lang.push(param);
            } else if k == "sync_html_tag_dir" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
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
                    k
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath,
                    k,
                    input,
                    url_param,
                    "url_param"
                );
            } else if k == "initial_language_from_url_param" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_url_param
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_param",
                )?;
                initial_language_from_url_param.push(param);
            } else if k == "initial_language_from_url_param_to_localstorage" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_url_param_to_localstorage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_param_to_localstorage",
                )?;
                initial_language_from_url_param_to_localstorage.push(param);
            } else if k == "initial_language_from_url_param_to_cookie" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
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
                let mut param = Identifier::new();
                clone_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.ident = Some(k);
                } else {
                    param.ident = Some(input.parse()?);
                }
                initial_language_from_url_param_to_server_function.push(param);
            } else if k == "set_language_to_url_param" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    set_language_to_url_param
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_to_url_param",
                )?;
                set_language_to_url_param.push(param);
            } else if k == "localstorage_key" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    localstorage_key,
                    k
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath,
                    k,
                    input,
                    localstorage_key,
                    "localstorage_key"
                );
            } else if k == "initial_language_from_localstorage" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_localstorage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_localstorage",
                )?;
                initial_language_from_localstorage.push(param);
            } else if k == "initial_language_from_localstorage_to_cookie" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_localstorage_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_localstorage_to_cookie",
                )?;
                initial_language_from_localstorage_to_cookie.push(param);
            } else if k
                == "initial_language_from_localstorage_to_server_function"
            {
                let mut param = Identifier::new();
                clone_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.ident = Some(k);
                } else {
                    param.ident = Some(input.parse()?);
                }
                initial_language_from_localstorage_to_server_function
                    .push(param);
            } else if k == "set_language_to_localstorage" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    set_language_to_localstorage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_to_localstorage",
                )?;
                set_language_to_localstorage.push(param);
            } else if k == "initial_language_from_navigator" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_navigator
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_navigator",
                )?;
                initial_language_from_navigator.push(param);
            } else if k == "initial_language_from_navigator_to_localstorage" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_navigator_to_localstorage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_navigator_to_localstorage",
                )?;
                initial_language_from_navigator_to_localstorage.push(param);
            } else if k == "initial_language_from_navigator_to_cookie" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
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
                let mut param = Identifier::new();
                clone_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.ident = Some(k);
                } else {
                    param.ident = Some(input.parse()?);
                }
                initial_language_from_navigator_to_server_function.push(param);
            } else if k == "initial_language_from_accept_language_header" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_accept_language_header
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_accept_language_header",
                )?;
                initial_language_from_accept_language_header.push(param);
            } else if k == "set_language_from_navigator" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
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
                    k
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath,
                    k,
                    input,
                    cookie_name,
                    "cookie_name"
                );
            } else if k == "cookie_attrs" {
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    cookie_attrs,
                    k
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath,
                    k,
                    input,
                    cookie_attrs,
                    "cookie_attrs"
                );
            } else if k == "initial_language_from_cookie" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_cookie",
                )?;
                initial_language_from_cookie.push(param);
            } else if k == "initial_language_from_cookie_to_localstorage" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_cookie_to_localstorage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_cookie_to_localstorage",
                )?;
                initial_language_from_cookie_to_localstorage.push(param);
            } else if k == "initial_language_from_cookie_to_server_function" {
                let mut param = Identifier::new();
                clone_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.ident = Some(k);
                } else {
                    param.ident = Some(input.parse()?);
                }
                initial_language_from_cookie_to_server_function.push(param);
            } else if k == "set_language_to_cookie" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    set_language_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "set_language_to_cookie",
                )?;
                set_language_to_cookie.push(param);
            } else if k == "initial_language_from_server_function" {
                let mut param = Identifier::new();
                clone_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.ident = Some(k);
                } else {
                    param.ident = Some(input.parse()?);
                }
                initial_language_from_server_function.push(param);
            } else if k == "initial_language_from_server_function_to_cookie" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
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
            {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_server_function_to_localstorage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_server_function_to_localstorage",
                )?;
                initial_language_from_server_function_to_localstorage
                    .push(param);
            } else if k == "set_language_to_server_function" {
                let mut param = Identifier::new();
                clone_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.ident = Some(k);
                } else {
                    param.ident = Some(input.parse()?);
                }
                set_language_to_server_function.push(param);
            } else if k == "url_path" {
                if let Some(ref e) = exprpath {
                    let evaluated_exprpath = crate::evaluate_exprpath(e);
                    if !evaluated_exprpath.supported {
                        return Err(syn::Error::new(
                            e.span(),
                            exprpath_not_supported_error_message(e, &k),
                        ));
                    } else if evaluated_exprpath.result {
                        if struct_field_init_shorthand {
                            url_path = Some(k);
                        } else {
                            url_path = Some(input.parse()?);
                        }
                    }
                } else if struct_field_init_shorthand {
                    url_path = Some(k);
                } else {
                    url_path = Some(input.parse()?);
                }
            } else if k == "initial_language_from_url_path" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_url_path
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_path",
                )?;
                initial_language_from_url_path.push(param);
            } else if k == "initial_language_from_url_path_to_cookie" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_url_path_to_cookie
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_path_to_cookie",
                )?;
                initial_language_from_url_path_to_cookie.push(param);
            } else if k == "initial_language_from_url_path_to_localstorage" {
                let mut param = LitBoolExpr::new();
                clone_runtime_exprpath!(exprpath, param);
                parse_struct_field_init_shorthand!(
                    struct_field_init_shorthand,
                    param,
                    k,
                    initial_language_from_url_path_to_localstorage
                );
                parse_litbool_or_expr_param(
                    input,
                    &mut param.expr,
                    "initial_language_from_url_path_to_localstorage",
                )?;
                initial_language_from_url_path_to_localstorage.push(param);
            } else if k == "initial_language_from_url_path_to_server_function" {
                let mut param = Identifier::new();
                clone_runtime_exprpath!(exprpath, param);
                if struct_field_init_shorthand {
                    param.ident = Some(k);
                } else {
                    param.ident = Some(input.parse()?);
                }
                initial_language_from_url_path_to_server_function.push(param);
            } else if k == "initial_language_from_system" {
                #[cfg(feature = "system")]
                {
                    let mut param = LitBoolExpr::new();
                    clone_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k,
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
                    let mut param = LitBoolExpr::new();
                    clone_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k,
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
                    let mut param = LitBoolExpr::new();
                    clone_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k,
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
                    let mut param = LitBoolExpr::new();
                    clone_runtime_exprpath!(exprpath, param);
                    parse_struct_field_init_shorthand!(
                        struct_field_init_shorthand,
                        param,
                        k,
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
                    k
                );
                parse_litstr_or_expr_param_with_maybe_comptime_exprpath!(
                    exprpath,
                    k,
                    input,
                    data_file_key,
                    "data_file_key"
                );
            } else if k == "provide_meta_context" {
                let mut param = LitBool::new();
                clone_runtime_exprpath!(exprpath, param);
                struct_field_init_shorthand_not_supported!(
                    struct_field_init_shorthand,
                    k
                );
                param.lit = Some(input.parse()?);
                provide_meta_context.push(param);
            } else {
                return Err(syn::Error::new(
                    k.span(),
                    format!(
                        "Not a valid parameter '{k}' for leptos_fluent! macro."
                    ),
                ));
            }

            if input.is_empty() {
                break;
            }
            input.parse::<token::Comma>()?;
        }

        // translations
        let translations = translations.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing `translations` parameter")
        })?;

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
            .map(|langs| workspace_path.join(langs.value()));

        let locales_folder_path = locales_path
            .as_ref()
            .map(|locales| workspace_path.join(locales.value()))
            .unwrap();

        if let Some(ref file) = languages_file {
            if std::fs::metadata(file).is_err() {
                #[cfg(feature = "nightly")]
                let file_path =
                    std::path::absolute(file).unwrap_or(file.to_path_buf());
                #[cfg(not(feature = "nightly"))]
                let file_path = file.to_path_buf();

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
            #[cfg(feature = "nightly")]
            let file_path = std::path::absolute(&locales_folder_path)
                .unwrap_or(locales_folder_path.to_path_buf());
            #[cfg(not(feature = "nightly"))]
            let file_path = locales_folder_path.to_path_buf();

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
        #[cfg(not(feature = "ssr"))]
        let mut core_locales_content = None;
        let mut core_locales_path_str = None;
        if let Some(core_locales) = &core_locales_path {
            let core_locales = workspace_path.join(core_locales.value());
            if std::fs::metadata(&core_locales).is_err() {
                #[cfg(feature = "nightly")]
                let file_path = std::path::absolute(&core_locales)
                    .unwrap_or(core_locales.to_path_buf());
                #[cfg(not(feature = "nightly"))]
                let file_path = core_locales.to_path_buf();

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
                #[cfg(not(feature = "ssr"))]
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

        #[cfg(not(feature = "ssr"))]
        if let Some(ref check_translations_globstr) = check_translations {
            {
                let (fluent_resources, ref fluent_file_paths) =
                    fluent_resources_and_file_paths;
                let (check_messages, errors) = crate::translations_checker::run(
                    &check_translations_globstr.value(),
                    &workspace_path,
                    &fluent_resources,
                    fluent_file_paths,
                    &core_locales_path_str,
                    &core_locales_content,
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
                        check_translations_globstr.span(),
                        report,
                    ));
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

        Ok(Self {
            fluent_file_paths: fluent_resources_and_file_paths.1,
            translations,
            languages,
            languages_path: languages_file_path,
            raw_languages_path: languages_path.map(|x| x.value()),
            locales_path: locales_path.unwrap().value(),
            core_locales_path: core_locales_path_str,
            check_translations: check_translations.map(|x| x.value()),
            provide_meta_context,
            sync_html_tag_lang,
            sync_html_tag_dir,
            url_param,
            initial_language_from_url_param,
            initial_language_from_url_param_to_localstorage,
            initial_language_from_url_param_to_cookie,
            initial_language_from_url_param_to_server_function,
            set_language_to_url_param,
            localstorage_key,
            initial_language_from_localstorage,
            initial_language_from_localstorage_to_cookie,
            initial_language_from_localstorage_to_server_function,
            set_language_to_localstorage,
            initial_language_from_navigator,
            initial_language_from_navigator_to_localstorage,
            initial_language_from_navigator_to_cookie,
            initial_language_from_navigator_to_server_function,
            initial_language_from_accept_language_header,
            set_language_from_navigator,
            cookie_name,
            cookie_attrs,
            initial_language_from_cookie,
            initial_language_from_cookie_to_localstorage,
            initial_language_from_cookie_to_server_function,
            set_language_to_cookie,
            initial_language_from_server_function,
            initial_language_from_server_function_to_cookie,
            initial_language_from_server_function_to_localstorage,
            set_language_to_server_function,
            url_path,
            initial_language_from_url_path,
            initial_language_from_url_path_to_cookie,
            initial_language_from_url_path_to_localstorage,
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
        })
    }
}
