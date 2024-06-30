use crate::{
    build_fluent_resources_and_file_paths,
    cookie::validate_cookie_attrs,
    languages::{read_languages_file, read_locales_folder},
    FluentFilePaths,
};
use quote::ToTokens;
use std::path::PathBuf;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token, Result,
};

fn parse_litstr_or_expr_param(
    fields: ParseStream,
    strlit: &mut Option<syn::LitStr>,
    expr: &mut Option<syn::Expr>,
    param_name: &'static str,
) -> Option<syn::Error> {
    match fields.parse::<syn::LitStr>() {
        Ok(lit) => {
            *strlit = Some(lit);
            None
        }
        Err(_) => match fields.parse::<syn::Expr>() {
            Ok(e) => {
                *expr = Some(e);
                None
            }
            Err(_) => Some(syn::Error::new(
                fields.span(),
                format!(
                    concat!(
                        "Not a valid value for '{}' of leptos_fluent! macro.",
                        " Must be a literal string or a valid expression.",
                        " Found {:?}",
                    ),
                    param_name, fields,
                ),
            )),
        },
    }
}

fn parse_litbool_or_expr_param(
    fields: ParseStream,
    litbool: &mut Option<syn::LitBool>,
    expr: &mut Option<syn::Expr>,
    param_name: &'static str,
) -> Option<syn::Error> {
    match fields.parse::<syn::LitBool>() {
        Ok(lit) => {
            *litbool = Some(lit);
            None
        }
        Err(_) => match fields.parse::<syn::Expr>() {
            Ok(e) => {
                *expr = Some(e);
                None
            }
            Err(_) => Some(syn::Error::new(
                fields.span(),
                format!(
                    concat!(
                        "Not a valid value for '{}' of leptos_fluent! macro.",
                        " Must be a literal boolean or a valid expression.",
                        " Found {:?}",
                    ),
                    param_name, fields,
                ),
            )),
        },
    }
}

/// A syntax part consisting of a list of simple loaders.
///
/// e.g. `[loader1, loader2]`
///
/// # Note
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
            "The parameter '{}' of",
            " leptos_fluent! macro does not accept an expression",
            " path like '{}'. Maybe in the future.",
            " Consider to move your configuration to a variable:\n\n",
            "```rust\n",
            "{}
{{
    let {}_dyn = {{ ... }};
}}

leptos_fluent! {{
    // ...
    {}: {}_dyn,
}};",
        ),
        k, exprpath_str, exprpath_str, k, k, k,
    )
}

macro_rules! exprpath_not_supported {
    ($exprpath:ident, $k:ident) => {
        if let Some(ref e) = $exprpath {
            return Err(syn::Error::new(
                e.span(),
                exprpath_not_supported_error_message(e, &$k),
            ));
        }
    };
}

pub(crate) struct LitBoolExpr {
    pub lit: Option<syn::LitBool>,
    pub expr: Option<syn::Expr>,
    pub exprpath: Option<proc_macro2::TokenStream>,
}

impl LitBoolExpr {
    pub fn new() -> Self {
        Self {
            lit: None,
            expr: None,
            exprpath: None,
        }
    }
}

pub(crate) struct LitStrExpr {
    pub lit: Option<syn::LitStr>,
    pub expr: Option<syn::Expr>,
    pub exprpath: Option<proc_macro2::TokenStream>,
}

impl LitStrExpr {
    pub fn new() -> Self {
        Self {
            lit: None,
            expr: None,
            exprpath: None,
        }
    }
}

pub(crate) struct Identifier {
    pub ident: Option<syn::Ident>,
    pub exprpath: Option<proc_macro2::TokenStream>,
}

impl Identifier {
    pub fn new() -> Self {
        Self {
            ident: None,
            exprpath: None,
        }
    }
}

