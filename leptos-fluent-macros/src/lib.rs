#![deny(missing_docs)]
#![forbid(unsafe_code)]
//! Macros for the leptos-fluent crate.
//!
//! See [leptos-fluent] for more information.
//!
//! [leptos-fluent]: https://crates.io/crates/leptos-fluent

extern crate proc_macro;

mod languages;

use languages::{
    generate_code_for_static_language, read_languages_file, read_locales_folder,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, Result,
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

struct I18nLoader {
    languages: Vec<(String, String)>,
    translations_ident: syn::Ident,
    sync_html_tag_lang_bool: Option<syn::LitBool>,
    sync_html_tag_lang_expr: Option<syn::Expr>,
    url_param_str: Option<syn::LitStr>,
    url_param_expr: Option<syn::Expr>,
    initial_language_from_url_param_bool: Option<syn::LitBool>,
    initial_language_from_url_param_expr: Option<syn::Expr>,
    initial_language_from_url_param_to_localstorage_bool: Option<syn::LitBool>,
    initial_language_from_url_param_to_localstorage_expr: Option<syn::Expr>,
    set_language_to_url_param_bool: Option<syn::LitBool>,
    set_language_to_url_param_expr: Option<syn::Expr>,
    localstorage_key_str: Option<syn::LitStr>,
    localstorage_key_expr: Option<syn::Expr>,
    initial_language_from_localstorage_bool: Option<syn::LitBool>,
    initial_language_from_localstorage_expr: Option<syn::Expr>,
    set_language_to_localstorage_bool: Option<syn::LitBool>,
    set_language_to_localstorage_expr: Option<syn::Expr>,
    initial_language_from_navigator_bool: Option<syn::LitBool>,
    initial_language_from_navigator_expr: Option<syn::Expr>,
    initial_language_from_accept_language_header_bool: Option<syn::LitBool>,
    initial_language_from_accept_language_header_expr: Option<syn::Expr>,
    cookie_name_str: Option<syn::LitStr>,
    cookie_name_expr: Option<syn::Expr>,
    initial_language_from_cookie_bool: Option<syn::LitBool>,
    initial_language_from_cookie_expr: Option<syn::Expr>,
    set_language_to_cookie_bool: Option<syn::LitBool>,
    set_language_to_cookie_expr: Option<syn::Expr>,
}

impl Parse for I18nLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        let workspace_path = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".into()),
        );

        let fields;
        braced!(fields in input);
        let mut locales_path: Option<syn::LitStr> = None;
        let mut translations_identifier: Option<syn::Ident> = None;
        let mut languages_path: Option<syn::LitStr> = None;
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

