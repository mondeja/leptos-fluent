#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Macros for the leptos-fluent crate.
//!
//! See [leptos-fluent] for more information.
//!
//! [leptos-fluent]: https://crates.io/crates/leptos-fluent

extern crate proc_macro;

pub(crate) mod cookie;
mod files_tracker;
mod fluent_resources;
mod languages;
mod loader;
#[cfg(not(feature = "ssr"))]
mod translations_checker;

use files_tracker::build_files_tracker_quote;
#[cfg(not(feature = "ssr"))]
pub(crate) use fluent_resources::FluentResources;
pub(crate) use fluent_resources::{
    build_fluent_resources_and_file_paths, FluentFilePaths,
};
use languages::build_languages_quote;
use loader::I18nLoader;
use quote::quote;

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
///         translations: [TRANSLATIONS],
///         languages: "./locales/languages.json",
///         sync_html_tag_lang: true,
///         sync_html_tag_dir: true,
///         url_param: "lang",
///         initial_language_from_url_param: true,
///         initial_language_from_url_param_to_localstorage: true,
///         initial_language_from_url_param_to_cookie: true,
///         set_language_to_url_param: true,
///         localstorage_key: "language",
///         initial_language_from_localstorage: true,
///         set_language_to_localstorage: true,
///         initial_language_from_navigator: true,
///         initial_language_from_accept_language_header: true,
///         cookie_name: "lang",
///         cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=2592000",
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
///   `Cargo.toml` file, the same used in the [`fluent_templates::static_loader!`]
///   macro.
/// - **`core_locales`**: Path to the core locales file, which must contain a shared
///   translation for all languages. Is expected to be a path relative from `Cargo.toml`,
///   the same used in the [`fluent_templates::static_loader!`] macro.
/// - **`check_translations`**: Path to the files to check if all translations are
///   being used and their placeholders are correct. Is expected to be a glob pattern
///   relative from `Cargo.toml` file. Tipically, you should use `"./src/**/*.rs"` for
///   a single crate or something like `"../{app,components}/src/**/*.rs"` to match
///   multiple crates in a workspace.
/// - **`languages`**: Path to a languages file, which should an array of arrays
///   where each inner array contains a language identifier and a language name,
///   respectively. The language identifier should be a valid language tag, such as
///   `en-US`, `en`, `es-ES`, etc. Is expected to be a path relative from `Cargo.toml`
///   file.
///   By default, the languages file should be a JSON with a *.json* extension because
///   the `json` feature is enabled. For example:
///   ```json
///   [
///     ["en-US", "English (United States)"],
///     ["es-ES", "Español (España)"]
///   ]
///   ```
///   You can set `default-features = false` and enable the `yaml` or the `json5` feature
///   to be able to use a YAML or JSON5 file. For example:
///   ```yaml
///   # locales/languages.yaml
///   - - en-US
///     - English (United States)
///   - - es-ES
///     - Español (España)
///   ```
///   ```json5
///   // locales/languages.json5
///   [
///     ["en-US", "English (United States)"],
///     ["es-ES", "Español (España)"]
///   ]
///   ```
///   You can define a third element in the inner array with the direction of the language,
///   to use it in the [`<html dir="...">` attribute] (see `sync_html_tag_dir`). For example:
///   ```json
///   [
///     ["en-US", "English (United States)", "ltr"],
///     ["es-ES", "Español (España)", "auto"],
///     ["ar", "العربية", "rtl"],
///     ["it", "Italiano"]
///   ]
///   ```
/// - **`sync_html_tag_lang`** (_`false`_): Synchronize the global [`<html lang="...">` attribute]
///   with current language using [`leptos::create_effect`]. Can be a literal boolean or an
///   expression that will be evaluated at runtime.
/// - **`sync_html_tag_dir`** (_`false`_): Synchronize the global [`<html dir="...">` attribute]
///   with current language using [`leptos::create_effect`]. Can be a literal boolean or an
///   expression that will be evaluated at runtime. For custom languages from a languages file,
///   you can specify a third element in the inner array with the direction of the language,
///   which can be `"auto"`, `"ltr"`, or `"rtl"`. For automatic languages will be defined depending
///   on the language. For example, Arabic will be `"rtl"`, English will be `"ltr"` and Japanese
///   will be `"auto"`.
/// - **`url_param`** (_`"lang"`_): The parameter name to manage the language in a URL parameter.
///   Can be a literal string or an expression that will be evaluated at runtime. It will take effect
///   on client-side and server side.
/// - **`initial_language_from_url_param`** (_`false`_): Load the initial language of the user
///   from a URL parameter. Can be a literal boolean or an expression that will be evaluated at
///   runtime. It will take effect on client-side and server side.
/// - **`set_language_to_url_param`** (_`false`_): Save the language of the user to an URL parameter
///   when setting the language. Can be a literal boolean or an expression that will be evaluated at
///   runtime. It will only take effect on client-side.
/// - **`initial_language_from_url_param_to_localstorage`** (_`false`_): Save the initial language
///   of the user from the URL to [local storage]. Can be a literal boolean or an expression that will
///   be evaluated at runtime. It will only take effect on client-side.
/// - **`initial_language_from_url_param_to_cookie`** (_`false`_): Save the initial language of the user
///   from the URL to a cookie. Can be a literal boolean or an expression that will be evaluated at runtime.
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
/// - **`cookie_attrs`** (_`""`_): The [attributes][cookie-attributes] to set in the cookie. Can be a literal string or an expression
///   that will be evaluated at runtime. For example, `"SameSite=Strict; Secure; path=/; max-age=600"`.
///   It will take effect on client-side.
/// - **`initial_language_from_cookie`** (_`false`_): Load the initial language of the user from a cookie.
///   Can be a literal boolean or an expression that will be evaluated at runtime. It will take effect on client-side
///   and server side.
/// - **`initial_language_from_cookie_to_localstorage`** (_`false`_): Save the initial language of the user
///   from the cookie to [local storage]. Can be a literal boolean or an expression that will be evaluated at runtime.
/// - **`set_language_to_cookie`** (_`false`_): Save the language of the user to a cookie when setting the language.
///   Can be a literal boolean or an expression that will be evaluated at runtime. It will only take effect on client-side.
///
/// [`fluent_templates::static_loader!`]: https://docs.rs/fluent-templates/latest/fluent_templates/macro.static_loader.html
/// [`once_cell:sync::Lazy`]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html
/// [`StaticLoader`]: https://docs.rs/fluent-templates/latest/fluent_templates/struct.StaticLoader.html
/// [`<html lang="...">` attribute]: https://developer.mozilla.org/es/docs/Web/HTML/Global_attributes/lang
/// [`<html dir="...">` attribute]: https://developer.mozilla.org/es/docs/Web/HTML/Global_attributes/dir
/// [local storage]: https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
/// [`navigator.languages`]: https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages
/// [`leptos::create_effect`]: https://docs.rs/leptos/latest/leptos/fn.create_effect.html
/// [cookie-attributes]: https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie#write_a_new_cookie
#[proc_macro]
pub fn leptos_fluent(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let I18nLoader {
        translations,
        languages,
        languages_path,
        sync_html_tag_lang_bool,
        sync_html_tag_lang_expr,
        sync_html_tag_dir_bool,
        sync_html_tag_dir_expr,
        initial_language_from_url_param_bool,
        initial_language_from_url_param_expr,
        url_param_str,
        url_param_expr,
        initial_language_from_url_param_to_localstorage_bool,
        initial_language_from_url_param_to_localstorage_expr,
        initial_language_from_url_param_to_cookie_bool,
        initial_language_from_url_param_to_cookie_expr,
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
        cookie_attrs_str,
        cookie_attrs_expr,
        initial_language_from_cookie_bool,
        initial_language_from_cookie_expr,
        initial_language_from_cookie_to_localstorage_bool,
        initial_language_from_cookie_to_localstorage_expr,
        set_language_to_cookie_bool,
        set_language_to_cookie_expr,
        fluent_file_paths,
        core_locales_path,
    } = syn::parse_macro_input!(input as I18nLoader);

    let n_languages = languages.len();
    let languages_quote = build_languages_quote(&languages);

    // files tracker
    let files_tracker_quote = build_files_tracker_quote(
        &fluent_file_paths,
        &languages_path,
        &core_locales_path,
    );

    #[cfg(not(feature = "ssr"))]
    let sync_html_tag_lang_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                use wasm_bindgen::JsCast;
                _ = ::leptos::document()
                    .document_element()
                    .unwrap()
                    .unchecked_into::<::leptos::web_sys::HtmlElement>()
                    .set_attribute(
                        "lang",
                        &::leptos_fluent::expect_i18n().language.get().id.to_string()
                    );
            });
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
    #[cfg(feature = "ssr")]
    {
        _ = sync_html_tag_lang_bool;
        _ = sync_html_tag_lang_expr;
    }

    #[cfg(not(feature = "ssr"))]
    let sync_html_tag_dir_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                use wasm_bindgen::JsCast;
                _ = ::leptos::document()
                    .document_element()
                    .unwrap()
                    .unchecked_into::<::leptos::web_sys::HtmlElement>()
                    .set_attribute(
                        "dir",
                        ::leptos_fluent::expect_i18n().language.get().dir.as_str(),
                    );
            });
        };

        match sync_html_tag_dir_bool {
            Some(lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match sync_html_tag_dir_expr {
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
    let sync_html_tag_dir_quote = quote! {};
    #[cfg(feature = "ssr")]
    {
        _ = sync_html_tag_dir_bool;
        _ = sync_html_tag_dir_expr;
    }

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

    #[cfg(any(
        not(feature = "ssr"),
        all(feature = "ssr", feature = "actix"),
        all(feature = "ssr", feature = "axum")
    ))]
    let cookie_name = match cookie_name_str {
        Some(lit) => quote! { #lit },
        None => match cookie_name_expr {
            Some(expr) => quote! { #expr },
            None => quote! { "lf-lang" },
        },
    };

    #[cfg(not(feature = "ssr"))]
    let cookie_attrs = match cookie_attrs_str {
        Some(lit) => quote! { #lit },
        None => match cookie_attrs_expr {
            Some(expr) => quote! { #expr },
            None => quote! { "" },
        },
    };

    let initial_language_from_url_param_quote = {
        #[cfg(feature = "hydrate")]
        let hydrate_rerender_quote = quote! {
            ::leptos::create_effect(move |prev| {
                if prev.is_none() {
                    l.activate();
                }
            });
        };

        #[cfg(all(not(feature = "hydrate"), not(feature = "ssr")))]
        let hydrate_rerender_quote = quote! {};

        #[cfg(not(feature = "ssr"))]
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

        #[cfg(not(feature = "ssr"))]
        let set_to_cookie_quote =
            match initial_language_from_url_param_to_cookie_bool {
                Some(lit) => match lit.value {
                    true => quote! {
                        ::leptos_fluent::cookie::set(
                            #cookie_name,
                            &l.id.to_string(),
                            &#cookie_attrs,
                        );
                    },
                    false => quote! {},
                },
                None => match initial_language_from_url_param_to_cookie_expr {
                    Some(expr) => quote! {
                        if #expr {
                            ::leptos_fluent::cookie::set(
                                #cookie_name,
                                &l.id.to_string(),
                                &#cookie_attrs,
                            );
                        }
                    },
                    None => quote! {},
                },
            };

        #[cfg(feature = "ssr")]
        {
            _ = initial_language_from_url_param_to_localstorage_bool;
            _ = initial_language_from_url_param_to_localstorage_expr;
            _ = initial_language_from_url_param_to_cookie_bool;
            _ = initial_language_from_url_param_to_cookie_expr;
        }

        #[cfg(not(feature = "ssr"))]
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
                    #set_to_cookie_quote;
                }
            }
        };

        #[cfg(all(feature = "ssr", feature = "actix"))]
        let parse_language_from_url_quote = quote! {
            if let Some(req) = leptos::use_context::<actix_web::HttpRequest>() {
                let uri_query = req.uri().query().unwrap_or("");
                let mut maybe_lang = None;
                for (key, value) in uri_query.split('&').map(|pair| {
                    let mut split = pair.splitn(2, '=');
                    (split.next().unwrap_or(""), split.next().unwrap_or(""))
                }) {
                    if key == #url_param {
                        maybe_lang = Some(value);
                        break;
                    }
                }

                if let Some(l) = maybe_lang {
                    lang = ::leptos_fluent::language_from_str_between_languages(
                        &l,
                        &LANGUAGES
                    );
                }
            }
        };

        #[cfg(all(feature = "ssr", feature = "axum"))]
        let parse_language_from_url_quote = quote! {
            if let Some(req) = leptos::use_context::<axum::http::request::Parts>() {
                let uri_query = req.uri.query().unwrap_or("");
                let mut maybe_lang = None;
                for (key, value) in uri_query.split('&').map(|pair| {
                    let mut split = pair.splitn(2, '=');
                    (split.next().unwrap_or(""), split.next().unwrap_or(""))
                }) {
                    if key == #url_param {
                        maybe_lang = Some(value);
                        break;
                    }
                }

                if let Some(l) = maybe_lang {
                    lang = ::leptos_fluent::language_from_str_between_languages(
                        &l,
                        &LANGUAGES
                    );
                }
            }
        };

        // Other SSR framework or the user is not using any
        #[cfg(all(
            not(feature = "actix"),
            not(feature = "axum"),
            feature = "ssr"
        ))]
        let parse_language_from_url_quote = quote! {};

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

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_localstorage_bool;
        _ = initial_language_from_localstorage_expr;
    }

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

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_navigator_bool;
        _ = initial_language_from_navigator_expr;
    }

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
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_axum_header_quote
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_accept_language_header_expr {
                Some(ref expr) => quote! {
                    if #expr && lang.is_none() {
                        #parse_axum_header_quote;
                    }
                },
                None => quote! {},
            },
        }
    };

    //   Other SSR framework or the user is not using any
    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let initial_language_from_accept_language_header_quote = quote! {};

    // No SSR
    #[cfg(not(feature = "ssr"))]
    {
        _ = initial_language_from_accept_language_header_bool;
        _ = initial_language_from_accept_language_header_expr;
    }

    // Cookie
    #[cfg(not(feature = "ssr"))]
    let initial_language_from_cookie_quote = {
        let initial_language_from_cookie_to_localstorage_quote = {
            let effect_quote = quote! {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key,
                    &l.id.to_string(),
                );
            };

            match initial_language_from_cookie_to_localstorage_bool {
                Some(lit) => match lit.value {
                    true => effect_quote,
                    false => quote! {},
                },
                None => match initial_language_from_cookie_to_localstorage_expr
                {
                    Some(expr) => quote! {
                        if #expr {
                            #effect_quote
                        }
                    },
                    None => quote! {},
                },
            }
        };

        let parse_client_cookie_quote = quote! {
            if let Some(cookie) = ::leptos_fluent::cookie::get(#cookie_name) {
                if let Some(l) = ::leptos_fluent::language_from_str_between_languages(&cookie, &LANGUAGES) {
                    lang = Some(l);

                    #initial_language_from_cookie_to_localstorage_quote;
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

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_cookie_to_localstorage_bool;
        _ = initial_language_from_cookie_to_localstorage_expr;
    }

    #[cfg(not(feature = "ssr"))]
    let sync_language_with_cookie_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::cookie::set(
                    #cookie_name,
                    &::leptos_fluent::expect_i18n().language.get().id.to_string(),
                    &#cookie_attrs,
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

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_cookie_bool;
        _ = initial_language_from_cookie_expr;
        _ = cookie_attrs_str;
        _ = cookie_attrs_expr;
        _ = set_language_to_cookie_bool;
        _ = set_language_to_cookie_expr;
    }

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
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_axum_cookie_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_cookie_expr {
                Some(ref expr) => quote! {
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
            #initial_language_from_url_param_quote
            #initial_language_from_cookie_quote
            #initial_language_from_accept_language_header_quote
        }
    };

    let translations = {
        let loader::Translations { simple, compound } = translations;

        let quote = quote! {{
            let mut all_loaders = Vec::new();
            all_loaders.extend([#(& #simple),*]);
            #(
                all_loaders.extend(#compound.iter());
            );*

            all_loaders
        }};

        quote
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
                translations: ::leptos::Signal::derive(|| #translations),
            };
            provide_context::<::leptos_fluent::I18n>(i18n);
            #sync_html_tag_lang_quote
            #sync_html_tag_dir_quote
            #sync_language_with_localstorage_quote
            #sync_language_with_url_param_quote
            #sync_language_with_cookie_quote
            #files_tracker_quote

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
        // cargo-readme does not allow to use `version.workspace = true`,
        // see https://github.com/webern/cargo-readme/issues/81

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
