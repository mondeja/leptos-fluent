#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(track_path))]
#![cfg_attr(feature = "nightly", feature(absolute_path))]

//! Macros for the leptos-fluent crate.
//!
//! See [leptos-fluent] for more information.
//!
//! [leptos-fluent]: https://crates.io/crates/leptos-fluent

extern crate proc_macro;

pub(crate) mod cookie;
mod exprpath;
mod files_tracker;
mod fluent_resources;
mod languages;
mod loader;
#[cfg(not(feature = "ssr"))]
mod translations_checker;

pub(crate) use exprpath::evaluate_exprpath;
use files_tracker::build_files_tracker_quote;
#[cfg(not(feature = "ssr"))]
pub(crate) use fluent_resources::FluentResources;
pub(crate) use fluent_resources::{
    build_fluent_resources_and_file_paths, FluentFilePaths,
};
use languages::build_languages_quote;
pub(crate) use languages::ParsedLanguage;
use loader::{I18nLoader, Identifier, LitBoolExpr};
use quote::quote;

#[cfg(feature = "debug")]
#[inline(always)]
pub(crate) fn debug(msg: &str) {
    println!("[leptos-fluent/debug] {}", msg);
}

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
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
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
        sync_html_tag_lang,
        sync_html_tag_dir,
        initial_language_from_url_param,
        url_param,
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
        ::leptos_fluent::i18n()()
    };

    #[cfg(all(feature = "nightly", not(feature = "ssr")))]
    let set_language_quote = quote! {
        ::leptos_fluent::i18n()(l)
    };

    #[cfg(not(feature = "nightly"))]
    let get_language_quote = quote! {
        ::leptos_fluent::i18n().language.get()
    };

    #[cfg(all(not(feature = "nightly"), not(feature = "ssr")))]
    let set_language_quote = quote! {
        ::leptos_fluent::i18n().language.set(l)
    };

    let cookie_name_quote = match cookie_name.lit {
        Some(ref lit) => quote! { #lit },
        None => match cookie_name.expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "lf-lang" },
        },
    };

    let cookie_attrs_quote = match cookie_attrs.lit {
        Some(ref lit) => quote! { #lit },
        None => match cookie_attrs.expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "" },
        },
    };

    let localstorage_key_quote = match localstorage_key.lit {
        Some(ref lit) => quote! { #lit },
        None => match localstorage_key.expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    #[cfg(feature = "system")]
    let data_file_key_quote = match data_file_key.lit {
        Some(ref lit) => quote! { #lit },
        None => match data_file_key.expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "leptos-fluent" },
        },
    };

    // discover from system language (desktop apps)
    #[cfg(all(feature = "system", not(feature = "ssr")))]
    let initial_language_from_system_quote: proc_macro2::TokenStream = {
        let initial_language_from_system_to_data_file_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::data_file::set(
                    #data_file_key_quote,
                    &l.id.to_string(),
                );
            };

            initial_language_from_system_to_data_file.iter().map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! {
                           if lang.is_none() && !#data_file_key_quote.is_empty() {
                               #effect_quote
                           }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if lang.is_none() && #expr && !#data_file_key_quote.is_empty() {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    }
                }
            }).collect()
        };

        let effect_quote = quote! {
            if let Ok(l) = ::leptos_fluent::current_locale::current_locale() {
                lang = ::leptos_fluent::l(&l, &LANGUAGES);
                if let Some(l) = lang {
                    #initial_language_from_system_to_data_file_quote
                }
            }
        };

        initial_language_from_system
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! {
                            if lang.is_none() {
                                #effect_quote
                            }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr && lang.is_none() {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    #[cfg(all(not(feature = "system"), not(feature = "ssr")))]
    let initial_language_from_system_quote = quote! {};

    #[cfg(all(feature = "system", feature = "ssr"))]
    {
        _ = data_file_key;
        _ = initial_language_from_system;
        _ = initial_language_from_system_to_data_file;
    }

    #[cfg(feature = "system")]
    let sync_language_with_data_file_quote: proc_macro2::TokenStream =
        set_language_to_data_file
            .iter()
            .map(|param| {
                let set_language_to_data_file_quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! { #data_file_key_quote },
                        false => quote! { "" },
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #data_file_key_quote
                            } else {
                                ""
                            }
                        },
                        None => quote! { "" },
                    },
                };

                // TODO: optimize checking if empty at compile time when literal
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

                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => effect_quote,
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect();

    #[cfg(not(feature = "system"))]
    let sync_language_with_data_file_quote = quote! {};

    #[cfg(all(feature = "system", not(feature = "ssr")))]
    let initial_language_from_data_file_quote: proc_macro2::TokenStream =
        initial_language_from_data_file
            .iter()
            .map(|param| {
                let initial_language_from_data_file_quote = match param.lit {
                    Some(ref lit) => match lit.value() {
                        true => quote! { #data_file_key_quote },
                        false => quote! { "" },
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #data_file_key_quote
                            } else {
                                ""
                            }
                        },
                        None => quote! { "" },
                    },
                };

                let effect_quote = quote! {
                    if #initial_language_from_data_file_quote.is_empty() {
                        return;
                    }
                    if let Some(l) = ::leptos_fluent::data_file::get(
                        #initial_language_from_data_file_quote
                    ) {
                        lang = ::leptos_fluent::l(&l, &LANGUAGES);
                    }
                };

                let quote = match param.lit {
                    Some(ref lit) => match lit.value() {
                        true => quote! {
                            if lang.is_none() {
                                #effect_quote
                            }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr && lang.is_none() {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect();

    #[cfg(all(not(feature = "system"), not(feature = "ssr")))]
    let initial_language_from_data_file_quote = quote! {};

    #[cfg(all(feature = "system", feature = "ssr"))]
    {
        _ = initial_language_from_data_file;
    }

    let initial_language_from_server_function_quote: proc_macro2::TokenStream = {
        let set_to_cookie_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::cookie::set(
                    #cookie_name_quote,
                    &l.id.to_string(),
                    &#cookie_attrs_quote
                );
            };

            initial_language_from_server_function_to_cookie
                .iter()
                .map(|param| {
                    let quote = match param.lit {
                        Some(ref lit) => match lit.value {
                            true => effect_quote.clone(),
                            false => quote!(),
                        },
                        None => match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #effect_quote
                                }
                            },
                            None => quote!(),
                        },
                    };

                    match quote.is_empty() {
                        true => quote!(),
                        false => match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        },
                    }
                })
                .collect()
        };

        #[cfg(not(feature = "ssr"))]
        let set_to_localstorage_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key_quote,
                    &l.id.to_string()
                );
            };

            initial_language_from_server_function_to_localstorage
                .iter()
                .map(|param| {
                    let quote = match param.lit {
                        Some(ref lit) => match lit.value {
                            true => effect_quote.clone(),
                            false => quote!(),
                        },
                        None => match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #effect_quote
                                }
                            },
                            None => quote!(),
                        },
                    };

                    match quote.is_empty() {
                        true => quote!(),
                        false => match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        },
                    }
                })
                .collect()
        };

        #[cfg(feature = "ssr")]
        let set_to_localstorage_quote = {
            _ = initial_language_from_server_function_to_localstorage;

            quote! {}
        };

        initial_language_from_server_function
            .iter()
            .map(|param| {
                let ident = &param.ident;
                let effect_quote = quote! {
                    spawn_local(async move {
                        let lang_result = #ident().await;
                        if let Ok(maybe_lang) = lang_result {
                            if let Some(l) = maybe_lang {
                                lang = ::leptos_fluent::l(&l, &LANGUAGES);
                                if let Some(l) = lang {
                                    #set_to_cookie_quote
                                    #set_to_localstorage_quote
                                }

                            }
                        }
                    });
                };

                match param.ident {
                    Some(_) => {
                        let quote = quote! {
                            if lang.is_none() {
                                #effect_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        }
                    }
                    None => quote!(),
                }
            })
            .collect()
    };

    let sync_language_with_server_function_quote: proc_macro2::TokenStream =
        set_language_to_server_function.iter().map(|param| {
            let ident = &param.ident;
            let effect_quote = quote! {
                ::leptos::create_effect(move |_| {
                    spawn_local(async {
                        _ = #ident(#get_language_quote.id.to_string()).await;
                    });
                });
            };

            match param.ident {
                Some(_) => match param.exprpath {
                    Some(ref path) => quote! { #path{#effect_quote} },
                    None => effect_quote,
                },
                None => quote!(),
            }
        }).collect();

    let initial_language_from_url_path_to_server_function_quote: proc_macro2::TokenStream =
        initial_language_from_url_path_to_server_function.iter().map(|param| {
            match param.ident {
                Some(ref ident) => {
                    let quote = quote! {
                        spawn_local(async move {
                            _ = #ident(l.id.to_string()).await;
                        });
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    }
                },
                None => quote!(),
            }
        }).collect();

    let initial_language_from_url_path_quote: proc_macro2::TokenStream = {
        if url_path.is_none() {
            quote! {}
        } else {
            let ident = url_path.as_ref().unwrap();

            initial_language_from_url_path.iter().map(|param| {
                #[cfg(not(feature = "ssr"))]
                let effect_quote = {
                    let to_cookie_effect_quote: proc_macro2::TokenStream =
                        initial_language_from_url_path_to_cookie.iter().map(|param| {
                            let effect_quote = quote! {
                                ::leptos_fluent::cookie::set(
                                    #cookie_name_quote,
                                    &l.id.to_string(),
                                    &#cookie_attrs_quote
                                );
                            };

                            let quote = match param.lit {
                                Some(ref lit) => match lit.value {
                                    true => effect_quote,
                                    false => quote!(),
                                },
                                None => match param.expr {
                                    Some(ref expr) => quote! {
                                        if #expr {
                                            #effect_quote
                                        }
                                    },
                                    None => quote!(),
                                },
                            };

                            match quote.is_empty() {
                                true => quote!(),
                                false => match param.exprpath {
                                    Some(ref path) => quote!(#path{#quote}),
                                    None => quote,
                                },
                            }
                        }).collect();

                    let to_localstorage_effect_quote: proc_macro2::TokenStream =
                        initial_language_from_url_path_to_localstorage.iter().map(|param| {
                            let effect_quote = quote! {
                                ::leptos_fluent::localstorage::set(
                                    #localstorage_key_quote,
                                    &l.id.to_string()
                                );
                            };

                            let quote = match param.lit {
                                Some(ref lit) => match lit.value {
                                    true => effect_quote,
                                    false => quote!(),
                                },
                                None => match param.expr {
                                    Some(ref expr) => quote! {
                                        if #expr {
                                            #effect_quote
                                        }
                                    },
                                    None => quote!(),
                                },
                            };

                            match quote.is_empty() {
                                true => quote!(),
                                false => match param.exprpath {
                                    Some(ref path) => quote!(#path{#quote}),
                                    None => quote,
                                },
                            }
                        }).collect();

                    quote! {
                        if let Some(url_path) = ::leptos_fluent::url::path::get() {
                            lang = ::leptos_fluent::l(#ident(&url_path), &LANGUAGES);
                            if let Some(l) = lang {
                                #to_cookie_effect_quote
                                #to_localstorage_effect_quote
                                #initial_language_from_url_path_to_server_function_quote
                            }
                        }
                    }
                };

                #[cfg(all(feature = "ssr", feature = "actix"))]
                let effect_quote = quote! {
                    if let Some(req) = ::leptos::use_context::<::actix_web::HttpRequest>() {
                        lang = ::leptos_fluent::l(#ident(&req.path()), &LANGUAGES);
                        if let Some(l) = lang {
                            #initial_language_from_url_path_to_server_function_quote
                        }
                    }
                };

                #[cfg(all(feature = "ssr", feature = "axum"))]
                let effect_quote = quote! {
                    if let Some(req) = ::leptos::use_context::<::axum::http::request::Parts>() {
                        lang = ::leptos_fluent::l(#ident(&req.uri.path()), &LANGUAGES);
                        if let Some(l) = lang {
                            #initial_language_from_url_path_to_server_function_quote
                        }
                    }
                };

                #[cfg(all(feature = "ssr", not(feature = "axum"), not(feature = "actix")))]
                let effect_quote = {
                    _ = initial_language_from_url_path_to_server_function_quote;
                    quote! {}
                };

                let quote = quote! {
                    if lang.is_none() {
                        #effect_quote
                    }
                };

                match param.exprpath {
                    Some(ref path) => quote! {
                        #path{#quote}
                    },
                    None => quote,
                }
            }).collect()
        }
    };

    let sync_html_tag_quote: proc_macro2::TokenStream = {
        // TODO: optimize code size
        // TODO: handle other attributes in Leptos v0.7
        //       (in Leptos v0.6 attribute keys are static)

        // Calling `provide_meta_context()` to not show a warning
        #[cfg(feature = "ssr")]
        let previous_html_tag_attrs_quote = quote! {{
            ::leptos_fluent::leptos_meta::provide_meta_context();
            let html_tag_as_string = ::leptos_fluent::leptos_meta::use_head().html.as_string().unwrap_or("".to_string());
            let mut class: Option<::leptos::TextProp> = None;
            let mut lang: Option<::leptos::TextProp> = None;
            let mut dir: Option<::leptos::TextProp> = None;
            for attr in html_tag_as_string.split(' ') {
                let mut parts = attr.split('=');
                let key = parts.next().unwrap_or("");
                let value = parts.next().unwrap_or("");
                if key == "class" {
                    class = Some(value.trim_matches('"').to_string().into());
                } else if key == "lang" {
                    lang = Some(value.trim_matches('"').to_string().into());
                } else if key == "dir" {
                    dir = Some(value.trim_matches('"').to_string().into());
                }
            }
            (class, lang, dir)
        }};

        let sync_html_tag_lang_effect_quote = {
            #[cfg(feature = "ssr")]
            {
                let sync_html_tag_dir_bool_quote: proc_macro2::TokenStream = {
                    let quote = sync_html_tag_dir
                        .iter()
                        .map(|param| {
                            let quote = match param.lit {
                                Some(ref lit) => quote! { #lit },
                                None => match param.expr {
                                    Some(ref expr) => quote! { #expr },
                                    None => quote! { false },
                                },
                            };

                            match quote.is_empty() {
                                true => quote! { false },
                                false => match param.exprpath {
                                    Some(ref path) => quote!(#path{#quote}),
                                    None => quote,
                                },
                            }
                        })
                        .collect::<proc_macro2::TokenStream>();

                    match quote.is_empty() {
                        true => quote! { false },
                        false => quote! { #quote },
                    }
                };

                quote! {
                    let l = #get_language_quote;
                    let (class, _, dir) = #previous_html_tag_attrs_quote;
                    ::leptos_fluent::leptos_meta::Html(
                        ::leptos_fluent::leptos_meta::HtmlProps {
                            lang: Some(l.id.to_string().into()),
                            dir: if #sync_html_tag_dir_bool_quote {
                                Some(l.dir.as_str().into())
                            } else {
                                dir
                            },
                            class,
                            attributes: Vec::new(),
                        }
                    );
                }
            }

            #[cfg(not(feature = "ssr"))]
            quote! {
                ::leptos::create_effect(move |_| {
                    use leptos_fluent::web_sys::wasm_bindgen::JsCast;
                    _ = ::leptos::document()
                        .document_element()
                        .unwrap()
                        .unchecked_into::<::leptos_fluent::web_sys::HtmlElement>()
                        .set_attribute(
                            "lang",
                            &#get_language_quote.id.to_string()
                        );
                });
            }
        };

        let sync_html_tag_dir_effect_quote = {
            #[cfg(feature = "ssr")]
            {
                let sync_html_tag_lang_bool_quote: proc_macro2::TokenStream = {
                    let quote = sync_html_tag_lang
                        .iter()
                        .map(|param| {
                            let quote = match param.lit {
                                Some(ref lit) => quote! { #lit },
                                None => match param.expr {
                                    Some(ref expr) => quote! { #expr },
                                    None => quote! { false },
                                },
                            };

                            match quote.is_empty() {
                                true => quote! { false },
                                false => match param.exprpath {
                                    Some(ref path) => quote!(#path{#quote}),
                                    None => quote,
                                },
                            }
                        })
                        .collect::<proc_macro2::TokenStream>();

                    match quote.is_empty() {
                        true => quote! { false },
                        false => quote! { #quote },
                    }
                };

                quote! {
                    let l = #get_language_quote;
                    let (class, lang, _) = #previous_html_tag_attrs_quote;
                    ::leptos_fluent::leptos_meta::Html(
                        ::leptos_fluent::leptos_meta::HtmlProps {
                            lang: if #sync_html_tag_lang_bool_quote {
                                Some(l.id.to_string().into())
                            } else {
                                lang
                            },
                            dir: Some(l.dir.as_str().into()),
                            class,
                            attributes: Vec::new(),
                        }
                    );
                }
            }

            #[cfg(not(feature = "ssr"))]
            quote! {
                ::leptos::create_effect(move |_| {
                    use leptos_fluent::web_sys::wasm_bindgen::JsCast;
                    _ = ::leptos::document()
                        .document_element()
                        .unwrap()
                        .unchecked_into::<::leptos_fluent::web_sys::HtmlElement>()
                        .set_attribute(
                            "dir",
                            &#get_language_quote.dir.as_str()
                        );
                });
            }
        };

        let sync_html_tag_lang_quote: proc_macro2::TokenStream =
            sync_html_tag_lang
                .iter()
                .map(|param| {
                    let quote = match param.lit {
                        Some(ref lit) => match lit.value {
                            true => sync_html_tag_lang_effect_quote.clone(),
                            false => quote!(),
                        },
                        None => match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #sync_html_tag_lang_effect_quote
                                }
                            },
                            None => quote!(),
                        },
                    };

                    match quote.is_empty() {
                        true => quote!(),
                        false => match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        },
                    }
                })
                .collect();

        let sync_html_tag_dir_quote: proc_macro2::TokenStream =
            sync_html_tag_dir
                .iter()
                .map(|param| {
                    let quote = match param.lit {
                        Some(ref lit) => match lit.value {
                            true => sync_html_tag_dir_effect_quote.clone(),
                            false => quote!(),
                        },
                        None => match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #sync_html_tag_dir_effect_quote
                                }
                            },
                            None => quote!(),
                        },
                    };

                    match quote.is_empty() {
                        true => quote!(),
                        false => match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        },
                    }
                })
                .collect();

        quote! {
            #sync_html_tag_lang_quote
            #sync_html_tag_dir_quote
        }
    };

    let url_param_quote = match url_param.lit {
        Some(ref lit) => quote! { #lit },
        None => match url_param.expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    let sync_language_with_localstorage_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key_quote,
                    &#get_language_quote.id.to_string()
                );
            });
        };

        set_language_to_localstorage
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => effect_quote.clone(),
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    let initial_language_from_url_param_quote: proc_macro2::TokenStream = {
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
        let set_to_localstorage_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key_quote,
                    &l.id.to_string()
                );
            };

            initial_language_from_url_param_to_localstorage
                .iter()
                .map(|param| {
                    let quote = match param.lit {
                        Some(ref lit) => match lit.value {
                            true => effect_quote.clone(),
                            false => quote!(),
                        },
                        None => match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #effect_quote
                                }
                            },
                            None => quote!(),
                        },
                    };

                    match quote.is_empty() {
                        true => quote!(),
                        false => match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        },
                    }
                })
                .collect()
        };

        #[cfg(not(feature = "ssr"))]
        let set_to_cookie_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::cookie::set(
                    #cookie_name_quote,
                    &l.id.to_string(),
                    &#cookie_attrs_quote
                );
            };

            initial_language_from_url_param_to_cookie
                .iter()
                .map(|param| {
                    let quote = match param.lit {
                        Some(ref lit) => match lit.value {
                            true => effect_quote.clone(),
                            false => quote!(),
                        },
                        None => match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #effect_quote
                                }
                            },
                            None => quote!(),
                        },
                    };

                    match quote.is_empty() {
                        true => quote!(),
                        false => match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        },
                    }
                })
                .collect()
        };

        #[cfg(feature = "ssr")]
        {
            _ = initial_language_from_url_param_to_localstorage;
            _ = initial_language_from_url_param_to_cookie;
        }

        let set_to_server_function_quote: proc_macro2::TokenStream =
            initial_language_from_url_param_to_server_function
                .iter()
                .map(|param| match param.ident {
                    Some(ref ident) => {
                        let quote = quote! {
                            spawn_local(async move {
                                _ = #ident(l.id.to_string()).await;
                            });
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        }
                    }
                    None => quote!(),
                })
                .collect();

        #[cfg(not(feature = "ssr"))]
        let parse_language_quote = quote! {
            if let Some(l) = ::leptos_fluent::url::param::get(#url_param_quote) {
                lang = ::leptos_fluent::l(&l, &LANGUAGES);
                if let Some(l) = lang {
                    #hydrate_rerender_quote
                    #set_to_localstorage_quote
                    #set_to_cookie_quote
                    #set_to_server_function_quote
                }
            }
        };

        #[cfg(all(feature = "ssr", any(feature = "actix", feature = "axum")))]
        let lang_parser_quote = quote! {
            let mut maybe_lang = None;
            for (key, value) in uri_query.split('&').map(|pair| {
                let mut split = pair.splitn(2, '=');
                (split.next().unwrap_or(""), split.next().unwrap_or(""))
            }) {
                if key == #url_param_quote {
                    maybe_lang = Some(value);
                    break;
                }
            }

            if let Some(l) = maybe_lang {
                lang = ::leptos_fluent::l(&l, &LANGUAGES);
                if let Some(l) = lang {
                    #set_to_server_function_quote
                }
            }
        };

        #[cfg(all(feature = "ssr", feature = "actix"))]
        let parse_language_quote = quote! {
            if let Some(req) = ::leptos::use_context::<actix_web::HttpRequest>() {
                let uri_query = req.uri().query().unwrap_or("");
                #lang_parser_quote
            }
        };

        #[cfg(all(feature = "ssr", feature = "axum"))]
        let parse_language_quote = quote! {
            if let Some(req) = ::leptos::use_context::<::axum::http::request::Parts>() {
                let uri_query = req.uri.query().unwrap_or("");
                #lang_parser_quote
            }
        };

        // Other SSR framework or the user is not using any
        #[cfg(all(
            feature = "ssr",
            not(feature = "actix"),
            not(feature = "axum"),
        ))]
        let parse_language_quote = quote! {};

        initial_language_from_url_param
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => parse_language_quote.clone(),
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => {
                            match parse_language_quote.is_empty() {
                                true => quote!(),
                                false => quote! {
                                    if #expr {
                                        #parse_language_quote
                                    }
                                },
                            }
                        }
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_localstorage_quote: proc_macro2::TokenStream = {
        let set_cookie_quote = quote! {
            ::leptos_fluent::cookie::set(
                #cookie_name_quote,
                &l.id.to_string(),
                &#cookie_attrs_quote
            );
        };

        let initial_language_from_localstorage_to_cookie_quote: proc_macro2::TokenStream =
            initial_language_from_localstorage_to_cookie.iter().map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => set_cookie_quote.clone(),
                        false => quote!(),
                    },
                    None => match param.expr
                    {
                        Some(ref expr) => quote! {
                            if #expr {
                                #set_cookie_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    }
                }
            }).collect();

        let initial_language_from_localstorage_to_server_function_quote: proc_macro2::TokenStream =
            initial_language_from_localstorage_to_server_function.iter().map(|param| {
                match param.ident {
                    Some(ref ident) => {
                        let quote = quote! {
                            spawn_local(async move {
                                _ = #ident(l.id.to_string()).await;
                            });
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        }
                    },
                    None => quote!(),
                }
            }).collect();

        let localstorage_get_quote = quote! {
            if let Some(l) = ::leptos_fluent::localstorage::get(#localstorage_key_quote)
            {
                lang = ::leptos_fluent::l(&l, &LANGUAGES);
                if let Some(l) = lang {
                    #initial_language_from_localstorage_to_cookie_quote
                    #initial_language_from_localstorage_to_server_function_quote
                }
            }
        };

        initial_language_from_localstorage
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! {
                            if lang.is_none() {
                                #localstorage_get_quote
                            }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr && lang.is_none() {
                                #localstorage_get_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_localstorage;
        _ = initial_language_from_localstorage_to_cookie;
        _ = initial_language_from_localstorage_to_server_function;
    }

    let sync_language_with_url_param_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::url::param::set(
                    #url_param_quote,
                    &#get_language_quote.id.to_string()
                );
            });
        };

        set_language_to_url_param
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => effect_quote.clone(),
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_navigator_quote: proc_macro2::TokenStream = {
        let initial_language_from_navigator_to_localstorage_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key_quote,
                    &l.id.to_string()
                );
            };

            initial_language_from_navigator_to_localstorage.iter().map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => effect_quote.clone(),
                        false => quote!(),
                    },
                    None => {
                        match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #effect_quote
                                }
                            },
                            None => quote!(),
                        }
                    }
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            }).collect()
        };

        let initial_language_from_navigator_to_cookie_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::cookie::set(
                    #cookie_name_quote,
                    &l.id.to_string(),
                    &#cookie_attrs_quote
                );
            };

            initial_language_from_navigator_to_cookie.iter().map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => effect_quote.clone(),
                        false => quote!(),
                    },
                    None => {
                        match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #effect_quote
                                }
                            },
                            None => quote!(),
                        }
                    }
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            }).collect()
        };

        let initial_language_from_navigator_to_server_function_quote: proc_macro2::TokenStream =
            initial_language_from_navigator_to_server_function.iter().map(|param| {
                match param.ident {
                    Some(ref ident) => {
                        let quote = quote! {
                            spawn_local(async move {
                                _ = #ident(l.id.to_string()).await;
                            });
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        }
                    },
                    None => quote!(),
                }
            }).collect();

        let window_navigator_languages_quote = quote! {
            let languages = window().navigator().languages().to_vec();
            for raw_language in languages {
                let language = raw_language.as_string();
                if language.is_none() {
                    continue;
                }
                lang = ::leptos_fluent::l(&language.unwrap(), &LANGUAGES);
                if let Some(l) = lang {
                    #initial_language_from_navigator_to_localstorage_quote
                    #initial_language_from_navigator_to_cookie_quote
                    #initial_language_from_navigator_to_server_function_quote
                    break;
                }
            }
        };

        initial_language_from_navigator
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! {
                            if lang.is_none() {
                                #window_navigator_languages_quote
                            }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr && lang.is_none() {
                                #window_navigator_languages_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_navigator;
        _ = initial_language_from_navigator_to_localstorage;
        _ = initial_language_from_navigator_to_cookie;
        _ = initial_language_from_navigator_to_server_function;
    }

    let set_language_from_navigator_quote: proc_macro2::TokenStream = {
        #[cfg(not(feature = "ssr"))]
        {
            let effect_quote = quote! {
                use ::leptos_fluent::web_sys::wasm_bindgen::JsCast;
                let closure: Box<dyn FnMut(_)> = Box::new(
                    move |_: web_sys::Window| {
                        let languages = window().navigator().languages().to_vec();
                        for raw_language in languages {
                            let language = raw_language.as_string();
                            if language.is_none() {
                                continue;
                            }
                            let l = ::leptos_fluent::l(&language.unwrap(), &LANGUAGES);
                            ::leptos::logging::log!("Language changed to {:?}", &l);
                            if let Some(l) = l {
                                #set_language_quote;
                                break;
                            }
                        }
                    }
                );
                let cb = ::leptos_fluent::web_sys::wasm_bindgen::closure::Closure::wrap(
                    closure
                );
                ::leptos::window().add_event_listener_with_callback(
                    "languagechange",
                    cb.as_ref().unchecked_ref()
                ).expect("Failed to add event listener for window languagechange");
                cb.forget();
            };

            set_language_from_navigator
                .iter()
                .map(|param| {
                    let quote = match param.lit {
                        Some(ref lit) => match lit.value {
                            true => effect_quote.clone(),
                            false => quote!(),
                        },
                        None => match param.expr {
                            Some(ref expr) => quote! {
                                if #expr {
                                    #effect_quote
                                }
                            },
                            None => quote!(),
                        },
                    };

                    match quote.is_empty() {
                        true => quote!(),
                        false => match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        },
                    }
                })
                .collect()
        }

        #[cfg(feature = "ssr")]
        {
            _ = set_language_from_navigator;
            quote!()
        }
    };

    // Accept-Language header
    //   Actix
    #[cfg(all(feature = "actix", feature = "ssr"))]
    let initial_language_from_accept_language_header_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            if let Some(req) = ::leptos::use_context::<::actix_web::HttpRequest>() {
                let maybe_header = req
                    .headers()
                    .get(::actix_web::http::header::ACCEPT_LANGUAGE)
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

        initial_language_from_accept_language_header.iter().map(|param| {
            let quote = match param.lit {
                Some(ref lit) => match lit.value {
                    true => quote! {
                        if lang.is_none() {
                            #effect_quote
                        }
                    },
                    false => quote!(),
                },
                None => match param.expr {
                    Some(ref expr) => quote! {
                        if #expr && lang.is_none() {
                            #effect_quote
                        }
                    },
                    None => quote!(),
                },
            };

            match quote.is_empty() {
                true => quote!(),
                false => match param.exprpath {
                    Some(ref path) => quote!(#path{#quote}),
                    None => quote,
                },
            }
        }).collect()
    };

    //   Axum
    #[cfg(all(feature = "axum", feature = "ssr"))]
    let initial_language_from_accept_language_header_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            if let Some(req) = ::leptos::use_context::<::axum::http::request::Parts>() {
                let maybe_header = req
                    .headers
                    .get(::axum::http::header::ACCEPT_LANGUAGE)
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

        initial_language_from_accept_language_header.iter().map(|param| {
            let quote = match param.lit {
                Some(ref lit) => match lit.value {
                    true => quote! {
                        if lang.is_none() {
                            #effect_quote
                        }
                    },
                    false => quote!(),
                },
                None => match param.expr {
                    Some(ref expr) => quote! {
                        if #expr && lang.is_none() {
                            #effect_quote
                        }
                    },
                    None => quote!(),
                },
            };

            match quote.is_empty() {
                true => quote!(),
                false => match param.exprpath {
                    Some(ref path) => quote!(#path{#quote}),
                    None => quote,
                },
            }
        }).collect()
    };

    //   Other SSR framework or the user is not using any
    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let initial_language_from_accept_language_header_quote = quote! {};

    // No SSR
    #[cfg(not(feature = "ssr"))]
    {
        _ = initial_language_from_accept_language_header;
    }

    // Cookie
    let initial_language_from_cookie_to_server_function_quote: proc_macro2::TokenStream =
        initial_language_from_cookie_to_server_function.iter().map(|param| {
            match param.ident {
                Some(ref ident) => {
                    let quote = quote! {
                        spawn_local(async move {
                            _ = #ident(l.id.to_string()).await;
                        });
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    }
                },
                None => quote!(),
            }
        }).collect();

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_cookie_quote: proc_macro2::TokenStream = {
        let initial_language_from_cookie_to_localstorage_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key_quote,
                    &l.id.to_string()
                );
            };

            initial_language_from_cookie_to_localstorage.iter().map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => effect_quote.clone(),
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            }).collect()
        };

        let parse_client_cookie_quote = quote! {
            if let Some(cookie) = ::leptos_fluent::cookie::get(#cookie_name_quote) {
                if let Some(l) = ::leptos_fluent::l(&cookie, &LANGUAGES) {
                    lang = Some(l);
                    #initial_language_from_cookie_to_localstorage_quote
                    #initial_language_from_cookie_to_server_function_quote
                }
            }
        };

        initial_language_from_cookie
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! {
                            if lang.is_none() {
                                #parse_client_cookie_quote
                            }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr && lang.is_none() {
                                #parse_client_cookie_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    #[cfg(not(feature = "ssr"))]
    let sync_language_with_cookie_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            ::leptos::create_effect(move |_| {
                ::leptos_fluent::cookie::set(
                    #cookie_name_quote,
                    &#get_language_quote.id.to_string(),
                    &#cookie_attrs_quote
                );
            });
        };

        set_language_to_cookie
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => effect_quote.clone(),
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_cookie;
        _ = initial_language_from_cookie_to_localstorage;
        _ = cookie_attrs;
        _ = set_language_to_cookie;
    }

    //   Actix
    #[cfg(all(feature = "ssr", feature = "actix"))]
    let initial_language_from_cookie_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            if let Some(req) = ::leptos::use_context::<::actix_web::HttpRequest>() {
                let maybe_cookie = req
                    .cookie(#cookie_name_quote)
                    .and_then(|cookie| Some(cookie.value().to_string()));

                if let Some(cookie) = maybe_cookie {
                    if let Some(l) = ::leptos_fluent::l(&cookie, &LANGUAGES) {
                        lang = Some(l);
                        #initial_language_from_cookie_to_server_function_quote
                    }
                }
            }
        };

        initial_language_from_cookie
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! {
                            if lang.is_none() {
                                #effect_quote
                            }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr && lang.is_none() {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    //     TODO: Set in Set-Cookie header?
    #[cfg(all(feature = "ssr", feature = "actix"))]
    let sync_language_with_cookie_quote = quote! {};

    //   Axum
    #[cfg(all(feature = "ssr", feature = "axum"))]
    let initial_language_from_cookie_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            if let Some(req) = ::leptos::use_context::<::axum::http::request::Parts>() {
                let maybe_cookie = req
                    .headers
                    .get(::axum::http::header::COOKIE)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|cookie| {
                        let cookie = cookie.split(';').find(|c| c.starts_with(#cookie_name_quote));
                        cookie.map(|c| c.split('=').nth(1).unwrap().to_string())
                    });

                if let Some(cookie) = maybe_cookie {
                    if let Some(l) = ::leptos_fluent::l(&cookie, &LANGUAGES) {
                        lang = Some(l);
                        #initial_language_from_cookie_to_server_function_quote
                    }
                }
            }
        };

        initial_language_from_cookie
            .iter()
            .map(|param| {
                let quote = match param.lit {
                    Some(ref lit) => match lit.value {
                        true => quote! {
                            if lang.is_none() {
                                #effect_quote
                            }
                        },
                        false => quote!(),
                    },
                    None => match param.expr {
                        Some(ref expr) => quote! {
                            if #expr && lang.is_none() {
                                #effect_quote
                            }
                        },
                        None => quote!(),
                    },
                };

                match quote.is_empty() {
                    true => quote!(),
                    false => match param.exprpath {
                        Some(ref path) => quote!(#path{#quote}),
                        None => quote,
                    },
                }
            })
            .collect()
    };

    //     TODO: Set in Set-Cookie header?
    #[cfg(all(feature = "ssr", feature = "axum"))]
    let sync_language_with_cookie_quote = quote! {};

    //   Other SSR frameworks or the user is not using any
    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let initial_language_from_cookie_quote = quote! {};
    #[cfg(all(
        not(feature = "actix"),
        not(feature = "axum"),
        feature = "ssr"
    ))]
    {
        _ = initial_language_from_cookie_to_server_function;
    };

    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let sync_language_with_cookie_quote = quote! {};

    let initial_language_quote = {
        #[cfg(not(feature = "ssr"))]
        quote! {
            #initial_language_from_server_function_quote
            #initial_language_from_data_file_quote
            #initial_language_from_system_quote
            #initial_language_from_url_param_quote
            #initial_language_from_url_path_quote
            #initial_language_from_cookie_quote
            #initial_language_from_localstorage_quote
            #initial_language_from_navigator_quote
        }

        #[cfg(feature = "ssr")]
        quote! {
            #initial_language_from_server_function_quote
            #initial_language_from_url_param_quote
            #initial_language_from_url_path_quote
            #initial_language_from_cookie_quote
            #initial_language_from_accept_language_header_quote
        }
    };

    let translations_quote = {
        let loader::Translations { simple, compound } = translations;

        quote! {
            {
                let mut all_loaders = Vec::new();
                all_loaders.extend([#(& #simple),*]);
                #(
                    all_loaders.extend(#compound.iter());
                );*

                all_loaders
            }
        }
    };

    let leptos_fluent_provide_meta_context_quote: proc_macro2::TokenStream = {
        let bool_param = |boolean: &Option<syn::LitBool>,
                          expr: &Option<syn::Expr>|
         -> proc_macro2::TokenStream {
            match boolean {
                Some(ref lit) => quote! { #lit },
                None => match expr {
                    Some(ref expr) => quote! { #expr },
                    None => quote! { false },
                },
            }
        };

        let lit_bool_exprs =
            |params: &[LitBoolExpr]| -> proc_macro2::TokenStream {
                if params.is_empty() {
                    return quote! { false };
                }

                params
                    .iter()
                    .map(|param| {
                        let quote = bool_param(&param.lit, &param.expr);
                        match quote.is_empty() {
                            true => quote!(),
                            false => match param.exprpath {
                                Some(ref path) => {
                                    quote!(#path{#quote})
                                }
                                None => quote,
                            },
                        }
                    })
                    .collect()
            };

        let maybe_litstr_param =
            |lit: &Option<String>| -> proc_macro2::TokenStream {
                match lit {
                    Some(ref lit) => quote! { #lit },
                    None => quote! { None },
                }
            };

        let maybe_some_litstr_param =
            |lit: &Option<String>| -> proc_macro2::TokenStream {
                match lit {
                    Some(ref lit) => quote! { Some(#lit) },
                    None => quote! { None },
                }
            };

        let litstr_or_default = |lit: &Option<syn::LitStr>,
                                 expr: &Option<syn::Expr>,
                                 default: &'static str|
         -> proc_macro2::TokenStream {
            match lit {
                Some(ref lit) => quote! { #lit },
                None => match expr {
                    Some(ref expr) => quote! { #expr },
                    None => quote! { #default },
                },
            }
        };

        let identifiers_as_bools =
            |params: &[Identifier]| -> proc_macro2::TokenStream {
                if params.is_empty() {
                    return quote! { false };
                }

                params
                    .iter()
                    .map(|param| {
                        let quote = match param.ident {
                            Some(_) => quote! { true },
                            None => quote! { false },
                        };

                        match param.exprpath {
                            Some(ref path) => quote!(#path{#quote}),
                            None => quote,
                        }
                    })
                    .collect()
            };

        provide_meta_context.iter().map(|param| {
            let provide_meta_context_value = match param.lit {
                Some(ref lit) => lit.value(),
                None => false,
            };

            match provide_meta_context_value {
                true => {
                    let core_locales_quote = maybe_litstr_param(&core_locales_path);
                    let languages_quote =
                        maybe_some_litstr_param(&raw_languages_path);
                    let check_translations_quote =
                        maybe_some_litstr_param(&check_translations);
                    let sync_html_tag_lang_quote =
                        lit_bool_exprs(&sync_html_tag_lang);
                    let sync_html_tag_dir_quote =
                        lit_bool_exprs(&sync_html_tag_dir);
                    let url_param_quote =
                        litstr_or_default(&url_param.lit, &url_param.expr, "lang");
                    let initial_language_from_url_param_quote =
                        lit_bool_exprs(&initial_language_from_url_param);
                    let initial_language_from_url_param_to_localstorage =
                        lit_bool_exprs(
                            &initial_language_from_url_param_to_localstorage,
                        );
                    let initial_language_from_url_param_to_cookie_quote =
                        lit_bool_exprs(&initial_language_from_url_param_to_cookie);
                    let initial_language_from_url_param_to_server_function_quote =
                        identifiers_as_bools(
                            &initial_language_from_url_param_to_server_function,
                        );
                    let set_language_to_url_param_quote =
                        lit_bool_exprs(&set_language_to_url_param);
                    let localstorage_key_quote = litstr_or_default(
                        &localstorage_key.lit,
                        &localstorage_key.expr,
                        "lang",
                    );
                    let initial_language_from_localstorage_quote =
                        lit_bool_exprs(&initial_language_from_localstorage);
                    let initial_language_from_localstorage_to_cookie_quote =
                        lit_bool_exprs(
                            &initial_language_from_localstorage_to_cookie,
                        );
                    let initial_language_from_localstorage_to_server_function_quote =
                        identifiers_as_bools(
                            &initial_language_from_localstorage_to_server_function,
                        );
                    let set_language_to_localstorage_quote =
                        lit_bool_exprs(&set_language_to_localstorage);
                    let initial_language_from_navigator_quote =
                        lit_bool_exprs(&initial_language_from_navigator);
                    let initial_language_from_navigator_to_localstorage_quote =
                        lit_bool_exprs(
                            &initial_language_from_navigator_to_localstorage,
                        );
                    let initial_language_from_navigator_to_cookie_quote =
                        lit_bool_exprs(&initial_language_from_navigator_to_cookie);
                    let initial_language_from_navigator_to_server_function_quote =
                        identifiers_as_bools(
                            &initial_language_from_navigator_to_server_function,
                        );
                    let set_language_from_navigator_quote =
                        lit_bool_exprs(&set_language_from_navigator);
                    let initial_language_from_accept_language_header_quote =
                        lit_bool_exprs(
                            &initial_language_from_accept_language_header,
                        );
                    let cookie_name_quote = litstr_or_default(
                        &cookie_name.lit,
                        &cookie_name.expr,
                        "lf-lang",
                    );
                    let cookie_attrs_quote =
                        litstr_or_default(&cookie_attrs.lit, &cookie_attrs.expr, "");
                    let initial_language_from_cookie_quote =
                        lit_bool_exprs(&initial_language_from_cookie);
                    let initial_language_from_cookie_to_localstorage_quote =
                        lit_bool_exprs(
                            &initial_language_from_cookie_to_localstorage,
                        );
                    let initial_language_from_cookie_to_server_function_quote =
                        identifiers_as_bools(
                            &initial_language_from_cookie_to_server_function,
                        );
                    let set_language_to_cookie_quote =
                        lit_bool_exprs(&set_language_to_cookie);
                    let initial_language_from_server_function_quote =
                        identifiers_as_bools(
                            &initial_language_from_server_function,
                        );
                    let initial_language_from_server_function_to_cookie_quote =
                        lit_bool_exprs(
                            &initial_language_from_server_function_to_cookie,
                        );
                    let initial_language_from_server_function_to_localstorage_quote =
                        lit_bool_exprs(
                            &initial_language_from_server_function_to_localstorage,
                        );
                    let set_language_to_server_function_quote =
                        identifiers_as_bools(&set_language_to_server_function);
                    let url_path_quote = if url_path.is_some() {quote!{true}} else {quote!{false}};
                    let initial_language_from_url_path_quote =
                        lit_bool_exprs(&initial_language_from_url_path);
                    let initial_language_from_url_path_to_cookie_quote =
                        lit_bool_exprs(&initial_language_from_url_path_to_cookie);
                    let initial_language_from_url_path_to_localstorage_quote =
                        lit_bool_exprs(
                            &initial_language_from_url_path_to_localstorage,
                        );
                    let initial_language_from_url_path_to_server_function_quote =
                        identifiers_as_bools(
                            &initial_language_from_url_path_to_server_function,
                        );

                    let system_quote = {
                        #[cfg(not(feature = "system"))]
                        quote! {}

                        #[cfg(feature = "system")]
                        {
                            let initial_language_from_system_quote =
                                lit_bool_exprs(&initial_language_from_system);
                            let initial_language_from_data_file_quote =
                                lit_bool_exprs(&initial_language_from_data_file);
                            let initial_language_from_system_to_data_file_quote =
                                lit_bool_exprs(
                                    &initial_language_from_system_to_data_file,
                                );
                            let set_language_to_data_file_quote =
                                lit_bool_exprs(&set_language_to_data_file);
                            let data_file_key_quote = litstr_or_default(
                                &data_file_key.lit,
                                &data_file_key.expr,
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
                            initial_language_from_url_param_to_server_function: #initial_language_from_url_param_to_server_function_quote,
                            set_language_to_url_param: #set_language_to_url_param_quote,
                            localstorage_key: #localstorage_key_quote,
                            initial_language_from_localstorage: #initial_language_from_localstorage_quote,
                            initial_language_from_localstorage_to_cookie: #initial_language_from_localstorage_to_cookie_quote,
                            initial_language_from_localstorage_to_server_function: #initial_language_from_localstorage_to_server_function_quote,
                            set_language_to_localstorage: #set_language_to_localstorage_quote,
                            initial_language_from_navigator: #initial_language_from_navigator_quote,
                            initial_language_from_navigator_to_localstorage: #initial_language_from_navigator_to_localstorage_quote,
                            initial_language_from_navigator_to_cookie: #initial_language_from_navigator_to_cookie_quote,
                            initial_language_from_navigator_to_server_function: #initial_language_from_navigator_to_server_function_quote,
                            set_language_from_navigator: #set_language_from_navigator_quote,
                            initial_language_from_accept_language_header: #initial_language_from_accept_language_header_quote,
                            cookie_name: #cookie_name_quote,
                            cookie_attrs: #cookie_attrs_quote,
                            initial_language_from_cookie: #initial_language_from_cookie_quote,
                            initial_language_from_cookie_to_localstorage: #initial_language_from_cookie_to_localstorage_quote,
                            initial_language_from_cookie_to_server_function: #initial_language_from_cookie_to_server_function_quote,
                            set_language_to_cookie: #set_language_to_cookie_quote,
                            initial_language_from_server_function: #initial_language_from_server_function_quote,
                            initial_language_from_server_function_to_cookie: #initial_language_from_server_function_to_cookie_quote,
                            initial_language_from_server_function_to_localstorage: #initial_language_from_server_function_to_localstorage_quote,
                            set_language_to_server_function: #set_language_to_server_function_quote,
                            url_path: #url_path_quote,
                            initial_language_from_url_path: #initial_language_from_url_path_quote,
                            initial_language_from_url_path_to_cookie: #initial_language_from_url_path_to_cookie_quote,
                            initial_language_from_url_path_to_localstorage: #initial_language_from_url_path_to_localstorage_quote,
                            initial_language_from_url_path_to_server_function: #initial_language_from_url_path_to_server_function_quote,
                            provide_meta_context: true,
                            #system_quote
                        };
                        ::leptos::provide_context::<::leptos_fluent::LeptosFluentMeta>(meta);
                    };

                    match param.exprpath {
                        Some(ref path) => quote! { #path{#quote}; },
                        None => quote,
                    }
                }
                false => quote!(),
            }
        }).collect()
    };

    let other_quotes = quote! {
        #sync_html_tag_quote
        #sync_language_with_server_function_quote
        #sync_language_with_localstorage_quote
        #sync_language_with_url_param_quote
        #sync_language_with_cookie_quote
        #sync_language_with_data_file_quote
        #set_language_from_navigator_quote
        #files_tracker_quote
        #leptos_fluent_provide_meta_context_quote
    };

    let debug_quote = quote! {
        const LANGUAGES: [&::leptos_fluent::Language; #n_languages] =
            #languages_quote;

        let mut lang: Option<&'static ::leptos_fluent::Language> = None;
        #initial_language_quote;

        let initial_lang = if let Some(l) = lang {
            l
        } else {
            LANGUAGES[0]
        };

        let translations = ::std::rc::Rc::new(#translations_quote);
        let mut i18n = ::leptos_fluent::I18n {
            language: ::leptos::create_rw_signal(initial_lang),
            languages: &LANGUAGES,
            translations: ::leptos::Signal::derive(move || ::std::rc::Rc::clone(&translations)),
        };
        ::leptos::provide_context::<::leptos_fluent::I18n>(i18n);
    };

    #[cfg(feature = "tracing")]
    tracing::trace!("{}", debug_quote);

    let quote = quote! {
        {
            #debug_quote
            #other_quotes
            i18n
        }
    };

    #[cfg(feature = "debug")]
    debug(&format!("\n{}", &quote.to_string()));

    proc_macro::TokenStream::from(quote)
}

#[cfg(test)]
mod test {
    use trybuild;

    #[test]
    fn ui_pass() {
        let t = trybuild::TestCases::new();
        t.pass("tests/ui/leptos_fluent/pass/*.rs");
    }

    #[test]
    fn ui_fail() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/leptos_fluent/fail/*.rs");
    }

    #[test]
    fn main_and_macros_package_versions_match() {
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