/// Create the i18n context for internationalization.
///
/// # Example
///
/// ```rust,ignore
/// use fluent_templates::static_loader;
/// use leptos::*;
/// use leptos_fluent::leptos_fluent;
///
/// static_loader! {
///     static TRANSLATIONS = {
///         locales: "./locales",
///         fallback_language: "en-US",
///     };
/// }
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     leptos_fluent! {{
///         translations: TRANSLATIONS,
///         languages: "./locales/languages.json",
///         sync_html_tag_lang: true,
///         url_param: "lang",
///         initial_language_from_url_param: true,
///         initial_language_from_url_param_to_localstorage: true,
///         set_language_to_url_param: true,
///         localstorage_key: "language",
///         initial_language_from_localstorage: true,
///         set_language_to_localstorage: true,
///         initial_language_from_navigator: true,
///         initial_language_from_accept_language_header: true,
///         cookie_name: "lang",
///         initial_language_from_cookie: true,
///         set_language_to_cookie: true,
///     }};
///
///     view! {
///         ...
///     }
/// }
/// ```
///
/// ## Arguments
///
/// - **`translations` \***: Translations to be used by your application. This
///   must be the same identifier used in the [`fluent_templates::static_loader!`]
///   macro, which returns [`once_cell:sync::Lazy`]`<[`StaticLoader`]>`.
/// - **`locales`**: Path to the locales folder, which must contain the translations
///   for each language in your application. Is expected to be a path relative from
///   `Cargo.toml` file. Either `locales` or `languages` is required.
/// - **`languages`**: Path to a languages file, which should an array of arrays
///   where each inner array contains a language identifier and a language name,
///   respectively. The language identifier should be a valid language tag, such as
///   `en-US`, `en`, `es-ES`, etc. Is expected to be a path relative from `Cargo.toml`
///   file. Either `locales` or `languages` is required.
///   By default, the languages file should be a JSON with a *.json* extension because
///   the `json` feature is enabled. For example:
///   ```json
///   [
///     ["en-US", "English (United States)"],
///     ["es-ES", "Español (España)"]
///   ]
///   ```
///   You can use `default-features = false` and enable the `yaml` feature to be able to
///   use a YAML file with *.yaml* or *.yml* extension. For example:
///   ```yaml
///   - - en-US
///     - English (United States)
///   - - es-ES
///     - Español (España)
///   ```
/// - **`sync_html_tag_lang`** (_`false`_): Synchronize the global [`<html lang="...">` attribute]
///   with current language using [`leptos::create_effect`]. Can be a literal boolean or an
///   expression that will be evaluated at runtime.
/// - **`url_param`** (_`"lang"`_): The parameter name to manage the language in a URL parameter.
///   Can be a literal string or an expression that will be evaluated at runtime. It will only take
///   effect on client-side.
/// - **`initial_language_from_url_param`** (_`false`_): Load the initial language of the user
///   from a URL parameter. Can be a literal boolean or an expression that will be evaluated at
///   runtime. It will only take effect on client-side.
/// - **`set_language_to_url_param`** (_`false`_): Save the language of the user to an URL parameter
///   when setting the language. Can be a literal boolean or an expression that will be evaluated at
///   runtime. It will only take effect on client-side.
/// - **`initial_language_from_url_param_to_localstorage`** (_`false`_): Save the initial language
///   of the user from the URL to [local storage]. Can be a literal boolean or an expression that will
///   be evaluated at runtime. It will only take effect on client-side.
/// - **`localstorage_key`** (_`"lang"`_): The [local storage] field to get and save the current language
///   of the user. Can be a literal string or an expression that will be evaluated at runtime.
///   It will only take effect on client-side.
/// - **`initial_language_from_localstorage`** (_`false`_): Load the initial language of the
///   user from [local storage] if not found in the URL param. Can be a literal boolean or an expression
///   that will be evaluated at runtime. It will only take effect on client-side.
///   **`set_language_to_localstorage`** (_`false`_): Save the language of the user to [local storage] if
///   when setting the language. Can be a literal boolean or an expression that will be evaluated at
///   runtime. It will only take effect on client-side.
/// - **`initial_language_from_navigator`** (_`false`_): Load the initial language of the user
///   from [`navigator.languages`] if not found in [local storage]. Can be a literal boolean or an
///   expression that will be evaluated at runtime. It will only take effect on client-side.
/// - **`initial_language_from_accept_language_header`** (_`false`_): Load the initial language of the user
///   from the `Accept-Language` header. Can be a literal boolean or an expression that will be evaluated at
///   runtime. It will only take effect on server-side.
/// - **`cookie_name`** (_`"lf-lang"`_): The cookie name to manage language in a cookie. Can be a literal string or an
///   expression that will be evaluated at runtime. It will take effect on client-side and server side.
/// - **`initial_language_from_cookie`** (_`false`_): Load the initial language of the user from a cookie.
///   Can be a literal boolean or an expression that will be evaluated at runtime. It will take effect on client-side
///   and server side.
/// - **`set_language_to_cookie`** (_`false`_): Save the language of the user to a cookie when setting the language.
///   Can be a literal boolean or an expression that will be evaluated at runtime. It will only take effect on client-side.
///
/// [`fluent_templates::static_loader!`]: https://docs.rs/fluent-templates/0.8.0/fluent_templates/macro.static_loader.html
/// [`once_cell:sync::Lazy`]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html
/// [`StaticLoader`]: https://docs.rs/fluent-templates/0.8.0/fluent_templates/struct.StaticLoader.html
/// [`<html lang="...">` attribute]: https://developer.mozilla.org/es/docs/Web/HTML/Global_attributes/lang
/// [local storage]: https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
/// [`navigator.languages`]: https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages
/// [`leptos::create_effect`]: https://docs.rs/leptos/latest/leptos/fn.create_effect.html
#[proc_macro]
pub fn leptos_fluent(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    #[allow(unused_variables)]
    let I18nLoader {
        translations_ident,
        languages,
        sync_html_tag_lang_bool,
        sync_html_tag_lang_expr,
        initial_language_from_url_param_bool,
        initial_language_from_url_param_expr,
        url_param_str,
        url_param_expr,
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
    } = parse_macro_input!(input as I18nLoader);

    let n_languages = languages.len();

    let languages_quote = format!(
        "[{}]",
        languages
            .iter()
            .map(|(id, name)| generate_code_for_static_language(id, name))
            .collect::<Vec<String>>()
            .join(",")
    )
    .parse::<TokenStream>()
    .unwrap();

    #[cfg(not(feature = "ssr"))]
    let sync_html_tag_lang_quote = {
        let effect_quote = quote! {
            use wasm_bindgen::JsCast;
            ::leptos::create_effect(move |_| ::leptos::document()
                .document_element()
                .unwrap()
                .unchecked_into::<::leptos::web_sys::HtmlElement>()
                .set_attribute(
                    "lang",
                    &::leptos_fluent::expect_i18n().language.get().id.to_string()
                )
            );
        };

        match sync_html_tag_lang_bool {
            Some(lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match sync_html_tag_lang_expr {
                Some(expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        }
    };

    #[cfg(feature = "ssr")]
    let sync_html_tag_lang_quote = quote! {};

    let url_param = match url_param_str {
        Some(lit) => quote! { #lit },
        None => match url_param_expr {
            Some(expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    let localstorage_key = match localstorage_key_str {
        Some(lit) => quote! { #lit },
        None => match localstorage_key_expr {
            Some(expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    let sync_language_with_localstorage_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key,
                    &::leptos_fluent::expect_i18n().language.get().id.to_string(),
                );
            });
        };

        match set_language_to_localstorage_bool {
            Some(lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match set_language_to_localstorage_expr {
                Some(expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        }
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_url_param_quote = {
        #[cfg(feature = "hydrate")]
        let hydrate_rerender_quote = quote! {
            ::leptos::create_effect(move |prev| {
                if prev.is_none() {
                    ::leptos_fluent::expect_i18n().language.set(l);
                }
            });
        };

        #[cfg(not(feature = "hydrate"))]
        let hydrate_rerender_quote = quote! {};

        let set_to_localstorage_quote =
            match initial_language_from_url_param_to_localstorage_bool {
                Some(lit) => match lit.value {
                    true => quote! {
                        ::leptos_fluent::localstorage::set(
                            #localstorage_key,
                            &l.id.to_string(),
                        );
                    },
                    false => quote! {},
                },
                None => {
                    match initial_language_from_url_param_to_localstorage_expr {
                        Some(expr) => quote! {
                            if #expr {
                                ::leptos_fluent::localstorage::set(
                                    #localstorage_key,
                                    &l.id.to_string(),
                                );
                            }
                        },
                        None => quote! {},
                    }
                }
            };

        let parse_language_from_url_quote = quote! {
            if let Some(l) = ::leptos_fluent::url::get(
                #url_param
            ) {
                lang = ::leptos_fluent::language_from_str_between_languages(
                    &l,
                    &LANGUAGES
                );
                if let Some(l) = lang {
                    #hydrate_rerender_quote;
                    #set_to_localstorage_quote;
                }
            }
        };

        match initial_language_from_url_param_bool {
            Some(lit) => match lit.value {
                true => parse_language_from_url_quote,
                false => quote! {},
            },
            None => match initial_language_from_url_param_expr {
                Some(expr) => quote! {
                    if #expr {
                        #parse_language_from_url_quote
                    }
                },
                None => quote! {},
            },
        }
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_localstorage_quote =
        match initial_language_from_localstorage_bool {
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        if let Some(l) = ::leptos_fluent::localstorage::get(#localstorage_key)
                        {
                            lang = ::leptos_fluent::language_from_str_between_languages(
                                &l,
                                &LANGUAGES
                            );
                        }
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_localstorage_expr {
                Some(expr) => quote! {
                    if #expr && lang.is_none() {
                        if let Some(l) = ::leptos_fluent::localstorage::get(#localstorage_key)
                        {
                            lang = ::leptos_fluent::language_from_str_between_languages(
                                &l,
                                &LANGUAGES
                            );
                        }
                    }
                },
                None => quote! {},
            },
        };

    let sync_language_with_url_param_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::url::set(
                    #url_param,
                    &::leptos_fluent::expect_i18n().language.get().id.to_string(),
                );
            });
        };

        match set_language_to_url_param_bool {
            Some(lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match set_language_to_url_param_expr {
                Some(expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        }
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_navigator_quote = {
        let window_navigator_languages_quote = quote! {
            let languages = window().navigator().languages().to_vec();
            for raw_language in languages {
                let language = raw_language.as_string();
                if language.is_none() {
                    continue;
                }
                if let Some(l) = ::leptos_fluent::language_from_str_between_languages(
                    &language.unwrap(),
                    &LANGUAGES
                ) {
                    lang = Some(l);
                    break;
                }
            }
        };

        match initial_language_from_navigator_bool {
            Some(lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #window_navigator_languages_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_navigator_expr {
                Some(expr) => quote! {
                    if #expr && lang.is_none() {
                        #window_navigator_languages_quote;
                    }
                },
                None => quote! {},
            },
        }
    };

    // Accept-Language header
    //   Actix
    #[cfg(all(feature = "actix", feature = "ssr"))]
    let initial_language_from_accept_language_header_quote = {
        let parse_actix_header_quote = quote! {
            if let Some(req) = leptos::use_context::<actix_web::HttpRequest>() {
                let maybe_header = req
                    .headers()
                    .get(actix_web::http::header::ACCEPT_LANGUAGE)
                    .and_then(|header| header.to_str().ok());

                if let Some(header) = maybe_header {
                    let langs = ::leptos_fluent::http_header::parse(header);
                    for l in langs {
                        if let Some(l) = ::leptos_fluent::language_from_str_between_languages(&l, &LANGUAGES) {
                            lang = Some(l);

                            break;
                        }
                    }
                }
            }
        };

        match initial_language_from_accept_language_header_bool {
            Some(lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_actix_header_quote
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_accept_language_header_expr {
                Some(expr) => quote! {
                    if #expr && lang.is_none() {
                        #parse_actix_header_quote;
                    }
                },
                None => quote! {},
            },
        }
    };

    //   Axum
    #[cfg(all(feature = "axum", feature = "ssr"))]
    let initial_language_from_accept_language_header_quote = {
        let parse_axum_header_quote = quote! {
            if let Some(req) = leptos::use_context::<axum::http::request::Parts>() {
                let maybe_header = req
                    .headers
                    .get(axum::http::header::ACCEPT_LANGUAGE)
                    .and_then(|header| header.to_str().ok());

                if let Some(header) = maybe_header {
                    let langs = ::leptos_fluent::http_header::parse(header);
                    for l in langs {
                        if let Some(l) = ::leptos_fluent::language_from_str_between_languages(&l, &LANGUAGES) {
                            lang = Some(l);

                            break;
                        }
                    }
                }
            }
        };

        match initial_language_from_accept_language_header_bool {
            Some(lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_axum_header_quote
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_accept_language_header_expr {
                Some(expr) => quote! {
                    if #expr && lang.is_none() {
                        #parse_axum_header_quote;
                    }
                },
                None => quote! {},
            },
        }
    };

    //   Other SSR frameworks or the user is not using any
    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let initial_language_from_accept_language_header_quote = quote! {};

    // Cookie
    let cookie_name = match cookie_name_str {
        Some(lit) => quote! { #lit },
        None => match cookie_name_expr {
            Some(expr) => quote! { #expr },
            None => quote! { "lf-lang" },
        },
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_cookie_quote = {
        let parse_client_cookie_quote = quote! {
            if let Some(cookie) = ::leptos_fluent::cookie::get(#cookie_name) {
                if let Some(l) = ::leptos_fluent::language_from_str_between_languages(&cookie, &LANGUAGES) {
                    lang = Some(l);
                }
            }
        };

        match initial_language_from_cookie_bool {
            Some(lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_client_cookie_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_cookie_expr {
                Some(expr) => quote! {
                    if #expr && lang.is_none() {
                        #parse_client_cookie_quote;
                    }
                },
                None => quote! {},
            },
        }
    };

    #[cfg(not(feature = "ssr"))]
    let sync_language_with_cookie_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::cookie::set(
                    #cookie_name,
                    &::leptos_fluent::expect_i18n().language.get().id.to_string(),
                );
            });
        };

        match set_language_to_cookie_bool {
            Some(lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match set_language_to_cookie_expr {
                Some(expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        }
    };

    //   Actix
    #[cfg(all(feature = "ssr", feature = "actix"))]
    let initial_language_from_cookie_quote = {
        let parse_actix_cookie_quote = quote! {
            if let Some(req) = leptos::use_context::<actix_web::HttpRequest>() {
                let maybe_cookie = req
                    .cookie(#cookie_name)
                    .and_then(|cookie| Some(cookie.value().to_string()));

                if let Some(cookie) = maybe_cookie {
                    if let Some(l) = ::leptos_fluent::language_from_str_between_languages(&cookie, &LANGUAGES) {
                        lang = Some(l);
                    }
                }
            }
        };

        match initial_language_from_cookie_bool {
            Some(lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_actix_cookie_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_cookie_expr {
                Some(expr) => quote! {
                    if #expr && lang.is_none() {
                        #parse_actix_cookie_quote;
                    }
                },
                None => quote! {},
            },
        }
    };

    //     TODO: Set in Set-Cookie header?
    #[cfg(all(feature = "ssr", feature = "actix"))]
    let sync_language_with_cookie_quote = quote! {};

    //   Axum
    #[cfg(all(feature = "ssr", feature = "axum"))]
    let initial_language_from_cookie_quote = {
        let parse_axum_cookie_quote = quote! {
            if let Some(req) = leptos::use_context::<axum::http::request::Parts>() {
                let maybe_cookie = req
                    .headers
                    .get(axum::http::header::COOKIE)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|cookie| {
                        let cookie = cookie.split(';').find(|c| c.starts_with(#cookie_name));
                        cookie.map(|c| c.split('=').nth(1).unwrap().to_string())
                    });

                if let Some(cookie) = maybe_cookie {
                    if let Some(l) = ::leptos_fluent::language_from_str_between_languages(&cookie, &LANGUAGES) {
                        lang = Some(l);
                    }
                }
            }
        };

        match initial_language_from_cookie_bool {
            Some(lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_axum_cookie_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_cookie_expr {
                Some(expr) => quote! {
                    if #expr && lang.is_none() {
                        #parse_axum_cookie_quote;
                    }
                },
                None => quote! {},
            },
        }
    };

    //     TODO: Set in Set-Cookie header?
    #[cfg(all(feature = "ssr", feature = "axum"))]
    let sync_language_with_cookie_quote = quote! {};

    //   Other SSR frameworks or the user is not using any
    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let initial_language_from_cookie_quote = quote! {};
    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let sync_language_with_cookie_quote = quote! {};

    let initial_language_quote = {
        #[cfg(not(feature = "ssr"))]
        quote! {
            #initial_language_from_url_param_quote
            #initial_language_from_cookie_quote
            #initial_language_from_localstorage_quote
            #initial_language_from_navigator_quote
        }

        #[cfg(feature = "ssr")]
        quote! {
            #initial_language_from_cookie_quote
            #initial_language_from_accept_language_header_quote
        }
    };

    let quote = quote! {
        {
            const LANGUAGES: [&::leptos_fluent::Language; #n_languages] = #languages_quote;

            let mut lang: Option<&'static ::leptos_fluent::Language> = None;
            #initial_language_quote;

            let initial_lang = if let Some(l) = lang {
                l
            } else {
                LANGUAGES[0]
            };

            let mut i18n = ::leptos_fluent::I18n {
                language: ::leptos::create_rw_signal(initial_lang),
                languages: &LANGUAGES,
                translations: &#translations_ident,
            };
            provide_context::<::leptos_fluent::I18n>(i18n);
            #sync_html_tag_lang_quote
            #sync_language_with_localstorage_quote
            #sync_language_with_url_param_quote
            #sync_language_with_cookie_quote

            i18n
        }
    };

    //println!("{}", quote);
    proc_macro::TokenStream::from(quote)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_main_and_macros_package_versions_match() {
        let macros_cargo_toml = include_str!("../Cargo.toml");
        let main_cargo_toml = include_str!("../../leptos-fluent/Cargo.toml");

        let get_version = move |content: &str| -> Option<String> {
            let mut version = None;
            for line in content.lines() {
                if line.starts_with("version = ") {
                    version = Some(
                        line.split("version = \"")
                            .nth(1)
                            .unwrap()
                            .split('"')
                            .next()
                            .unwrap()
                            .to_string(),
                    );
                    break;
                }
            }
            version
        };

        let macros_version = get_version(macros_cargo_toml);
        let main_version = get_version(main_cargo_toml);
        assert!(
            macros_version.is_some(),
            "leptos-fluent-macros version not found in Cargo.toml"
        );
        assert!(
            main_version.is_some(),
            "leptos-fluent version not found in Cargo.toml"
        );
        assert_eq!(
            macros_version.unwrap(),
            main_version.unwrap(),
            "leptos-fluent-macros and leptos-fluent versions do not match"
        );
    }
}
