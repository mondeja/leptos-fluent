use crate::languages::{read_languages_file, read_locales_folder};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token, Ident, Result,
};

pub(crate) fn read_from_dir<P: AsRef<Path>>(
    path: P,
) -> (Vec<String>, Vec<String>) {
    let mut paths = vec![];
    let mut contents = vec![];

    walkdir::WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |e| e == "ftl"))
        .for_each(|e| {
            let p = e.path().to_owned().as_path().to_str().unwrap().to_string();
            if let Ok(string) = std::fs::read_to_string(&p) {
                paths.push(p);
                contents.push(string);
            }
            // TODO: Handle error
        });

    (paths, contents)
}

/// Copied from `fluent_templates/macros` to ensure that the same implementation
/// is followed.
fn build_fluent_resources(
    dir: impl AsRef<std::path::Path>,
) -> HashMap<String, (Vec<String>, Vec<String>)> {
    let mut all_resources = HashMap::new();
    for entry in std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|rs| rs.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
    {
        if let Some(lang) = entry.file_name().into_string().ok().filter(|l| {
            l.parse::<fluent_templates::LanguageIdentifier>().is_ok()
        }) {
            let (file_paths, resources) = read_from_dir(entry.path());
            all_resources.insert(lang, (file_paths, resources));
        }
    }
    all_resources
}

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
    pub(crate) languages_path: Option<String>,
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
    pub(crate) fluent_resources: HashMap<String, (Vec<String>, Vec<String>)>,
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
        #[cfg(not(feature = "ssr"))]
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
                #[cfg(not(feature = "ssr"))]
                {
                    check_translations = Some(fields.parse()?);
                }

                #[cfg(feature = "ssr")]
                {
                    _ = fields.parse::<syn::Expr>()?;
                }
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
                #[cfg(not(feature = "ssr"))]
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

        let languages_path_copy = languages_path.clone();
        let languages_file = languages_path
            .as_ref()
            .map(|languages| workspace_path.join(languages.value()));

        let locales_path_copy = locales_path.clone();
        let locales_folder_path = locales_path
            .as_ref()
            .map(|locales| workspace_path.join(locales.value()))
            .unwrap();

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
                let langs_path = &languages_file.unwrap();
                languages = read_languages_file(langs_path);
                languages_file_path =
                    Some(langs_path.as_path().to_str().unwrap().to_string());
            }
        } else if std::fs::metadata(&locales_folder_path).is_err() {
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
                    &locales_folder_path,
                ),
            ));
        } else {
            languages = read_locales_folder(&locales_folder_path);
        }

        let locales_path_str =
            locales_folder_path.as_path().to_str().unwrap().to_string();
        let fluent_resources = build_fluent_resources(locales_path_str);

        #[cfg(not(feature = "ssr"))]
        if let Some(ref check_translations_globstr) = check_translations {
            {
                match crate::translations_checker::run(
                    &check_translations_globstr.value(),
                    &workspace_path,
                    &fluent_resources,
                ) {
                    Ok(check_translations_error_messages) => {
                        if !check_translations_error_messages.is_empty() {
                            return Err(syn::Error::new(
                                check_translations_globstr.span(),
                                format!(
                                    "Translations check failed:\n- {}",
                                    check_translations_error_messages
                                        .join("\n- "),
                                ),
                            ));
                        }
                    }
                    Err(errors) => {
                        return Err(syn::Error::new(
                            check_translations_globstr.span(),
                            format!(
                                "Unrecoverable errors checking translations:\n- {}",
                                errors.join("\n- "),
                            )
                        ));
                    }
                };
            }
        }

        Ok(Self {
            translations_ident,
            languages,
            languages_path: languages_file_path,
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
            fluent_resources,
        })
    }
}
