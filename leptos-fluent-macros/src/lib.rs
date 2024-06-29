#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(track_path))]

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
/// [Reference](https://mondeja.github.io/leptos-fluent/leptos_fluent.html)
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
///         initial_language_from_localstorage_to_cookie: true,
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
/// See the reference with all the parameters explained in detail at
/// https://mondeja.github.io/leptos-fluent/leptos_fluent.html
#[proc_macro]
pub fn leptos_fluent(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let I18nLoader {
        fluent_file_paths,
        translations,
        languages,
        languages_path,
        raw_languages_path,
        locales_path,
        core_locales_path,
        check_translations,
        provide_meta_context,
        provide_meta_context_exprpath,
        sync_html_tag_lang_bool,
        sync_html_tag_lang_expr,
        sync_html_tag_lang_exprpath,
        sync_html_tag_dir_bool,
        sync_html_tag_dir_expr,
        sync_html_tag_dir_exprpath,
        initial_language_from_url_param_bool,
        initial_language_from_url_param_expr,
        initial_language_from_url_param_exprpath,
        url_param_str,
        url_param_expr,
        url_param_exprpath,
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
    } = syn::parse_macro_input!(input as I18nLoader);

    let n_languages = languages.len();
    let languages_quote = build_languages_quote(&languages);

    // files tracker
    let files_tracker_quote = build_files_tracker_quote(
        &fluent_file_paths,
        &languages_path,
        &core_locales_path,
    );

    // Less code possible on nightly
    #[cfg(feature = "nightly")]
    let get_language_quote = quote! {
        (::leptos_fluent::i18n())()
    };

    #[cfg(not(feature = "nightly"))]
    let get_language_quote = quote! {
        ::leptos_fluent::i18n().language.get()
    };

    #[cfg(feature = "system")]
    let data_file_key = match data_file_key_str {
        Some(ref lit) => match data_file_key_exprpath {
            Some(path) => quote! { #path{#lit} },
            None => quote! { #lit },
        },
        None => match data_file_key_expr {
            Some(ref expr) => match data_file_key_exprpath {
                Some(path) => quote! { #path{#expr} },
                None => quote! { #expr },
            },
            None => quote! { "leptos-fluent" },
        },
    };

    // discover from system language (desktop apps)
    #[cfg(all(feature = "system", not(feature = "ssr")))]
    let initial_language_from_system_quote = {
        let initial_language_from_system_to_data_file_quote = {
            let quote = match initial_language_from_system_to_data_file_bool {
                Some(ref lit) => match lit.value {
                    true => quote! {
                       if lang.is_none() && !#data_file_key.is_empty() {
                           ::leptos_fluent::data_file::set(
                               #data_file_key,
                               &l.id.to_string(),
                           );
                       }
                    },
                    false => quote! {},
                },
                None => match initial_language_from_system_to_data_file_expr {
                    Some(ref expr) => quote! {
                        if lang.is_none() && #expr && !#data_file_key.is_empty() {
                            ::leptos_fluent::data_file::set(
                                #data_file_key,
                                &l.id.to_string(),
                            );
                        }
                    },
                    None => quote! {},
                },
            };

            match initial_language_from_system_to_data_file_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        let effect_quote = quote! {
            if let Ok(l) = ::leptos_fluent::current_locale() {
                lang = ::leptos_fluent::l(
                    &l,
                    &LANGUAGES
                );

                if let Some(l) = lang {
                    #initial_language_from_system_to_data_file_quote
                }
            }
        };

        let quote = match initial_language_from_system_bool {
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #effect_quote
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_system_expr {
                Some(ref expr) => quote! {
                    if #expr && lang.is_none() {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        };

        match initial_language_from_system_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(all(not(feature = "system"), not(feature = "ssr")))]
    let initial_language_from_system_quote = quote! {};

    #[cfg(all(feature = "system", feature = "ssr"))]
    {
        _ = data_file_key_exprpath;
        _ = initial_language_from_system_bool;
        _ = initial_language_from_system_expr;
        _ = initial_language_from_system_exprpath;
        _ = initial_language_from_system_to_data_file_bool;
        _ = initial_language_from_system_to_data_file_expr;
        _ = initial_language_from_system_to_data_file_exprpath;
    }

    #[cfg(feature = "system")]
    let sync_language_with_data_file_quote = {
        let set_language_to_data_file_quote = {
            let quote = match set_language_to_data_file_bool {
                Some(ref lit) => match lit.value {
                    true => quote! { #data_file_key },
                    false => quote! { "" },
                },
                None => match set_language_to_data_file_expr {
                    Some(ref expr) => quote! {
                        if #expr {
                            #data_file_key
                        } else {
                            ""
                        }
                    },
                    None => quote! { "" },
                },
            };

            match set_language_to_data_file_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                if #set_language_to_data_file_quote.is_empty() {
                    return;
                }
                ::leptos_fluent::data_file::set(
                    #set_language_to_data_file_quote,
                    &#get_language_quote.id.to_string(),
                );
            });
        };

        match set_language_to_data_file_bool {
            Some(ref lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match set_language_to_data_file_expr {
                Some(ref expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        }
    };

    #[cfg(not(feature = "system"))]
    let sync_language_with_data_file_quote = quote! {};

    #[cfg(all(feature = "system", not(feature = "ssr")))]
    let initial_language_from_data_file_quote = {
        let initial_language_from_data_file_quote = {
            let quote = match initial_language_from_data_file_bool {
                Some(ref lit) => match lit.value() {
                    true => quote! { #data_file_key },
                    false => quote! { "" },
                },
                None => match initial_language_from_data_file_expr {
                    Some(ref expr) => quote! {
                        if #expr {
                            #data_file_key
                        } else {
                            ""
                        }
                    },
                    None => quote! { "" },
                },
            };

            match initial_language_from_data_file_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        let effect_quote = quote! {
            if #initial_language_from_data_file_quote.is_empty() {
                return;
            }
            if let Some(l) = ::leptos_fluent::data_file::get(
                #initial_language_from_data_file_quote
            ) {
                lang = ::leptos_fluent::l(
                    &l,
                    &LANGUAGES
                );
            }
        };

        match initial_language_from_data_file_bool {
            Some(ref lit) => match lit.value() {
                true => quote! {
                    if lang.is_none() {
                        #effect_quote
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_data_file_expr {
                Some(ref expr) => quote! {
                    if #expr && lang.is_none() {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        }
    };

    #[cfg(all(not(feature = "system"), not(feature = "ssr")))]
    let initial_language_from_data_file_quote = quote! {};

    #[cfg(all(feature = "system", feature = "ssr"))]
    {
        _ = initial_language_from_data_file_bool;
        _ = initial_language_from_data_file_expr;
        _ = initial_language_from_data_file_exprpath;
    }

    let initial_language_from_server_function_quote = {
        let effect_quote = quote! {
            spawn_local(async move {
                let lang_result = #initial_language_from_server_function().await;
                if let Ok(maybe_lang) = lang_result {
                    if let Some(l) = maybe_lang {
                        lang = ::leptos_fluent::l(
                            &l,
                            &LANGUAGES
                        );
                    }
                }
            });
        };

        let quote = match initial_language_from_server_function {
            Some(_) => quote! {
                if lang.is_none() {
                    #effect_quote
                }
            },
            None => quote! {},
        };

        match initial_language_from_server_function_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    let sync_language_with_server_function_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                spawn_local(async {
                    _ = #set_language_to_server_function(
                        #get_language_quote.id.to_string()
                    )
                    .await;
                });
            });
        };

        let quote = match set_language_to_server_function {
            Some(_) => effect_quote,
            None => quote! {},
        };

        match set_language_to_server_function_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(not(feature = "ssr"))]
    let sync_html_tag_lang_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                use wasm_bindgen::JsCast;
                _ = ::leptos::document()
                    .document_element()
                    .unwrap()
                    .unchecked_into::<::leptos_fluent::web_sys::HtmlElement>()
                    .set_attribute(
                        "lang",
                        &#get_language_quote.id.to_string()
                    );
            });
        };

        let quote = match sync_html_tag_lang_bool {
            Some(ref lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match sync_html_tag_lang_expr {
                Some(ref expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        };

        match sync_html_tag_lang_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(feature = "ssr")]
    let sync_html_tag_lang_quote = quote! {};
    #[cfg(feature = "ssr")]
    {
        _ = sync_html_tag_lang_bool;
        _ = sync_html_tag_lang_expr;
        _ = sync_html_tag_lang_exprpath;
    }

    #[cfg(not(feature = "ssr"))]
    let sync_html_tag_dir_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                use wasm_bindgen::JsCast;
                _ = ::leptos::document()
                    .document_element()
                    .unwrap()
                    .unchecked_into::<::leptos_fluent::web_sys::HtmlElement>()
                    .set_attribute(
                        "dir",
                        &#get_language_quote.dir.as_str(),
                    );
            });
        };

        let quote = match sync_html_tag_dir_bool {
            Some(ref lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match sync_html_tag_dir_expr {
                Some(ref expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        };

        match sync_html_tag_dir_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(feature = "ssr")]
    let sync_html_tag_dir_quote = quote! {};
    #[cfg(feature = "ssr")]
    {
        _ = sync_html_tag_dir_bool;
        _ = sync_html_tag_dir_expr;
        _ = sync_html_tag_dir_exprpath;
    }

    let url_param = match url_param_str {
        Some(ref lit) => match url_param_exprpath {
            Some(path) => quote! { #path{#lit} },
            None => quote! { #lit },
        },
        None => match url_param_expr {
            Some(ref expr) => match url_param_exprpath {
                Some(path) => quote! { #path{#expr} },
                None => quote! { #expr },
            },
            None => quote! { "lang" },
        },
    };

    let localstorage_key = match localstorage_key_str {
        Some(ref lit) => quote! { #lit },
        None => match localstorage_key_expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    let sync_language_with_localstorage_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key,
                    &#get_language_quote.id.to_string(),
                );
            });
        };

        let quote = match set_language_to_localstorage_bool {
            Some(ref lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match set_language_to_localstorage_expr {
                Some(ref expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        };

        match set_language_to_localstorage_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(any(
        not(feature = "ssr"),
        all(feature = "ssr", feature = "actix"),
        all(feature = "ssr", feature = "axum")
    ))]
    let cookie_name = match cookie_name_str {
        Some(ref lit) => match cookie_name_exprpath {
            Some(path) => quote! { #path{#lit} },
            None => quote! { #lit },
        },
        None => match cookie_name_expr {
            Some(ref expr) => match cookie_name_exprpath {
                Some(path) => quote! { #path{#expr} },
                None => quote! { #expr },
            },
            None => quote! { "lf-lang" },
        },
    };

    #[cfg(not(feature = "ssr"))]
    let cookie_attrs = match cookie_attrs_str {
        Some(ref lit) => match cookie_attrs_exprpath {
            Some(path) => quote! { #path{#lit} },
            None => quote! { #lit },
        },
        None => match cookie_attrs_expr {
            Some(ref expr) => match cookie_attrs_exprpath {
                Some(path) => quote! { #path{#expr} },
                None => quote! { #expr },
            },
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
        let set_to_localstorage_quote = {
            let effect_quote = quote! {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key,
                    &l.id.to_string(),
                );
            };

            let quote = match initial_language_from_url_param_to_localstorage_bool {
                Some(ref lit) => match lit.value {
                    true => effect_quote,
                    false => quote! {},
                },
                None => {
                    match initial_language_from_url_param_to_localstorage_expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #effect_quote
                            }
                        },
                        None => quote! {},
                    }
                }
            };

            match initial_language_from_url_param_to_localstorage_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        #[cfg(not(feature = "ssr"))]
        let set_to_cookie_quote = {
            let effect_quote = quote! {
                ::leptos_fluent::cookie::set(
                    #cookie_name,
                    &l.id.to_string(),
                    &#cookie_attrs,
                );
            };

            let quote = match initial_language_from_url_param_to_cookie_bool {
                Some(ref lit) => match lit.value {
                    true => effect_quote,
                    false => quote! {},
                },
                None => match initial_language_from_url_param_to_cookie_expr {
                    Some(ref expr) => quote! {
                        if #expr {
                            #effect_quote
                        }
                    },
                    None => quote! {},
                },
            };

            match initial_language_from_url_param_to_cookie_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        #[cfg(feature = "ssr")]
        {
            _ = initial_language_from_url_param_to_localstorage_bool;
            _ = initial_language_from_url_param_to_localstorage_expr;
            _ = initial_language_from_url_param_to_localstorage_exprpath;
            _ = initial_language_from_url_param_to_cookie_bool;
            _ = initial_language_from_url_param_to_cookie_expr;
            _ = initial_language_from_url_param_to_cookie_exprpath;
        }

        #[cfg(not(feature = "ssr"))]
        let parse_language_from_url_quote = quote! {
            if let Some(l) = ::leptos_fluent::url::get(
                #url_param
            ) {
                lang = ::leptos_fluent::l(
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
                    lang = ::leptos_fluent::l(
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
                    lang = ::leptos_fluent::l(
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

        let quote = match initial_language_from_url_param_bool {
            Some(ref lit) => match lit.value {
                true => parse_language_from_url_quote,
                false => quote! {},
            },
            None => match initial_language_from_url_param_expr {
                Some(ref expr) => {
                    match parse_language_from_url_quote.is_empty() {
                        true => quote! {},
                        false => quote! {
                            if #expr {
                                #parse_language_from_url_quote
                            }
                        },
                    }
                }
                None => quote! {},
            },
        };

        match initial_language_from_url_param_exprpath {
            Some(ref path) => {
                quote! {
                    #path{#quote}
                }
            }
            None => quote,
        }
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_localstorage_quote = {
        let set_cookie_quote = quote! {
            ::leptos_fluent::cookie::set(
                #cookie_name,
                &l.id.to_string(),
                &#cookie_attrs,
            );
        };

        let initial_language_from_localstorage_to_cookie_quote = {
            let quote = match initial_language_from_localstorage_to_cookie_bool
            {
                Some(ref lit) => match lit.value {
                    true => set_cookie_quote,
                    false => quote! {},
                },
                None => match initial_language_from_localstorage_to_cookie_expr
                {
                    Some(ref expr) => quote! {
                        if #expr {
                            #set_cookie_quote
                        }
                    },
                    None => quote! {},
                },
            };

            match initial_language_from_localstorage_to_cookie_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        let localstorage_get_quote = quote! {
            if let Some(l) = ::leptos_fluent::localstorage::get(#localstorage_key)
            {
                lang = ::leptos_fluent::l(
                    &l,
                    &LANGUAGES
                );

                if let Some(l) = lang {
                    #initial_language_from_localstorage_to_cookie_quote;
                }
            }
        };

        let quote = match initial_language_from_localstorage_bool {
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #localstorage_get_quote
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_localstorage_expr {
                Some(ref expr) => quote! {
                    if #expr && lang.is_none() {
                        #localstorage_get_quote
                    }
                },
                None => quote! {},
            },
        };

        match initial_language_from_localstorage_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_localstorage_bool;
        _ = initial_language_from_localstorage_expr;
        _ = initial_language_from_localstorage_exprpath;
        _ = initial_language_from_localstorage_to_cookie_bool;
        _ = initial_language_from_localstorage_to_cookie_expr;
        _ = initial_language_from_localstorage_to_cookie_exprpath;
    }

    let sync_language_with_url_param_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::url::set(
                    #url_param,
                    &#get_language_quote.id.to_string(),
                );
            });
        };

        let quote = match set_language_to_url_param_bool {
            Some(ref lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match set_language_to_url_param_expr {
                Some(ref expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        };

        match set_language_to_url_param_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_navigator_quote = {
        let initial_language_from_navigator_to_localstorage_quote = {
            let effect_quote = quote! {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key,
                    &l.id.to_string(),
                );
            };

            let quote = match initial_language_from_navigator_to_localstorage_bool {
                Some(ref lit) => match lit.value {
                    true => effect_quote,
                    false => quote! {},
                },
                None => {
                    match initial_language_from_navigator_to_localstorage_expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #effect_quote
                            }
                        },
                        None => quote! {},
                    }
                }
            };

            match initial_language_from_navigator_to_localstorage_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        let initial_language_from_navigator_to_cookie_quote = {
            let effect_quote = quote! {
                ::leptos_fluent::cookie::set(
                    #cookie_name,
                    &l.id.to_string(),
                    &#cookie_attrs,
                );
            };

            let quote = match initial_language_from_navigator_to_cookie_bool {
                Some(ref lit) => match lit.value {
                    true => effect_quote,
                    false => quote! {},
                },
                None => match initial_language_from_navigator_to_cookie_expr {
                    Some(ref expr) => quote! {
                        if #expr {
                            #effect_quote
                        }
                    },
                    None => quote! {},
                },
            };

            match initial_language_from_navigator_to_cookie_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        let window_navigator_languages_quote = quote! {
            let languages = window().navigator().languages().to_vec();
            for raw_language in languages {
                let language = raw_language.as_string();
                if language.is_none() {
                    continue;
                }
                if let Some(l) = ::leptos_fluent::l(
                    &language.unwrap(),
                    &LANGUAGES
                ) {
                    lang = Some(l);
                    #initial_language_from_navigator_to_localstorage_quote;
                    #initial_language_from_navigator_to_cookie_quote;
                    break;
                }
            }
        };

        let quote = match initial_language_from_navigator_bool {
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #window_navigator_languages_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_navigator_expr {
                Some(ref expr) => quote! {
                    if #expr && lang.is_none() {
                        #window_navigator_languages_quote;
                    }
                },
                None => quote! {},
            },
        };

        match initial_language_from_navigator_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_navigator_bool;
        _ = initial_language_from_navigator_expr;
        _ = initial_language_from_navigator_exprpath;
        _ = initial_language_from_navigator_to_localstorage_bool;
        _ = initial_language_from_navigator_to_localstorage_expr;
        _ = initial_language_from_navigator_to_localstorage_exprpath;
        _ = initial_language_from_navigator_to_cookie_bool;
        _ = initial_language_from_navigator_to_cookie_expr;
        _ = initial_language_from_navigator_to_cookie_exprpath;
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
                        if let Some(l) = ::leptos_fluent::l(&l, &LANGUAGES) {
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
                        #parse_actix_header_quote
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_accept_language_header_expr {
                Some(ref expr) => quote! {
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
                        if let Some(l) = ::leptos_fluent::l(&l, &LANGUAGES) {
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

            let quote = match initial_language_from_cookie_to_localstorage_bool
            {
                Some(ref lit) => match lit.value {
                    true => effect_quote,
                    false => quote! {},
                },
                None => match initial_language_from_cookie_to_localstorage_expr
                {
                    Some(ref expr) => quote! {
                        if #expr {
                            #effect_quote
                        }
                    },
                    None => quote! {},
                },
            };

            match initial_language_from_cookie_to_localstorage_exprpath {
                Some(ref path) => quote! { #path{#quote} },
                None => quote,
            }
        };

        let parse_client_cookie_quote = quote! {
            if let Some(cookie) = ::leptos_fluent::cookie::get(#cookie_name) {
                if let Some(l) = ::leptos_fluent::l(&cookie, &LANGUAGES) {
                    lang = Some(l);

                    #initial_language_from_cookie_to_localstorage_quote;
                }
            }
        };

        let quote = match initial_language_from_cookie_bool {
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_client_cookie_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_cookie_expr {
                Some(ref expr) => quote! {
                    if #expr && lang.is_none() {
                        #parse_client_cookie_quote;
                    }
                },
                None => quote! {},
            },
        };

        match initial_language_from_cookie_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(not(feature = "ssr"))]
    let sync_language_with_cookie_quote = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::cookie::set(
                    #cookie_name,
                    &#get_language_quote.id.to_string(),
                    &#cookie_attrs,
                );
            });
        };

        let quote = match set_language_to_cookie_bool {
            Some(ref lit) => match lit.value {
                true => effect_quote,
                false => quote! {},
            },
            None => match set_language_to_cookie_expr {
                Some(ref expr) => quote! {
                    if #expr {
                        #effect_quote
                    }
                },
                None => quote! {},
            },
        };

        match set_language_to_cookie_exprpath {
            Some(ref path) => quote! { #path{#quote} },
            None => quote,
        }
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_cookie_bool;
        _ = initial_language_from_cookie_expr;
        _ = initial_language_from_cookie_exprpath;
        _ = initial_language_from_cookie_to_localstorage_bool;
        _ = initial_language_from_cookie_to_localstorage_expr;
        _ = initial_language_from_cookie_to_localstorage_exprpath;
        _ = cookie_attrs_str;
        _ = cookie_attrs_expr;
        _ = cookie_attrs_exprpath;
        _ = set_language_to_cookie_bool;
        _ = set_language_to_cookie_expr;
        _ = set_language_to_cookie_exprpath;
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
                    if let Some(l) = ::leptos_fluent::l(&cookie, &LANGUAGES) {
                        lang = Some(l);
                    }
                }
            }
        };

        match initial_language_from_cookie_bool {
            Some(ref lit) => match lit.value {
                true => quote! {
                    if lang.is_none() {
                        #parse_actix_cookie_quote;
                    }
                },
                false => quote! {},
            },
            None => match initial_language_from_cookie_expr {
                Some(ref expr) => quote! {
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
                    if let Some(l) = ::leptos_fluent::l(&cookie, &LANGUAGES) {
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
            #initial_language_from_server_function_quote
            #initial_language_from_data_file_quote
            #initial_language_from_system_quote
            #initial_language_from_url_param_quote
            #initial_language_from_cookie_quote
            #initial_language_from_localstorage_quote
            #initial_language_from_navigator_quote
        }

        #[cfg(feature = "ssr")]
        quote! {
            #initial_language_from_server_function_quote
            #initial_language_from_url_param_quote
            #initial_language_from_cookie_quote
            #initial_language_from_accept_language_header_quote
        }
    };

    let translations = {
        let loader::Translations { simple, compound } = translations;

        quote! {{
            let mut all_loaders = Vec::new();
            all_loaders.extend([#(& #simple),*]);
            #(
                all_loaders.extend(#compound.iter());
            );*

            all_loaders
        }}
    };

    let leptos_fluent_provide_meta_context_quote = match provide_meta_context {
        true => {
            let bool_param = |boolean: Option<syn::LitBool>,
                              expr: Option<syn::Expr>|
             -> proc_macro2::TokenStream {
                match boolean {
                    Some(ref lit) => quote! { #lit },
                    None => match expr {
                        Some(ref expr) => quote! { #expr },
                        None => quote! { false },
                    },
                }
            };

            let maybe_litstr_param =
                |lit: Option<String>| -> proc_macro2::TokenStream {
                    match lit {
                        Some(ref lit) => quote! { #lit },
                        None => quote! { None },
                    }
                };

            let maybe_some_litstr_param =
                |lit: Option<String>| -> proc_macro2::TokenStream {
                    match lit {
                        Some(ref lit) => quote! { Some(#lit) },
                        None => quote! { None },
                    }
                };

            let litstr_or_default = |lit: Option<syn::LitStr>,
                                     expr: Option<syn::Expr>,
                                     default: &str|
             -> proc_macro2::TokenStream {
                match lit {
                    Some(ref lit) => quote! { #lit },
                    None => match expr {
                        Some(ref expr) => quote! { #expr },
                        None => quote! { #default },
                    },
                }
            };

            let core_locales_quote = maybe_litstr_param(core_locales_path);
            let languages_quote = maybe_some_litstr_param(raw_languages_path);
            let check_translations_quote =
                maybe_some_litstr_param(check_translations);
            let sync_html_tag_lang_quote =
                bool_param(sync_html_tag_lang_bool, sync_html_tag_lang_expr);
            let sync_html_tag_dir_quote =
                bool_param(sync_html_tag_dir_bool, sync_html_tag_dir_expr);
            let url_param_quote =
                litstr_or_default(url_param_str, url_param_expr, "lang");
            let initial_language_from_url_param_quote = bool_param(
                initial_language_from_url_param_bool,
                initial_language_from_url_param_expr,
            );
            let initial_language_from_url_param_to_localstorage = bool_param(
                initial_language_from_url_param_to_localstorage_bool,
                initial_language_from_url_param_to_localstorage_expr,
            );
            let initial_language_from_url_param_to_cookie_quote = bool_param(
                initial_language_from_url_param_to_cookie_bool,
                initial_language_from_url_param_to_cookie_expr,
            );
            let set_language_to_url_param_quote = bool_param(
                set_language_to_url_param_bool,
                set_language_to_url_param_expr,
            );
            let localstorage_key_quote = litstr_or_default(
                localstorage_key_str,
                localstorage_key_expr,
                "lang",
            );
            let initial_language_from_localstorage_quote = bool_param(
                initial_language_from_localstorage_bool,
                initial_language_from_localstorage_expr,
            );
            let initial_language_from_localstorage_to_cookie_quote = bool_param(
                initial_language_from_localstorage_to_cookie_bool,
                initial_language_from_localstorage_to_cookie_expr,
            );
            let set_language_to_localstorage_quote = bool_param(
                set_language_to_localstorage_bool,
                set_language_to_localstorage_expr,
            );
            let initial_language_from_navigator_quote = bool_param(
                initial_language_from_navigator_bool,
                initial_language_from_navigator_expr,
            );
            let initial_language_from_navigator_to_localstorage_quote =
                bool_param(
                    initial_language_from_navigator_to_localstorage_bool,
                    initial_language_from_navigator_to_localstorage_expr,
                );
            let initial_language_from_navigator_to_cookie_quote = bool_param(
                initial_language_from_navigator_to_cookie_bool,
                initial_language_from_navigator_to_cookie_expr,
            );
            let initial_language_from_accept_language_header_quote = bool_param(
                initial_language_from_accept_language_header_bool,
                initial_language_from_accept_language_header_expr,
            );
            let cookie_name_quote =
                litstr_or_default(cookie_name_str, cookie_name_expr, "lf-lang");
            let cookie_attrs_quote =
                litstr_or_default(cookie_attrs_str, cookie_attrs_expr, "");
            let initial_language_from_cookie_quote = bool_param(
                initial_language_from_cookie_bool,
                initial_language_from_cookie_expr,
            );
            let initial_language_from_cookie_to_localstorage_quote = bool_param(
                initial_language_from_cookie_to_localstorage_bool,
                initial_language_from_cookie_to_localstorage_expr,
            );
            let set_language_to_cookie_quote = bool_param(
                set_language_to_cookie_bool,
                set_language_to_cookie_expr,
            );
            let initial_language_from_server_function_quote =
                match initial_language_from_server_function {
                    Some(_) => quote! { true },
                    None => quote! { false },
                };
            let set_language_to_server_function_quote =
                match set_language_to_server_function {
                    Some(_) => quote! { true },
                    None => quote! { false },
                };

            let system_quote = {
                #[cfg(not(feature = "system"))]
                quote! {}

                #[cfg(feature = "system")]
                {
                    let initial_language_from_system_quote = bool_param(
                        initial_language_from_system_bool,
                        initial_language_from_system_expr,
                    );
                    let initial_language_from_data_file_quote = bool_param(
                        initial_language_from_data_file_bool,
                        initial_language_from_data_file_expr,
                    );
                    let initial_language_from_system_to_data_file_quote =
                        bool_param(
                            initial_language_from_system_to_data_file_bool,
                            initial_language_from_system_to_data_file_expr,
                        );
                    let set_language_to_data_file_quote = bool_param(
                        set_language_to_data_file_bool,
                        set_language_to_data_file_expr,
                    );
                    let data_file_key_quote = litstr_or_default(
                        data_file_key_str,
                        data_file_key_expr,
                        "leptos-fluent",
                    );

                    quote! {
                        initial_language_from_system: #initial_language_from_system_quote,
                        initial_language_from_data_file: #initial_language_from_data_file_quote,
                        initial_language_from_system_to_data_file: #initial_language_from_system_to_data_file_quote,
                        set_language_to_data_file: #set_language_to_data_file_quote,
                        data_file_key: #data_file_key_quote,
                    }
                }
            };

            let quote = quote! {
                const meta: ::leptos_fluent::LeptosFluentMeta = ::leptos_fluent::LeptosFluentMeta {
                    locales: #locales_path,
                    core_locales: #core_locales_quote,
                    languages: #languages_quote,
                    check_translations: #check_translations_quote,
                    sync_html_tag_lang: #sync_html_tag_lang_quote,
                    sync_html_tag_dir: #sync_html_tag_dir_quote,
                    url_param: #url_param_quote,
                    initial_language_from_url_param: #initial_language_from_url_param_quote,
                    initial_language_from_url_param_to_localstorage: #initial_language_from_url_param_to_localstorage,
                    initial_language_from_url_param_to_cookie: #initial_language_from_url_param_to_cookie_quote,
                    set_language_to_url_param: #set_language_to_url_param_quote,
                    localstorage_key: #localstorage_key_quote,
                    initial_language_from_localstorage: #initial_language_from_localstorage_quote,
                    initial_language_from_localstorage_to_cookie: #initial_language_from_localstorage_to_cookie_quote,
                    set_language_to_localstorage: #set_language_to_localstorage_quote,
                    initial_language_from_navigator: #initial_language_from_navigator_quote,
                    initial_language_from_navigator_to_localstorage: #initial_language_from_navigator_to_localstorage_quote,
                    initial_language_from_navigator_to_cookie: #initial_language_from_navigator_to_cookie_quote,
                    initial_language_from_accept_language_header: #initial_language_from_accept_language_header_quote,
                    cookie_name: #cookie_name_quote,
                    cookie_attrs: #cookie_attrs_quote,
                    initial_language_from_cookie: #initial_language_from_cookie_quote,
                    initial_language_from_cookie_to_localstorage: #initial_language_from_cookie_to_localstorage_quote,
                    set_language_to_cookie: #set_language_to_cookie_quote,
                    initial_language_from_server_function: #initial_language_from_server_function_quote,
                    set_language_to_server_function: #set_language_to_server_function_quote,
                    provide_meta_context: true,
                    #system_quote
                };
                ::leptos::provide_context::<::leptos_fluent::LeptosFluentMeta>(meta);
            };

            match provide_meta_context_exprpath {
                Some(ref path) => quote! {
                    #path{#quote};
                },
                None => quote,
            }
        }
        false => quote! {},
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
            ::leptos::provide_context::<::leptos_fluent::I18n>(i18n);
            #sync_html_tag_lang_quote
            #sync_html_tag_dir_quote
            #sync_language_with_server_function_quote
            #sync_language_with_localstorage_quote
            #sync_language_with_url_param_quote
            #sync_language_with_cookie_quote
            #sync_language_with_data_file_quote
            #files_tracker_quote
            #leptos_fluent_provide_meta_context_quote;

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
