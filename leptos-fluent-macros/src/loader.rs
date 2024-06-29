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

pub(crate) struct I18nLoader {
    pub(crate) fluent_file_paths: FluentFilePaths,
    pub(crate) translations: Translations,
    pub(crate) languages: Vec<(String, String, String, Option<String>)>,
    pub(crate) languages_path: Option<String>,
    pub(crate) raw_languages_path: Option<String>,
    pub(crate) locales_path: String,
    pub(crate) core_locales_path: Option<String>,
    pub(crate) check_translations: Option<String>,
    pub(crate) provide_meta_context: bool,
    pub(crate) provide_meta_context_exprpath: Option<proc_macro2::TokenStream>,
    pub(crate) sync_html_tag_lang_bool: Option<syn::LitBool>,
    pub(crate) sync_html_tag_lang_expr: Option<syn::Expr>,
    pub(crate) sync_html_tag_lang_exprpath: Option<proc_macro2::TokenStream>,
    pub(crate) sync_html_tag_dir_bool: Option<syn::LitBool>,
    pub(crate) sync_html_tag_dir_expr: Option<syn::Expr>,
    pub(crate) sync_html_tag_dir_exprpath: Option<proc_macro2::TokenStream>,
    pub(crate) url_param_str: Option<syn::LitStr>,
    pub(crate) url_param_expr: Option<syn::Expr>,
    pub(crate) url_param_exprpath: Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_url_param_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_url_param_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_url_param_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_url_param_to_localstorage_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_url_param_to_localstorage_expr:
        Option<syn::Expr>,
    pub(crate) initial_language_from_url_param_to_localstorage_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_url_param_to_cookie_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_url_param_to_cookie_expr:
        Option<syn::Expr>,
    pub(crate) initial_language_from_url_param_to_cookie_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) set_language_to_url_param_bool: Option<syn::LitBool>,
    pub(crate) set_language_to_url_param_expr: Option<syn::Expr>,
    pub(crate) set_language_to_url_param_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) localstorage_key_str: Option<syn::LitStr>,
    pub(crate) localstorage_key_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_localstorage_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_localstorage_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_localstorage_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_localstorage_to_cookie_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_localstorage_to_cookie_expr:
        Option<syn::Expr>,
    pub(crate) initial_language_from_localstorage_to_cookie_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_localstorage_to_server_function:
        Option<syn::Ident>,
    pub(crate) initial_language_from_localstorage_to_server_function_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) set_language_to_localstorage_bool: Option<syn::LitBool>,
    pub(crate) set_language_to_localstorage_expr: Option<syn::Expr>,
    pub(crate) set_language_to_localstorage_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_navigator_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_navigator_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_navigator_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_navigator_to_localstorage_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_navigator_to_localstorage_expr:
        Option<syn::Expr>,
    pub(crate) initial_language_from_navigator_to_localstorage_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_navigator_to_cookie_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_navigator_to_cookie_expr:
        Option<syn::Expr>,
    pub(crate) initial_language_from_navigator_to_cookie_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_accept_language_header_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_accept_language_header_expr:
        Option<syn::Expr>,
    pub(crate) cookie_name_str: Option<syn::LitStr>,
    pub(crate) cookie_name_expr: Option<syn::Expr>,
    pub(crate) cookie_name_exprpath: Option<proc_macro2::TokenStream>,
    pub(crate) cookie_attrs_str: Option<syn::LitStr>,
    pub(crate) cookie_attrs_expr: Option<syn::Expr>,
    pub(crate) cookie_attrs_exprpath: Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_cookie_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_cookie_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_cookie_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_cookie_to_localstorage_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_cookie_to_localstorage_expr:
        Option<syn::Expr>,
    pub(crate) initial_language_from_cookie_to_localstorage_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_cookie_to_server_function:
        Option<syn::Ident>,
    pub(crate) initial_language_from_cookie_to_server_function_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) set_language_to_cookie_bool: Option<syn::LitBool>,
    pub(crate) set_language_to_cookie_expr: Option<syn::Expr>,
    pub(crate) set_language_to_cookie_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) initial_language_from_server_function: Option<syn::Ident>,
    pub(crate) initial_language_from_server_function_exprpath:
        Option<proc_macro2::TokenStream>,
    pub(crate) set_language_to_server_function: Option<syn::Ident>,
    pub(crate) set_language_to_server_function_exprpath:
        Option<proc_macro2::TokenStream>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_system_bool: Option<syn::LitBool>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_system_expr: Option<syn::Expr>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_system_exprpath:
        Option<proc_macro2::TokenStream>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_system_to_data_file_bool:
        Option<syn::LitBool>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_system_to_data_file_expr:
        Option<syn::Expr>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_system_to_data_file_exprpath:
        Option<proc_macro2::TokenStream>,
    #[cfg(feature = "system")]
    pub(crate) set_language_to_data_file_bool: Option<syn::LitBool>,
    #[cfg(feature = "system")]
    pub(crate) set_language_to_data_file_expr: Option<syn::Expr>,
    #[cfg(feature = "system")]
    pub(crate) set_language_to_data_file_exprpath:
        Option<proc_macro2::TokenStream>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_data_file_bool: Option<syn::LitBool>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_data_file_expr: Option<syn::Expr>,
    #[cfg(feature = "system")]
    pub(crate) initial_language_from_data_file_exprpath:
        Option<proc_macro2::TokenStream>,
    #[cfg(feature = "system")]
    pub(crate) data_file_key_str: Option<syn::LitStr>,
    #[cfg(feature = "system")]
    pub(crate) data_file_key_expr: Option<syn::Expr>,
    #[cfg(feature = "system")]
    pub(crate) data_file_key_exprpath: Option<proc_macro2::TokenStream>,
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
        let mut sync_html_tag_lang_bool: Option<syn::LitBool> = None;
        let mut sync_html_tag_lang_expr: Option<syn::Expr> = None;
        let mut sync_html_tag_lang_exprpath: Option<proc_macro2::TokenStream> =
            None;
        let mut sync_html_tag_dir_bool: Option<syn::LitBool> = None;
        let mut sync_html_tag_dir_expr: Option<syn::Expr> = None;
        let mut sync_html_tag_dir_exprpath: Option<proc_macro2::TokenStream> =
            None;
        let mut url_param_str: Option<syn::LitStr> = None;
        let mut url_param_expr: Option<syn::Expr> = None;
        let mut url_param_exprpath: Option<proc_macro2::TokenStream> = None;
        let mut initial_language_from_url_param_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_url_param_expr: Option<syn::Expr> = None;
        let mut initial_language_from_url_param_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_url_param_to_localstorage_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_url_param_to_localstorage_expr: Option<
            syn::Expr,
        > = None;
        let mut initial_language_from_url_param_to_localstorage_exprpath: Option<proc_macro2::TokenStream> =
            None;
        let mut initial_language_from_url_param_to_cookie_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_url_param_to_cookie_expr: Option<
            syn::Expr,
        > = None;
        let mut initial_language_from_url_param_to_cookie_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut set_language_to_url_param_bool: Option<syn::LitBool> = None;
        let mut set_language_to_url_param_expr: Option<syn::Expr> = None;
        let mut set_language_to_url_param_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut localstorage_key_str: Option<syn::LitStr> = None;
        let mut localstorage_key_expr: Option<syn::Expr> = None;
        let mut initial_language_from_localstorage_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_localstorage_expr: Option<syn::Expr> =
            None;
        let mut initial_language_from_localstorage_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_localstorage_to_cookie_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_localstorage_to_cookie_expr: Option<
            syn::Expr,
        > = None;
        let mut initial_language_from_localstorage_to_cookie_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_localstorage_to_server_function: Option<
            syn::Ident,
        > = None;
        let mut initial_language_from_localstorage_to_server_function_exprpath: Option<proc_macro2::TokenStream> =
            None;
        let mut set_language_to_localstorage_bool: Option<syn::LitBool> = None;
        let mut set_language_to_localstorage_expr: Option<syn::Expr> = None;
        let mut set_language_to_localstorage_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_navigator_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_navigator_expr: Option<syn::Expr> = None;
        let mut initial_language_from_navigator_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_navigator_to_localstorage_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_navigator_to_localstorage_expr: Option<
            syn::Expr,
        > = None;
        let mut initial_language_from_navigator_to_localstorage_exprpath: Option<proc_macro2::TokenStream> =
            None;
        let mut initial_language_from_navigator_to_cookie_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_navigator_to_cookie_expr: Option<
            syn::Expr,
        > = None;
        let mut initial_language_from_navigator_to_cookie_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_accept_language_header_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_accept_language_header_expr: Option<
            syn::Expr,
        > = None;
        let mut cookie_name_str: Option<syn::LitStr> = None;
        let mut cookie_name_expr: Option<syn::Expr> = None;
        let mut cookie_name_exprpath: Option<proc_macro2::TokenStream> = None;
        let mut cookie_attrs_str: Option<syn::LitStr> = None;
        let mut cookie_attrs_expr: Option<syn::Expr> = None;
        let mut cookie_attrs_exprpath: Option<proc_macro2::TokenStream> = None;
        let mut initial_language_from_cookie_bool: Option<syn::LitBool> = None;
        let mut initial_language_from_cookie_expr: Option<syn::Expr> = None;
        let mut initial_language_from_cookie_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_cookie_to_localstorage_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_cookie_to_localstorage_expr: Option<
            syn::Expr,
        > = None;
        let mut initial_language_from_cookie_to_localstorage_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_cookie_to_server_function: Option<
            syn::Ident,
        > = None;
        let mut initial_language_from_cookie_to_server_function_exprpath: Option<proc_macro2::TokenStream> =
            None;
        let mut set_language_to_cookie_bool: Option<syn::LitBool> = None;
        let mut set_language_to_cookie_expr: Option<syn::Expr> = None;
        let mut set_language_to_cookie_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut initial_language_from_server_function: Option<syn::Ident> =
            None;
        let mut initial_language_from_server_function_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        let mut set_language_to_server_function: Option<syn::Ident> = None;
        let mut set_language_to_server_function_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;

        #[cfg(feature = "system")]
        let mut initial_language_from_system_bool: Option<
            syn::LitBool,
        > = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_system_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_system_expr: Option<syn::Expr> = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_system_to_data_file_bool: Option<
            syn::LitBool,
        > = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_system_to_data_file_expr: Option<
            syn::Expr,
        > = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_system_to_data_file_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        #[cfg(feature = "system")]
        let mut set_language_to_data_file_bool: Option<syn::LitBool> = None;
        #[cfg(feature = "system")]
        let mut set_language_to_data_file_expr: Option<syn::Expr> = None;
        #[cfg(feature = "system")]
        let mut set_language_to_data_file_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_data_file_bool: Option<
            syn::LitBool,
        > = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_data_file_expr: Option<
            syn::Expr,
        > = None;
        #[cfg(feature = "system")]
        let mut initial_language_from_data_file_exprpath: Option<
            proc_macro2::TokenStream,
        > = None;

        let mut data_file_key_str: Option<syn::LitStr> = None;
        let mut data_file_key_expr: Option<syn::Expr> = None;
        let mut data_file_key_exprpath: Option<proc_macro2::TokenStream> = None;
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
                exprpath_not_supported!(exprpath, k);
            } else if k == "languages" {
                languages_path = Some(fields.parse()?);
                exprpath_not_supported!(exprpath, k);
            } else if k == "check_translations" {
                check_translations = Some(fields.parse()?);
                exprpath_not_supported!(exprpath, k);
            } else if k == "sync_html_tag_lang" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut sync_html_tag_lang_bool,
                    &mut sync_html_tag_lang_expr,
                    "sync_html_tag_lang",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    sync_html_tag_lang_exprpath.clone_from(&exprpath);
                }
            } else if k == "sync_html_tag_dir" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut sync_html_tag_dir_bool,
                    &mut sync_html_tag_dir_expr,
                    "sync_html_tag_dir",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    sync_html_tag_dir_exprpath.clone_from(&exprpath);
                }
            } else if k == "url_param" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut url_param_str,
                    &mut url_param_expr,
                    "url_param",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    url_param_exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_url_param" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param_bool,
                    &mut initial_language_from_url_param_expr,
                    "initial_language_from_url_param",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_url_param_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_url_param_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param_to_localstorage_bool,
                    &mut initial_language_from_url_param_to_localstorage_expr,
                    "initial_language_from_url_param_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_url_param_to_localstorage_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_url_param_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param_to_cookie_bool,
                    &mut initial_language_from_url_param_to_cookie_expr,
                    "initial_language_from_url_param_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_url_param_to_cookie_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_url_param" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_url_param_bool,
                    &mut set_language_to_url_param_expr,
                    "set_language_to_url_param",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    set_language_to_url_param_exprpath.clone_from(&exprpath);
                }
            } else if k == "localstorage_key" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut localstorage_key_str,
                    &mut localstorage_key_expr,
                    "localstorage_key",
                ) {
                    return Err(err);
                }
                exprpath_not_supported!(exprpath, k);
            } else if k == "initial_language_from_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_localstorage_bool,
                    &mut initial_language_from_localstorage_expr,
                    "initial_language_from_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_localstorage_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_localstorage_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_localstorage_to_cookie_bool,
                    &mut initial_language_from_localstorage_to_cookie_expr,
                    "initial_language_from_localstorage_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_localstorage_to_cookie_exprpath
                        .clone_from(&exprpath);
                }
            } else if k
                == "initial_language_from_localstorage_to_server_function"
            {
                initial_language_from_localstorage_to_server_function =
                    Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_localstorage_to_server_function_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_localstorage_bool,
                    &mut set_language_to_localstorage_expr,
                    "set_language_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    set_language_to_localstorage_exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_navigator" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator_bool,
                    &mut initial_language_from_navigator_expr,
                    "initial_language_from_navigator",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_navigator_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_navigator_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator_to_localstorage_bool,
                    &mut initial_language_from_navigator_to_localstorage_expr,
                    "initial_language_from_navigator_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_navigator_to_localstorage_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_navigator_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator_to_cookie_bool,
                    &mut initial_language_from_navigator_to_cookie_expr,
                    "initial_language_from_navigator_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_navigator_to_cookie_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_accept_language_header" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_accept_language_header_bool,
                    &mut initial_language_from_accept_language_header_expr,
                    "initial_language_from_accept_language_header",
                ) {
                    return Err(err);
                }
                exprpath_not_supported!(exprpath, k);
            } else if k == "cookie_name" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut cookie_name_str,
                    &mut cookie_name_expr,
                    "cookie_name",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    cookie_name_exprpath.clone_from(&exprpath);
                }
            } else if k == "cookie_attrs" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut cookie_attrs_str,
                    &mut cookie_attrs_expr,
                    "cookie_attrs",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    cookie_attrs_exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_cookie_bool,
                    &mut initial_language_from_cookie_expr,
                    "initial_language_from_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_cookie_exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_cookie_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_cookie_to_localstorage_bool,
                    &mut initial_language_from_cookie_to_localstorage_expr,
                    "initial_language_from_cookie_to_localstorage",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    initial_language_from_cookie_to_localstorage_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_cookie_to_server_function" {
                initial_language_from_cookie_to_server_function =
                    Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_cookie_to_server_function_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_cookie_bool,
                    &mut set_language_to_cookie_expr,
                    "set_language_to_cookie",
                ) {
                    return Err(err);
                }
                if exprpath.is_some() {
                    set_language_to_cookie_exprpath.clone_from(&exprpath);
                }
            } else if k == "initial_language_from_server_function" {
                initial_language_from_server_function = Some(fields.parse()?);
                if exprpath.is_some() {
                    initial_language_from_server_function_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "set_language_to_server_function" {
                set_language_to_server_function = Some(fields.parse()?);
                if exprpath.is_some() {
                    set_language_to_server_function_exprpath
                        .clone_from(&exprpath);
                }
            } else if k == "initial_language_from_system" {
                #[cfg(feature = "system")]
                {
                    if let Some(err) = parse_litbool_or_expr_param(
                        &fields,
                        &mut initial_language_from_system_bool,
                        &mut initial_language_from_system_expr,
                        "initial_language_from_system",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        initial_language_from_system_exprpath
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
                        &mut initial_language_from_data_file_bool,
                        &mut initial_language_from_data_file_expr,
                        "initial_language_from_data_file",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        initial_language_from_data_file_exprpath
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
                        &mut initial_language_from_system_to_data_file_bool,
                        &mut initial_language_from_system_to_data_file_expr,
                        "initial_language_from_system_to_data_file",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        initial_language_from_system_to_data_file_exprpath
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
                        &mut set_language_to_data_file_bool,
                        &mut set_language_to_data_file_expr,
                        "set_language_to_data_file",
                    ) {
                        return Err(err);
                    }
                    if exprpath.is_some() {
                        set_language_to_data_file_exprpath
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
                        &mut data_file_key_str,
                        &mut data_file_key_expr,
                        "data_file_key",
                    ) {
                        return Err(err);
                    }
                }
                if exprpath.is_some() {
                    data_file_key_exprpath.clone_from(&exprpath);
                }
                #[cfg(not(feature = "system"))]
                {
                    _ = data_file_key_exprpath;
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

        if let Some(ref cookie_attrs) = cookie_attrs_str {
            let cookie_attrs = cookie_attrs.value();
            let errors = validate_cookie_attrs(&cookie_attrs);
            if !errors.is_empty() {
                return Err(syn::Error::new(
                    cookie_attrs_str.unwrap().span(),
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
            sync_html_tag_lang_bool,
            sync_html_tag_lang_expr,
            sync_html_tag_lang_exprpath,
            sync_html_tag_dir_bool,
            sync_html_tag_dir_expr,
            sync_html_tag_dir_exprpath,
            url_param_str,
            url_param_expr,
            url_param_exprpath,
            initial_language_from_url_param_bool,
            initial_language_from_url_param_expr,
            initial_language_from_url_param_exprpath,
            initial_language_from_url_param_to_localstorage_bool,
            initial_language_from_url_param_to_localstorage_expr,
            initial_language_from_url_param_to_localstorage_exprpath,
            initial_language_from_url_param_to_cookie_bool,
            initial_language_from_url_param_to_cookie_expr,
            initial_language_from_url_param_to_cookie_exprpath,
            set_language_to_url_param_bool,
            set_language_to_url_param_expr,
            set_language_to_url_param_exprpath,
            localstorage_key_str,
            localstorage_key_expr,
            initial_language_from_localstorage_bool,
            initial_language_from_localstorage_expr,
            initial_language_from_localstorage_exprpath,
            initial_language_from_localstorage_to_cookie_bool,
            initial_language_from_localstorage_to_cookie_expr,
            initial_language_from_localstorage_to_cookie_exprpath,
            initial_language_from_localstorage_to_server_function,
            initial_language_from_localstorage_to_server_function_exprpath,
            set_language_to_localstorage_bool,
            set_language_to_localstorage_expr,
            set_language_to_localstorage_exprpath,
            initial_language_from_navigator_bool,
            initial_language_from_navigator_expr,
            initial_language_from_navigator_exprpath,
            initial_language_from_navigator_to_localstorage_bool,
            initial_language_from_navigator_to_localstorage_expr,
            initial_language_from_navigator_to_localstorage_exprpath,
            initial_language_from_navigator_to_cookie_bool,
            initial_language_from_navigator_to_cookie_expr,
            initial_language_from_navigator_to_cookie_exprpath,
            initial_language_from_accept_language_header_bool,
            initial_language_from_accept_language_header_expr,
            cookie_name_str,
            cookie_name_expr,
            cookie_name_exprpath,
            cookie_attrs_str,
            cookie_attrs_expr,
            cookie_attrs_exprpath,
            initial_language_from_cookie_bool,
            initial_language_from_cookie_expr,
            initial_language_from_cookie_exprpath,
            initial_language_from_cookie_to_localstorage_bool,
            initial_language_from_cookie_to_localstorage_expr,
            initial_language_from_cookie_to_localstorage_exprpath,
            initial_language_from_cookie_to_server_function,
            initial_language_from_cookie_to_server_function_exprpath,
            set_language_to_cookie_bool,
            set_language_to_cookie_expr,
            set_language_to_cookie_exprpath,
            initial_language_from_server_function,
            initial_language_from_server_function_exprpath,
            set_language_to_server_function,
            set_language_to_server_function_exprpath,
            #[cfg(feature = "system")]
            initial_language_from_system_bool,
            #[cfg(feature = "system")]
            initial_language_from_system_expr,
            #[cfg(feature = "system")]
            initial_language_from_system_exprpath,
            #[cfg(feature = "system")]
            initial_language_from_system_to_data_file_bool,
            #[cfg(feature = "system")]
            initial_language_from_system_to_data_file_expr,
            #[cfg(feature = "system")]
            initial_language_from_system_to_data_file_exprpath,
            #[cfg(feature = "system")]
            set_language_to_data_file_bool,
            #[cfg(feature = "system")]
            set_language_to_data_file_expr,
            #[cfg(feature = "system")]
            set_language_to_data_file_exprpath,
            #[cfg(feature = "system")]
            initial_language_from_data_file_bool,
            #[cfg(feature = "system")]
            initial_language_from_data_file_expr,
            #[cfg(feature = "system")]
            initial_language_from_data_file_exprpath,
            #[cfg(feature = "system")]
            data_file_key_str,
            #[cfg(feature = "system")]
            data_file_key_expr,
            #[cfg(feature = "system")]
            data_file_key_exprpath,
        })
    }
}