pub(crate) struct I18nLoader {
    pub fluent_file_paths: FluentFilePaths,
    pub translations: Translations,
    pub languages: Vec<(String, String, String, Option<String>)>,
    pub languages_path: Option<String>,
    pub raw_languages_path: Option<String>,
    pub locales_path: String,
    pub core_locales_path: Option<String>,
    pub check_translations: Option<String>,
    pub provide_meta_context: bool,
    pub provide_meta_context_exprpath: Option<proc_macro2::TokenStream>,
    pub sync_html_tag_lang: LitBoolExpr,
    pub sync_html_tag_dir: LitBoolExpr,
    pub url_param: LitStrExpr,
    pub initial_language_from_url_param: LitBoolExpr,
    pub initial_language_from_url_param_to_localstorage: LitBoolExpr,
    pub initial_language_from_url_param_to_cookie: LitBoolExpr,
    pub initial_language_from_url_param_to_server_function: Identifier,
    pub set_language_to_url_param: LitBoolExpr,
    pub localstorage_key: LitStrExpr,
    pub initial_language_from_localstorage: LitBoolExpr,
    pub initial_language_from_localstorage_to_cookie: LitBoolExpr,
    pub initial_language_from_localstorage_to_server_function: Identifier,
    pub set_language_to_localstorage: LitBoolExpr,
    pub initial_language_from_navigator: LitBoolExpr,
    pub initial_language_from_navigator_to_localstorage: LitBoolExpr,
    pub initial_language_from_navigator_to_cookie: LitBoolExpr,
    pub initial_language_from_navigator_to_server_function: Identifier,
    pub initial_language_from_accept_language_header: LitBoolExpr,
    pub cookie_name: LitStrExpr,
    pub cookie_attrs: LitStrExpr,
    pub initial_language_from_cookie: LitBoolExpr,
    pub initial_language_from_cookie_to_localstorage: LitBoolExpr,
    pub initial_language_from_cookie_to_server_function: Identifier,
    pub set_language_to_cookie: LitBoolExpr,
    pub initial_language_from_server_function: Identifier,
    pub initial_language_from_server_function_to_cookie: LitBoolExpr,
    pub initial_language_from_server_function_to_localstorage: LitBoolExpr,
    pub set_language_to_server_function: Identifier,
    #[cfg(feature = "system")]
    pub initial_language_from_system: LitBoolExpr,
    #[cfg(feature = "system")]
    pub initial_language_from_system_to_data_file: LitBoolExpr,
    #[cfg(feature = "system")]
    pub set_language_to_data_file: LitBoolExpr,
    #[cfg(feature = "system")]
    pub initial_language_from_data_file: LitBoolExpr,
    #[cfg(feature = "system")]
    pub data_file_key: LitStrExpr,
}

