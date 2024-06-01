use crate::languages::{read_languages_file, read_locales_folder};
use crate::translations_checker;
use std::path::PathBuf;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token, Ident, Result,
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

pub(crate) struct I18nLoader {
    pub(crate) languages: Vec<(String, String)>,
    pub(crate) translations_ident: syn::Ident,
    pub(crate) sync_html_tag_lang_bool: Option<syn::LitBool>,
    pub(crate) sync_html_tag_lang_expr: Option<syn::Expr>,
    pub(crate) url_param_str: Option<syn::LitStr>,
    pub(crate) url_param_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_url_param_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_url_param_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_url_param_to_localstorage_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_url_param_to_localstorage_expr:
        Option<syn::Expr>,
    pub(crate) set_language_to_url_param_bool: Option<syn::LitBool>,
    pub(crate) set_language_to_url_param_expr: Option<syn::Expr>,
    pub(crate) localstorage_key_str: Option<syn::LitStr>,
    pub(crate) localstorage_key_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_localstorage_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_localstorage_expr: Option<syn::Expr>,
    pub(crate) set_language_to_localstorage_bool: Option<syn::LitBool>,
    pub(crate) set_language_to_localstorage_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_navigator_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_navigator_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_accept_language_header_bool:
        Option<syn::LitBool>,
    pub(crate) initial_language_from_accept_language_header_expr:
        Option<syn::Expr>,
    pub(crate) cookie_name_str: Option<syn::LitStr>,
    pub(crate) cookie_name_expr: Option<syn::Expr>,
    pub(crate) initial_language_from_cookie_bool: Option<syn::LitBool>,
    pub(crate) initial_language_from_cookie_expr: Option<syn::Expr>,
    pub(crate) set_language_to_cookie_bool: Option<syn::LitBool>,
    pub(crate) set_language_to_cookie_expr: Option<syn::Expr>,
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
        let mut translations_identifier: Option<syn::Ident> = None;
        let mut check_translations: Option<syn::LitStr> = None;
        let mut sync_html_tag_lang_bool: Option<syn::LitBool> = None;
        let mut sync_html_tag_lang_expr: Option<syn::Expr> = None;
        let mut url_param_str: Option<syn::LitStr> = None;
        let mut url_param_expr: Option<syn::Expr> = None;
        let mut initial_language_from_url_param_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_url_param_expr: Option<syn::Expr> = None;
        let mut initial_language_from_url_param_to_localstorage_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_url_param_to_localstorage_expr: Option<
            syn::Expr,
        > = None;
        let mut set_language_to_url_param_bool: Option<syn::LitBool> = None;
        let mut set_language_to_url_param_expr: Option<syn::Expr> = None;
        let mut localstorage_key_str: Option<syn::LitStr> = None;
        let mut localstorage_key_expr: Option<syn::Expr> = None;
        let mut initial_language_from_localstorage_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_localstorage_expr: Option<syn::Expr> =
            None;
        let mut set_language_to_localstorage_bool: Option<syn::LitBool> = None;
        let mut set_language_to_localstorage_expr: Option<syn::Expr> = None;
        let mut initial_language_from_navigator_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_navigator_expr: Option<syn::Expr> = None;
        let mut initial_language_from_accept_language_header_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_accept_language_header_expr: Option<
            syn::Expr,
        > = None;
        let mut cookie_name_str: Option<syn::LitStr> = None;
        let mut cookie_name_expr: Option<syn::Expr> = None;
        let mut initial_language_from_cookie_bool: Option<syn::LitBool> = None;
        let mut initial_language_from_cookie_expr: Option<syn::Expr> = None;
        let mut set_language_to_cookie_bool: Option<syn::LitBool> = None;
        let mut set_language_to_cookie_expr: Option<syn::Expr> = None;

