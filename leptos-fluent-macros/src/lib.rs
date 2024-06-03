#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Macros for the leptos-fluent crate.
//!
//! See [leptos-fluent] for more information.
//!
//! [leptos-fluent]: https://crates.io/crates/leptos-fluent

extern crate proc_macro;

mod languages;
mod loader;
#[cfg(not(feature = "ssr"))]
mod translations_checker;

use languages::generate_code_for_static_language;
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
///   `Cargo.toml` file.
/// - **`check_translations`** (experimental): Path to the files to check if all
///    translations are being used and their placeholders are correct. Is expected
///   to be a glob pattern relative from `Cargo.toml` file. Tipically, you should use
///   `"./src/**/*.rs"` or something like `"../{app,components}/src/**/*.rs"`.
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
        languages_path,
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
        fluent_resources,
    } = syn::parse_macro_input!(input as I18nLoader);

    let n_languages = languages.len();
    let languages_quote = format!(
        "[{}]",
        languages
            .iter()
            .map(|(id, name)| generate_code_for_static_language(id, name))
            .collect::<Vec<String>>()
            .join(",")
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap();

    // locales tracker
    let mut locales_tracker = "{".to_string();
    for (lang, (paths, _)) in fluent_resources.iter() {
        locales_tracker
            .push_str(&format!("let {} = vec![", lang.replace('-', "_")));
        for path in paths {
            locales_tracker.push_str(&format!(
                "include_bytes!(\"{}\"),",
                &path.replace('\\', "\\\\").replace('"', "\\\"")
            ));
        }
        locales_tracker.push_str("];");
        if let Some(languages_file_path) = &languages_path {
            locales_tracker.push_str(&format!(
                "let languages_path = include_bytes!(\"{}\");",
                &languages_file_path
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
            ));
        }
    }
    locales_tracker.push_str("};");
    let locales_tracker_quote =
        locales_tracker.parse::<proc_macro2::TokenStream>().unwrap();

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
            #locales_tracker_quote

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