impl Parse for I18nLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        let workspace_path = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".into()),
        );

        let fields;
        braced!(fields in input);
        let mut locales_path: Option<syn::LitStr> = None;
        let mut languages_path: Option<syn::LitStr> = None;
        let mut core_locales_path: Option<syn::LitStr> = None;
        let mut translations: Option<Translations> = None;
        let mut check_translations: Option<syn::LitStr> = None;
        let mut sync_html_tag_lang = LitBoolExpr::new();
        let mut sync_html_tag_dir = LitBoolExpr::new();
        let mut url_param = LitStrExpr::new();
        let mut initial_language_from_url_param = LitBoolExpr::new();
        let mut initial_language_from_url_param_to_localstorage =
            LitBoolExpr::new();
        let mut initial_language_from_url_param_to_cookie = LitBoolExpr::new();
        let mut initial_language_from_url_param_to_server_function =
            Identifier::new();
        let mut set_language_to_url_param = LitBoolExpr::new();
        let mut localstorage_key = LitStrExpr::new();
        let mut initial_language_from_localstorage = LitBoolExpr::new();
        let mut initial_language_from_localstorage_to_cookie =
            LitBoolExpr::new();
        let mut initial_language_from_localstorage_to_server_function =
            Identifier::new();
        let mut set_language_to_localstorage = LitBoolExpr::new();
        let mut initial_language_from_navigator = LitBoolExpr::new();
        let mut initial_language_from_navigator_to_localstorage =
            LitBoolExpr::new();
        let mut initial_language_from_navigator_to_cookie = LitBoolExpr::new();
        let mut initial_language_from_navigator_to_server_function =
            Identifier::new();
        let mut initial_language_from_accept_language_header =
            LitBoolExpr::new();
        let mut cookie_name = LitStrExpr::new();
        let mut cookie_attrs = LitStrExpr::new();
        let mut initial_language_from_cookie = LitBoolExpr::new();
        let mut initial_language_from_cookie_to_localstorage =
            LitBoolExpr::new();
        let mut initial_language_from_cookie_to_server_function =
            Identifier::new();
        let mut set_language_to_cookie = LitBoolExpr::new();
        let mut initial_language_from_server_function = Identifier::new();
        let mut initial_language_from_server_function_to_cookie =
            LitBoolExpr::new();
        let mut initial_language_from_server_function_to_localstorage =
            LitBoolExpr::new();
        let mut set_language_to_server_function = Identifier::new();

        #[cfg(feature = "system")]
        let mut initial_language_from_system = LitBoolExpr::new();
        #[cfg(feature = "system")]
        let mut initial_language_from_system_to_data_file = LitBoolExpr::new();
        #[cfg(feature = "system")]
        let mut set_language_to_data_file = LitBoolExpr::new();
        #[cfg(feature = "system")]
        let mut initial_language_from_data_file = LitBoolExpr::new();
        let mut data_file_key = LitStrExpr::new();
        let mut provide_meta_context: Option<syn::LitBool> = None;
        let mut provide_meta_context_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;

        while !fields.is_empty() {
            let mut exprpath: Option<proc_macro2::TokenStream> = None;
            let k;
            if fields.peek(syn::Ident) && fields.peek2(syn::Token![:]) {
                k = fields.parse::<syn::Ident>()?;
                // expression:
            } else {
                let expr = fields.parse::<syn::Expr>()?;
                if matches!(expr, syn::Expr::Path(_)) {
                    let span = expr.span();
                    let string = expr.to_token_stream().to_string();
                    let splitter =
                        if string.contains('\n') { "\n" } else { "] " };
                    let ident = &string
                        .split(splitter)
                        .last()
                        .unwrap()
                        .replace('\n', " ");
                    k = syn::Ident::new(ident, span);

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

            fields.parse::<syn::Token![:]>()?;

            if k == "translations" {
                translations = Some(fields.parse()?);
                exprpath_not_supported!(exprpath, k);
            } else if k == "locales" {
                locales_path = Some(fields.parse()?);
                exprpath_not_supported!(exprpath, k);
            } else if k == "core_locales" {
                core_locales_path = Some(fields.parse()?);
                if let Some(ref e) = exprpath {
                    if e.to_string() == "#[cfg(debug_assertions)]" {
                        #[cfg(not(debug_assertions))]
                        {
                            core_locales_path = None;
                        }
                    } else if e.to_string() == "#[cfg(not(debug_assertions))]" {
                        #[cfg(debug_assertions)]
                        {
                            core_locales_path = None;
                        }
                    } else {
                        exprpath_not_supported!(exprpath, k);
                    }
                }
            } else if k == "languages" {
                languages_path = Some(fields.parse()?);
                if let Some(ref e) = exprpath {
                    if e.to_string() == "#[cfg(debug_assertions)]" {
                        #[cfg(not(debug_assertions))]
                        {
                            languages_path = None;
                        }
                    } else if e.to_string() == "#[cfg(not(debug_assertions))]" {
                        #[cfg(debug_assertions)]
                        {
                            languages_path = None;
                        }
                    } else {
                        exprpath_not_supported!(exprpath, k);
                    }
                }
            } else if k == "check_translations" {
                check_translations = Some(fields.parse()?);
                if let Some(ref e) = exprpath {
                    if e.to_string() == "#[cfg(debug_assertions)]" {
                        #[cfg(not(debug_assertions))]
                        {
                            check_translations = None;
                        }
                    } else if e.to_string() == "#[cfg(not(debug_assertions))]" {
                        #[cfg(debug_assertions)]
                        {
                            check_translations = None;
                        }
                    } else {
                        exprpath_not_supported!(exprpath, k);
                    }
                }
            } else if k == "sync_html_tag_lang" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut sync_html_tag_lang.lit,
                    &mut sync_html_tag_lang.expr,
                    "sync_html_tag_lang",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    sync_html_tag_lang.exprpath.clone_from(&exprpath);
                }
            } else if k == "sync_html_tag_dir" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut sync_html_tag_dir.lit,
                    &mut sync_html_tag_dir.expr,
                    "sync_html_tag_dir",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    sync_html_tag_dir.exprpath.clone_from(&exprpath);
                }
            } else if k == "url_param" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut url_param.lit,
                    &mut url_param.expr,
                    "url_param",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    url_param.exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_url_param" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param.lit,
                    &mut initial_language_from_url_param.expr,
                    "initial_language_from_url_param",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_url_param
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_url_param_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param_to_localstorage.lit,
                    &mut initial_language_from_url_param_to_localstorage.expr,
                    "initial_language_from_url_param_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_url_param_to_localstorage
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_url_param_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param_to_cookie.lit,
                    &mut initial_language_from_url_param_to_cookie.expr,
                    "initial_language_from_url_param_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_url_param_to_cookie
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_url_param_to_server_function"
            {
                initial_language_from_url_param_to_server_function.ident =
                    Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_url_param_to_server_function
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_url_param" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_url_param.lit,
                    &mut set_language_to_url_param.expr,
                    "set_language_to_url_param",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    set_language_to_url_param.exprpath.clone_from(&exprpath);
                }
            } else if k == "localstorage_key" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut localstorage_key.lit,
                    &mut localstorage_key.expr,
                    "localstorage_key",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    localstorage_key.exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_localstorage.lit,
                    &mut initial_language_from_localstorage.expr,
                    "initial_language_from_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_localstorage
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_localstorage_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_localstorage_to_cookie.lit,
                    &mut initial_language_from_localstorage_to_cookie.expr,
                    "initial_language_from_localstorage_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_localstorage_to_cookie
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k
                == "initial_language_from_localstorage_to_server_function"
            {
                initial_language_from_localstorage_to_server_function.ident =
                    Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_localstorage_to_server_function
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_localstorage.lit,
                    &mut set_language_to_localstorage.expr,
                    "set_language_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    set_language_to_localstorage.exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_navigator" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator.lit,
                    &mut initial_language_from_navigator.expr,
                    "initial_language_from_navigator",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_navigator
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_navigator_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator_to_localstorage.lit,
                    &mut initial_language_from_navigator_to_localstorage.expr,
                    "initial_language_from_navigator_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_navigator_to_localstorage
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_navigator_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator_to_cookie.lit,
                    &mut initial_language_from_navigator_to_cookie.expr,
                    "initial_language_from_navigator_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_navigator_to_cookie
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_navigator_to_server_function"
            {
                initial_language_from_navigator_to_server_function.ident =
                    Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_navigator_to_server_function
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_accept_language_header" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_accept_language_header.lit,
                    &mut initial_language_from_accept_language_header.expr,
                    "initial_language_from_accept_language_header",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_accept_language_header
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "cookie_name" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut cookie_name.lit,
                    &mut cookie_name.expr,
                    "cookie_name",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    cookie_name.exprpath.clone_from(&exprpath);
                }
            } else if k == "cookie_attrs" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut cookie_attrs.lit,
                    &mut cookie_attrs.expr,
                    "cookie_attrs",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    cookie_attrs.exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_cookie.lit,
                    &mut initial_language_from_cookie.expr,
                    "initial_language_from_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_cookie.exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_cookie_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_cookie_to_localstorage.lit,
                    &mut initial_language_from_cookie_to_localstorage.expr,
                    "initial_language_from_cookie_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_cookie_to_localstorage
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_cookie_to_server_function" {
                initial_language_from_cookie_to_server_function.ident =
                    Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_cookie_to_server_function
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_cookie.lit,
                    &mut set_language_to_cookie.expr,
                    "set_language_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    set_language_to_cookie.exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_server_function" {
                initial_language_from_server_function.ident =
                    Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_server_function
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_server_function_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_server_function_to_cookie.lit,
                    &mut initial_language_from_server_function_to_cookie.expr,
                    "initial_language_from_server_function_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_server_function_to_cookie
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k
                == "initial_language_from_server_function_to_localstorage"
            {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_server_function_to_localstorage
                        .lit,
                    &mut initial_language_from_server_function_to_localstorage
                        .expr,
                    "initial_language_from_server_function_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_server_function_to_localstorage
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_server_function" {
                set_language_to_server_function.ident = Some(fields.parse()?);
                if exprpath.is_some() {
                    set_language_to_server_function
                        .exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_system" {
                #[cfg(feature = "system")]
                {
                    if let Some(err) = parse_litbool_or_expr_param(
                        &fields,
                        &mut initial_language_from_system.lit,
                        &mut initial_language_from_system.expr,
                        "initial_language_from_system",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        initial_language_from_system
                            .exprpath
                            .clone_from(&exprpath);
                    }
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
                    if let Some(err) = parse_litbool_or_expr_param(
                        &fields,
                        &mut initial_language_from_data_file.lit,
                        &mut initial_language_from_data_file.expr,
                        "initial_language_from_data_file",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        initial_language_from_data_file
                            .exprpath
                            .clone_from(&exprpath);
                    }
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
                    if let Some(err) = parse_litbool_or_expr_param(
                        &fields,
                        &mut initial_language_from_system_to_data_file.lit,
                        &mut initial_language_from_system_to_data_file.expr,
                        "initial_language_from_system_to_data_file",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        initial_language_from_system_to_data_file
                            .exprpath
                            .clone_from(&exprpath);
                    }
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
                    if let Some(err) = parse_litbool_or_expr_param(
                        &fields,
                        &mut set_language_to_data_file.lit,
                        &mut set_language_to_data_file.expr,
                        "set_language_to_data_file",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        set_language_to_data_file
                            .exprpath
                            .clone_from(&exprpath);
                    }
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
                {
                    if let Some(err) = parse_litstr_or_expr_param(
                        &fields,
                        &mut data_file_key.lit,
                        &mut data_file_key.expr,
                        "data_file_key",
                    ) {
                        return Err(err);
                    }
                }
                if exprpath.is_some() {
                    data_file_key.exprpath.clone_from(&exprpath);
                }
                #[cfg(not(feature = "system"))]
                {
                    _ = data_file_key.exprpath;
                }
            } else if k == "provide_meta_context" {
                provide_meta_context = Some(fields.parse()?);
                if exprpath.is_some() {
                    provide_meta_context_exprpath.clone_from(&exprpath);
                }
            } else {
                return Err(syn::Error::new(
                    k.span(),
                    format!(
                        "Not a valid parameter '{k}' for leptos_fluent! macro."
                    ),
                ));
            }

            if fields.is_empty() {
                break;
            }
            fields.parse::<token::Comma>()?;
        }

        // translations
        let translations = translations.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing `translations` field")
        })?;

        // languages
        if locales_path.is_none() {
            return Err(syn::Error::new(
                input.span(),
                concat!(
                    "The `locales` field is required by leptos_fluent! macro.",
                ),
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
                return Err(syn::Error::new(
                    languages_path.as_ref().unwrap().span(),
                    format!(
                        concat!(
                            "Couldn't read languages file, this path should",
                            " be relative to your crate's `Cargo.toml`.",
                            " Looking for: {:?}",
                        ),
                        // TODO: Use std::path::absolute from
                        // #![feature(absolute_path)] when stable,
                        // see https://github.com/rust-lang/rust/issues/92750
                        file,
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
            return Err(syn::Error::new(
                locales_path.as_ref().unwrap().span(),
                format!(
                    concat!(
                        "Couldn't read locales folder, this path should",
                        " be relative to your crate's `Cargo.toml`.",
                        " Looking for: {:?}",
                    ),
                    // TODO: Use std::path::absolute from
                    // #![feature(absolute_path)] when stable,
                    // see https://github.com/rust-lang/rust/issues/92750
                    &locales_folder_path,
                ),
            ));
        } else {
            languages = read_locales_folder(&locales_folder_path);
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
                return Err(syn::Error::new(
                    core_locales_path.unwrap().span(),
                    format!(
                        concat!(
                            "Couldn't read core fluent resource, this path should",
                            " be relative to your crate's `Cargo.toml`.",
                            " Looking for: {:?}",
                        ),
                        core_locales,
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
            provide_meta_context: match provide_meta_context {
                Some(x) => x.value,
                None => false,
            },
            provide_meta_context_exprpath,
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