        while !fields.is_empty() {
            let k = fields.parse::<Ident>()?;
            fields.parse::<syn::Token![:]>()?;

            if k == "translations" {
                translations_identifier = Some(fields.parse()?);
            } else if k == "locales" {
                locales_path = Some(fields.parse()?);
            } else if k == "languages" {
                languages_path = Some(fields.parse()?);
            } else if k == "check_translations" {
                check_translations = Some(fields.parse()?);
            } else if k == "sync_html_tag_lang" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut sync_html_tag_lang_bool,
                    &mut sync_html_tag_lang_expr,
                    "sync_html_tag_lang",
                ) {
                    return Err(err);
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
            } else if k == "initial_language_from_url_param" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param_bool,
                    &mut initial_language_from_url_param_expr,
                    "initial_language_from_url_param",
                ) {
                    return Err(err);
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
            } else if k == "set_language_to_url_param" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_url_param_bool,
                    &mut set_language_to_url_param_expr,
                    "set_language_to_url_param",
                ) {
                    return Err(err);
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
            } else if k == "initial_language_from_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_localstorage_bool,
                    &mut initial_language_from_localstorage_expr,
                    "initial_language_from_localstorage",
                ) {
                    return Err(err);
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
            } else if k == "initial_language_from_navigator" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator_bool,
                    &mut initial_language_from_navigator_expr,
                    "initial_language_from_navigator",
                ) {
                    return Err(err);
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
            } else if k == "cookie_name" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut cookie_name_str,
                    &mut cookie_name_expr,
                    "cookie_name",
                ) {
                    return Err(err);
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
            } else if k == "set_language_to_cookie" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut set_language_to_cookie_bool,
                    &mut set_language_to_cookie_expr,
                    "set_language_to_cookie",
                ) {
                    return Err(err);
                }
            } else {
                return Err(syn::Error::new(
                    k.span(),
                    "Not a valid parameter for leptos_fluent! macro.",
                ));
            }

            if fields.is_empty() {
                break;
            }
            fields.parse::<token::Comma>()?;
        }

        // translations
        let translations_ident = translations_identifier.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing `translations` field")
        })?;

        // languages
        if languages_path.is_none() && locales_path.is_none() {
            return Err(syn::Error::new(
                input.span(),
                concat!(
                    "Either `languages` or `locales` field is required",
                    " by leptos_fluent! macro.",
                ),
            ));
        }

        let mut languages = Vec::new();

        let languages_path_copy = languages_path.clone();
        let languages_file = languages_path
            .map(|languages| workspace_path.join(languages.value()));

        if let Some(ref file) = languages_file {
            if std::fs::metadata(file).is_err() {
                return Err(syn::Error::new(
                    languages_path_copy.unwrap().span(),
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
                languages = read_languages_file(&languages_file.unwrap());

                if languages.len() < 2 {
                    return Err(syn::Error::new(
                        languages_path_copy.unwrap().span(),
                        "Languages file must contain at least two languages.",
                    ));
                }
            }
        } else {
            // locales
            let locales_path_copy = locales_path.clone();
            let locales_folder = locales_path
                .as_ref()
                .map(|locales| workspace_path.join(locales.value()));

            if let Some(ref folder) = locales_folder {
                if std::fs::metadata(folder).is_err() {
                    return Err(syn::Error::new(
                        locales_path_copy.unwrap().span(),
                        format!(
                            concat!(
                                "Couldn't read locales folder, this path should",
                                " be relative to your crate's `Cargo.toml`.",
                                " Looking for: {:?}",
                            ),
                            // TODO: Use std::path::absolute from
                            // #![feature(absolute_path)] when stable,
                            // see https://github.com/rust-lang/rust/issues/92750
                            folder,
                        ),
                    ));
                } else {
                    languages = read_locales_folder(&locales_folder.unwrap());

                    if languages.len() < 2 {
                        return Err(syn::Error::new(
                            locales_path_copy.unwrap().span(),
                            "Locales folder must contain at least two languages.",
                        ));
                    }
                }
            }
        }

        if let Some(check_translations_globstr) = check_translations {
            if locales_path.is_none() {
                return Err(syn::Error::new(
                    check_translations_globstr.span(),
                    concat!(
                        "You must provide a `locales` parameter",
                        " to use the `check_translations` parameter.",
                    ),
                ));
            }

            let translations_check_result = translations_checker::run(
                &check_translations_globstr.value(),
                &locales_path.unwrap().value(),
                &workspace_path,
            );
            if let Err(err) = translations_check_result {
                return Err(err);
            }
        }

        Ok(Self {
            translations_ident,
            languages,
            sync_html_tag_lang_bool,
            sync_html_tag_lang_expr,
            url_param_str,
            url_param_expr,
            initial_language_from_url_param_bool,
            initial_language_from_url_param_expr,
            initial_language_from_url_param_to_localstorage_bool,
            initial_language_from_url_param_to_localstorage_expr,
            set_language_to_url_param_bool,
            set_language_to_url_param_expr,
            localstorage_key_str,
            localstorage_key_expr,
            initial_language_from_localstorage_bool,
            initial_language_from_localstorage_expr,
            set_language_to_localstorage_bool,
            set_language_to_localstorage_expr,
            initial_language_from_navigator_bool,
            initial_language_from_navigator_expr,
            initial_language_from_accept_language_header_bool,
            initial_language_from_accept_language_header_expr,
            cookie_name_str,
            cookie_name_expr,
            initial_language_from_cookie_bool,
            initial_language_from_cookie_expr,
            set_language_to_cookie_bool,
            set_language_to_cookie_expr,
        })
    }
}
