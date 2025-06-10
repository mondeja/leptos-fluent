#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(track_path))]

//! Macros for [leptos-fluent].
//!
//! [leptos-fluent]: https://crates.io/crates/leptos-fluent

extern crate proc_macro;

pub(crate) mod cookie;
mod exprpath;
mod files_tracker;
#[cfg(not(feature = "ssr"))]
pub(crate) mod fluent_entries;
mod fluent_resources;
mod languages;
mod loader;
#[cfg(not(feature = "ssr"))]
pub(crate) mod tr_macros;
#[cfg(not(feature = "ssr"))]
mod translations_checker;
#[cfg(not(feature = "ssr"))]
mod translations_filler;

pub(crate) use exprpath::evaluate_exprpath;
use files_tracker::build_files_tracker_quote;
#[cfg(not(feature = "ssr"))]
pub(crate) use fluent_resources::FluentResources;
pub(crate) use fluent_resources::{
    build_fluent_resources_and_file_paths, FluentFilePaths,
};
use languages::build_languages_quote;
pub(crate) use languages::ParsedLanguage;
use loader::{I18nLoader, LitBoolExprOrIdent, TokenStreamStr};
use quote::{quote, ToTokens};

#[cfg(feature = "debug")]
#[inline(always)]
pub(crate) fn debug(msg: &str) {
    #[allow(clippy::print_stdout)]
    {
        println!("[leptos-fluent/debug] {msg}");
    };
}

/// Create the i18n context for internationalization.
///
/// [Reference](https://mondeja.github.io/leptos-fluent/latest/leptos_fluent.html)
///
/// # Example
///
/// ```rust,ignore
/// use fluent_templates::static_loader;
/// use leptos::prelude::*;
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
/// pub fn I18n(children: Children) -> impl IntoView {
///     leptos_fluent! {
///         children: children(),
///         translations: [TRANSLATIONS],
///         default_language: "en-US",
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
///     }
/// }
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <I18n>
///             <LanguageSelector/>
///         </I18n>
///     }
/// }
/// ```
///
/// See the reference with all the parameters explained in detail at
/// <https://mondeja.github.io/leptos-fluent/latest/leptos_fluent.html>
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[proc_macro]
pub fn leptos_fluent(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let I18nLoader {
        fluent_file_paths,
        children,
        translations,
        languages,
        languages_path,
        raw_languages_path,
        locales_path,
        core_locales_path,
        default_language,
        check_translations,
        fill_translations,
        provide_meta_context,
        sync_html_tag_lang,
        sync_html_tag_dir,
        initial_language_from_url_param,
        url_param,
        initial_language_from_url_param_to_localstorage,
        initial_language_from_url_param_to_sessionstorage,
        initial_language_from_url_param_to_cookie,
        initial_language_from_url_param_to_server_function,
        set_language_to_url_param,
        localstorage_key,
        initial_language_from_localstorage,
        initial_language_from_localstorage_to_cookie,
        initial_language_from_localstorage_to_sessionstorage,
        initial_language_from_localstorage_to_server_function,
        set_language_to_localstorage,
        sessionstorage_key,
        initial_language_from_sessionstorage,
        initial_language_from_sessionstorage_to_cookie,
        initial_language_from_sessionstorage_to_localstorage,
        initial_language_from_sessionstorage_to_server_function,
        set_language_to_sessionstorage,
        initial_language_from_navigator,
        initial_language_from_navigator_to_localstorage,
        initial_language_from_navigator_to_sessionstorage,
        initial_language_from_navigator_to_cookie,
        initial_language_from_navigator_to_server_function,
        initial_language_from_accept_language_header,
        set_language_from_navigator,
        cookie_name,
        cookie_attrs,
        initial_language_from_cookie,
        initial_language_from_cookie_to_localstorage,
        initial_language_from_cookie_to_sessionstorage,
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
        initial_language_from_url_path_to_sessionstorage,
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
        (::leptos::prelude::expect_context::<::leptos_fluent::I18n>())()
    };

    #[cfg(all(feature = "nightly", not(feature = "ssr")))]
    let set_language_quote = quote! {
        (::leptos::prelude::expect_context::<::leptos_fluent::I18n>())(l)
    };

    #[cfg(not(feature = "nightly"))]
    let get_language_quote = quote! {
        ::leptos::prelude::expect_context::<::leptos_fluent::I18n>().language.get()
    };

    #[cfg(all(not(feature = "nightly"), not(feature = "ssr")))]
    let set_language_quote = quote! {
        ::leptos::prelude::expect_context::<::leptos_fluent::I18n>().language.set(l)
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

    #[cfg(not(feature = "ssr"))]
    let localstorage_key_quote = match localstorage_key.lit {
        Some(ref lit) => quote! { #lit },
        None => match localstorage_key.expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    #[cfg(not(feature = "ssr"))]
    let sessionstorage_key_quote = match sessionstorage_key.lit {
        Some(ref lit) => quote! { #lit },
        None => match sessionstorage_key.expr {
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
                match param.expr {
                    Some(ref expr) => {
                        let q = quote! {
                            if lang.is_none() && #expr && !#data_file_key_quote.is_empty() {
                                #effect_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
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
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
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
                let set_language_to_data_file_quote = match param.expr {
                    Some(ref expr) => quote! {
                        if #expr {
                            #data_file_key_quote
                        } else {
                            ""
                        }
                    },
                    None => quote! { "" },
                };

                // TODO: optimize checking if empty at compile time when literal
                let effect_quote = quote! {
                    ::leptos::prelude::Effect::new(move |_| {
                        if #set_language_to_data_file_quote.is_empty() {
                            return;
                        }
                        ::leptos_fluent::data_file::set(
                            #set_language_to_data_file_quote,
                            &#get_language_quote.id.to_string(),
                        );
                    });
                };

                match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
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
                let initial_language_from_data_file_quote = match param.expr {
                    Some(ref expr) => quote! {
                        if #expr {
                            #data_file_key_quote
                        } else {
                            ""
                        }
                    },
                    None => quote! { "" },
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

                match param.expr {
                    Some(ref expr) => {
                        let q = quote! {
                            if #expr && lang.is_none() {
                                #effect_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
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
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
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
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
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
                let ident = &param.expr;
                let effect_quote = quote! {
                    ::leptos::task::spawn(async move {
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

                match param.expr {
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
            let ident = &param.expr;
            let effect_quote = quote! {
                ::leptos::prelude::Effect::new(move |_| {
                    ::leptos::task::spawn(async {
                        _ = #ident(#get_language_quote.id.to_string()).await;
                    });
                });
            };

            match param.expr {
                Some(_) => match param.exprpath {
                    Some(ref path) => quote! { #path{#effect_quote} },
                    None => effect_quote,
                },
                None => quote!(),
            }
        }).collect();

    let initial_language_from_url_path_to_server_function_quote: proc_macro2::TokenStream =
        initial_language_from_url_path_to_server_function.iter().map(|param| {
            match param.expr {
                Some(ref ident) => {
                    let quote = quote! {
                        ::leptos::task::spawn(async move {
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

                            match param.expr {
                                Some(ref expr) => {
                                    let q = quote! {
                                        if #expr {
                                            #effect_quote
                                        }
                                    };
                                    match param.exprpath {
                                        Some(ref path) => quote!(#path{#q}),
                                        None => q,
                                    }
                                },
                                None => quote!(),
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

                            match param.expr {
                                Some(ref expr) => {
                                    let q = quote! {
                                        if #expr {
                                            #effect_quote
                                        }
                                    };
                                    match param.exprpath {
                                        Some(ref path) => quote!(#path{#q}),
                                        None => q,
                                    }
                                },
                                None => quote!(),
                            }
                        }).collect();

                    let to_sessionstorage_effect_quote: proc_macro2::TokenStream =
                        initial_language_from_url_path_to_sessionstorage.iter().map(|param| {
                            let effect_quote = quote! {
                                ::leptos_fluent::sessionstorage::set(
                                    #localstorage_key_quote,
                                    &l.id.to_string()
                                );
                            };

                            match param.expr {
                                Some(ref expr) => {
                                    let q = quote! {
                                        if #expr {
                                            #effect_quote
                                        }
                                    };
                                    match param.exprpath {
                                        Some(ref path) => quote!(#path{#q}),
                                        None => q,
                                    }
                                },
                                None => quote!(),
                            }
                        }).collect();

                    quote! {
                        if let Some(url_path) = ::leptos_fluent::url::path::get() {
                            lang = ::leptos_fluent::l(#ident(&url_path), &LANGUAGES);
                            if let Some(l) = lang {
                                #to_cookie_effect_quote
                                #to_localstorage_effect_quote
                                #to_sessionstorage_effect_quote
                                #initial_language_from_url_path_to_server_function_quote
                            }
                        }
                    }
                };

                #[cfg(all(feature = "ssr", feature = "actix"))]
                let effect_quote = quote! {
                    if let Some(req) = ::leptos::prelude::use_context::<::actix_web::HttpRequest>() {
                        lang = ::leptos_fluent::l(#ident(&req.path()), &LANGUAGES);
                        if let Some(l) = lang {
                            #initial_language_from_url_path_to_server_function_quote
                        }
                    }
                };

                #[cfg(all(feature = "ssr", feature = "axum"))]
                let effect_quote = quote! {
                    if let Some(req) = ::leptos::prelude::use_context::<::axum::http::request::Parts>() {
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
        // TODO: handle other attributes in Leptos v0.7
        // See https://github.com/leptos-rs/leptos/issues/2856

        let sync_html_tag_lang_bool_quote: proc_macro2::TokenStream = {
            let quote = sync_html_tag_lang
                .iter()
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote! { #expr };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote! { false },
                })
                .collect::<proc_macro2::TokenStream>();

            match quote.is_empty() {
                true => quote! { false },
                false => quote! { #quote },
            }
        };

        let sync_html_tag_dir_bool_quote: proc_macro2::TokenStream = {
            let quote = sync_html_tag_dir
                .iter()
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote! { #expr };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote! { false },
                })
                .collect::<proc_macro2::TokenStream>();

            match quote.is_empty() {
                true => quote! { false },
                false => quote! { #quote },
            }
        };

        let attr_lang_quote =
            match sync_html_tag_lang_bool_quote.to_string() == "false" {
                true => quote! {},
                false => {
                    quote! { attr:lang=|| {#get_language_quote.id.to_string()} }
                }
            };
        let attr_dir_quote = match sync_html_tag_dir_bool_quote.to_string()
            == "false"
        {
            true => quote! {},
            false => quote! { attr:dir=|| {#get_language_quote.dir.as_str()} },
        };

        match attr_lang_quote.is_empty() && attr_dir_quote.is_empty() {
            true => quote! {},
            false => quote! {{
                use ::leptos_fluent::leptos_meta::{provide_meta_context, Html};
                provide_meta_context();
                ::leptos::prelude::view! {
                    <Html #attr_lang_quote #attr_dir_quote/>
                }
            }},
        }
    };

    let url_param_quote = match url_param.lit {
        Some(ref lit) => quote! { #lit },
        None => match url_param.expr {
            Some(ref expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    #[cfg(not(feature = "ssr"))]
    let sync_language_with_localstorage_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            ::leptos::prelude::Effect::new(move |_| {
                ::leptos_fluent::localstorage::set(
                    #localstorage_key_quote,
                    &#get_language_quote.id.to_string()
                );
            });
        };

        set_language_to_localstorage
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
            })
            .collect()
    };

    #[cfg(feature = "ssr")]
    let sync_language_with_localstorage_quote = quote!();

    #[cfg(not(feature = "ssr"))]
    let sync_language_with_sessionstorage_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            ::leptos::prelude::Effect::new(move |_| {
                ::leptos_fluent::sessionstorage::set(
                    #sessionstorage_key_quote,
                    &#get_language_quote.id.to_string()
                );
            });
        };

        set_language_to_sessionstorage
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
            })
            .collect()
    };

    #[cfg(feature = "ssr")]
    let sync_language_with_sessionstorage_quote = quote!();

    let initial_language_from_url_param_quote: proc_macro2::TokenStream = {
        #[cfg(feature = "hydrate")]
        let hydrate_rerender_quote = quote! {
            ::leptos::prelude::Effect::new(move |prev: Option<()>| {
                if prev.is_none() {
                    #set_language_quote;
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
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
                })
                .collect()
        };

        #[cfg(not(feature = "ssr"))]
        let set_to_sessionstorage_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::sessionstorage::set(
                    #sessionstorage_key_quote,
                    &l.id.to_string()
                );
            };

            initial_language_from_url_param_to_sessionstorage
                .iter()
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
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
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
                })
                .collect()
        };

        #[cfg(feature = "ssr")]
        {
            _ = initial_language_from_url_param_to_localstorage;
            _ = initial_language_from_url_param_to_sessionstorage;
            _ = initial_language_from_url_param_to_cookie;
        }

        let set_to_server_function_quote: proc_macro2::TokenStream =
            initial_language_from_url_param_to_server_function
                .iter()
                .map(|param| match param.expr {
                    Some(ref ident) => {
                        let quote = quote! {
                            ::leptos::task::spawn(async move {
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
                    #set_to_sessionstorage_quote
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
            if let Some(req) = ::leptos::prelude::use_context::<actix_web::HttpRequest>() {
                let uri_query = req.uri().query().unwrap_or("");
                #lang_parser_quote
            }
        };

        #[cfg(all(feature = "ssr", feature = "axum"))]
        let parse_language_quote = quote! {
            if let Some(req) = ::leptos::prelude::use_context::<::axum::http::request::Parts>() {
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
            .map(|param| match param.expr {
                Some(ref expr) => match parse_language_quote.is_empty() {
                    true => quote!(),
                    false => {
                        let q = quote! {
                            if #expr {
                                #parse_language_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                },
                None => quote!(),
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
                match param.expr {
                    Some(ref expr) => {
                        let q = quote! {
                            if #expr {
                                #set_cookie_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect();

        let set_sessionstorage_quote = quote! {
            ::leptos_fluent::sessionstorage::set(
                #sessionstorage_key_quote,
                &l.id.to_string()
            );
        };

        let initial_language_from_localstorage_to_sessionstorage_quote: proc_macro2::TokenStream =
            initial_language_from_localstorage_to_sessionstorage.iter().map(|param| {
                match param.expr {
                    Some(ref expr) => {
                        let q = quote! {
                            if #expr {
                                #set_sessionstorage_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect();

        let initial_language_from_localstorage_to_server_function_quote: proc_macro2::TokenStream =
            initial_language_from_localstorage_to_server_function.iter().map(|param| {
                match param.expr {
                    Some(ref ident) => {
                        let quote = quote! {
                            ::leptos::task::spawn(async move {
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
                    #initial_language_from_localstorage_to_sessionstorage_quote
                    #initial_language_from_localstorage_to_server_function_quote
                }
            }
        };

        initial_language_from_localstorage
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #localstorage_get_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
            })
            .collect()
    };

    #[cfg(not(feature = "ssr"))]
    let initial_language_from_sessionstorage_quote: proc_macro2::TokenStream = {
        let set_cookie_quote = quote! {
            ::leptos_fluent::cookie::set(
                #cookie_name_quote,
                &l.id.to_string(),
                &#cookie_attrs_quote
            );
        };

        let initial_language_from_sessionstorage_to_cookie_quote: proc_macro2::TokenStream =
            initial_language_from_sessionstorage_to_cookie.iter().map(|param| {
                match param.expr {
                    Some(ref expr) => {
                        let q = quote! {
                            if #expr {
                                #set_cookie_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect();

        let set_localstorage_quote = quote! {
            ::leptos_fluent::localstorage::set(
                #localstorage_key_quote,
                &l.id.to_string()
            );
        };

        let initial_language_from_sessionstorage_to_localstorage_quote: proc_macro2::TokenStream =
            initial_language_from_sessionstorage_to_localstorage.iter().map(|param| {
                match param.expr {
                    Some(ref expr) => {
                        let q = quote! {
                            if #expr {
                                #set_localstorage_quote
                            }
                        };
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect();

        let initial_language_from_sessionstorage_to_server_function_quote: proc_macro2::TokenStream =
            initial_language_from_sessionstorage_to_server_function.iter().map(|param| {
                match param.expr {
                    Some(ref ident) => {
                        let quote = quote! {
                            ::leptos::task::spawn(async move {
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

        let sessionstorage_get_quote = quote! {
            if let Some(l) = ::leptos_fluent::sessionstorage::get(#sessionstorage_key_quote)
            {
                lang = ::leptos_fluent::l(&l, &LANGUAGES);
                if let Some(l) = lang {
                    #initial_language_from_sessionstorage_to_cookie_quote
                    #initial_language_from_sessionstorage_to_localstorage_quote
                    #initial_language_from_sessionstorage_to_server_function_quote
                }
            }
        };

        initial_language_from_sessionstorage
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #sessionstorage_get_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
            })
            .collect()
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_localstorage;
        _ = initial_language_from_localstorage_to_cookie;
        _ = initial_language_from_localstorage_to_sessionstorage;
        _ = initial_language_from_localstorage_to_server_function;
        _ = initial_language_from_sessionstorage;
        _ = initial_language_from_sessionstorage_to_cookie;
        _ = initial_language_from_sessionstorage_to_localstorage;
        _ = initial_language_from_sessionstorage_to_server_function;
    }

    let sync_language_with_url_param_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            ::leptos::prelude::Effect::new(move |_| {
                ::leptos_fluent::url::param::set(
                    #url_param_quote,
                    &#get_language_quote.id.to_string()
                );
            });
        };

        set_language_to_url_param
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
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
                match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect()
        };

        let initial_language_from_navigator_to_sessionstorage_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::sessionstorage::set(
                    #sessionstorage_key_quote,
                    &l.id.to_string()
                );
            };

            initial_language_from_navigator_to_sessionstorage.iter().map(|param| {
                match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
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
                match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect()
        };

        let initial_language_from_navigator_to_server_function_quote: proc_macro2::TokenStream =
            initial_language_from_navigator_to_server_function.iter().map(|param| {
                match param.expr {
                    Some(ref ident) => {
                        let quote = quote! {
                            ::leptos::task::spawn(async move {
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
            let languages = ::leptos::prelude::window().navigator().languages().to_vec();
            for raw_language in languages {
                let language = raw_language.as_string();
                if language.is_none() {
                    continue;
                }
                lang = ::leptos_fluent::l(&language.unwrap(), &LANGUAGES);
                if let Some(l) = lang {
                    #initial_language_from_navigator_to_localstorage_quote
                    #initial_language_from_navigator_to_sessionstorage_quote
                    #initial_language_from_navigator_to_cookie_quote
                    #initial_language_from_navigator_to_server_function_quote
                    break;
                }
            }
        };

        initial_language_from_navigator
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #window_navigator_languages_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
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
            #[cfg(feature = "debug")]
            let log_language_quote = quote! {
                ::leptos::logging::log!("[leptos-fluent/debug] Language changed to {:?}", &l);
            };

            #[cfg(not(feature = "debug"))]
            let log_language_quote = quote!();

            let effect_quote = quote! {
                use ::leptos_fluent::web_sys::wasm_bindgen::JsCast;
                let closure: Box<dyn FnMut(_)> = Box::new(
                    move |_: ::leptos_fluent::web_sys::Window| {
                        let languages = ::leptos::prelude::window().navigator().languages().to_vec();
                        for raw_language in languages {
                            let language = raw_language.as_string();
                            if language.is_none() {
                                continue;
                            }
                            let l = ::leptos_fluent::l(&language.unwrap(), &LANGUAGES);
                            #log_language_quote
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
                ::leptos::prelude::window().add_event_listener_with_callback(
                    "languagechange",
                    cb.as_ref().unchecked_ref()
                ).expect("Failed to add event listener for window languagechange");
                cb.forget();
            };

            set_language_from_navigator
                .iter()
                .map(|param| match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    }
                    None => quote!(),
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
            if let Some(req) = ::leptos::prelude::use_context::<::actix_web::HttpRequest>() {
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
            match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                },
                None => quote!(),
            }
        }).collect()
    };

    //   Axum
    #[cfg(all(feature = "axum", feature = "ssr"))]
    let initial_language_from_accept_language_header_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            if let Some(req) = ::leptos::prelude::use_context::<::axum::http::request::Parts>() {
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
            match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                },
                None => quote!(),
            }
        }).collect()
    };

    //   Other SSR framework or the user is not using any
    #[cfg(all(not(feature = "actix"), not(feature = "axum"), feature = "ssr"))]
    let initial_language_from_accept_language_header_quote = quote! {};

    #[cfg(not(feature = "ssr"))]
    {
        _ = initial_language_from_accept_language_header;
    }

    // Cookie
    let initial_language_from_cookie_to_server_function_quote: proc_macro2::TokenStream =
        initial_language_from_cookie_to_server_function.iter().map(|param| {
            match param.expr {
                Some(ref ident) => {
                    let quote = quote! {
                        ::leptos::task::spawn(async move {
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
                match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect()
        };

        let initial_language_from_cookie_to_sessionstorage_quote: proc_macro2::TokenStream = {
            let effect_quote = quote! {
                ::leptos_fluent::sessionstorage::set(
                    #sessionstorage_key_quote,
                    &l.id.to_string()
                );
            };

            initial_language_from_cookie_to_sessionstorage.iter().map(|param| {
                match param.expr {
                    Some(ref expr) => {
                        let q = quote!(if #expr {#effect_quote});
                        match param.exprpath {
                            Some(ref path) => quote!(#path{#q}),
                            None => q,
                        }
                    },
                    None => quote!(),
                }
            }).collect()
        };

        let parse_client_cookie_quote = quote! {
            if let Some(cookie) = ::leptos_fluent::cookie::get(#cookie_name_quote) {
                if let Some(l) = ::leptos_fluent::l(&cookie, &LANGUAGES) {
                    lang = Some(l);
                    #initial_language_from_cookie_to_localstorage_quote
                    #initial_language_from_cookie_to_sessionstorage_quote
                    #initial_language_from_cookie_to_server_function_quote
                }
            }
        };

        initial_language_from_cookie
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #parse_client_cookie_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
            })
            .collect()
    };

    #[cfg(not(feature = "ssr"))]
    let sync_language_with_cookie_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            ::leptos::prelude::Effect::new(move |_| {
                ::leptos_fluent::cookie::set(
                    #cookie_name_quote,
                    &#get_language_quote.id.to_string(),
                    &#cookie_attrs_quote
                );
            });
        };

        set_language_to_cookie
            .iter()
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
            })
            .collect()
    };

    #[cfg(feature = "ssr")]
    {
        _ = initial_language_from_cookie;
        _ = initial_language_from_cookie_to_localstorage;
        _ = initial_language_from_cookie_to_sessionstorage;
        _ = cookie_attrs;
        _ = set_language_to_cookie;
    }

    //   Actix
    #[cfg(all(feature = "ssr", feature = "actix"))]
    let initial_language_from_cookie_quote: proc_macro2::TokenStream = {
        let effect_quote = quote! {
            if let Some(req) = ::leptos::prelude::use_context::<::actix_web::HttpRequest>() {
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
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
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
            if let Some(req) = ::leptos::prelude::use_context::<::axum::http::request::Parts>() {
                let maybe_cookie = req
                    .headers
                    .get(::axum::http::header::COOKIE)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|cookie| {
                        let cookie = cookie.split(';').find(|c| c.trim_start().starts_with(#cookie_name_quote));
                        cookie.map(|c| c.split('=').nth(1).unwrap().trim_start().to_string())
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
            .map(|param| match param.expr {
                Some(ref expr) => {
                    let q = quote! {
                        if #expr && lang.is_none() {
                            #effect_quote
                        }
                    };
                    match param.exprpath {
                        Some(ref path) => quote!(#path{#q}),
                        None => q,
                    }
                }
                None => quote!(),
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
            #initial_language_from_sessionstorage_quote
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

    let leptos_fluent_provide_meta_context_quote: proc_macro2::TokenStream = {
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
                                 expr: &Option<TokenStreamStr>,
                                 default_: &'static str|
         -> proc_macro2::TokenStream {
            match lit {
                Some(ref lit) => quote! { #lit },
                None => match expr {
                    Some(ref expr) => expr.into_token_stream(),
                    None => quote! { #default_ },
                },
            }
        };

        let lit_bool_expr_or_idents =
            |params: &[LitBoolExprOrIdent]| -> proc_macro2::TokenStream {
                if params.is_empty() {
                    return quote! { false };
                }

                params
                    .iter()
                    .map(|param| {
                        let quote = match &param.expr {
                            Some(ident) => match ident
                                .to_token_stream()
                                .to_string()
                                .as_str()
                            {
                                "false" => quote! { false },
                                _ => quote! { true },
                            },
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
            match param.lit.unwrap_or(false) {
                true => {
                    let core_locales_quote = maybe_litstr_param(&core_locales_path);
                    let default_language_quote = match &default_language {
                        Some(ref lang, ..) => {
                            let code = &lang.0;
                            quote!(Some(#code))
                        },
                        None => quote!(None)
                    };
                    let languages_quote =
                        maybe_some_litstr_param(&raw_languages_path);
                    let translations_quote = if translations.is_some() {
                        quote!(true)
                    } else {
                        quote!(false)
                    };
                    let check_translations_quote =
                        maybe_some_litstr_param(&check_translations);
                    let fill_translations_quote =
                        maybe_some_litstr_param(&fill_translations);
                    let sync_html_tag_lang_quote =
                        lit_bool_expr_or_idents(&sync_html_tag_lang);
                    let sync_html_tag_dir_quote =
                        lit_bool_expr_or_idents(&sync_html_tag_dir);
                    let url_param_quote =
                        litstr_or_default(&url_param.lit, &url_param.expr, "lang");
                    let initial_language_from_url_param_quote =
                        lit_bool_expr_or_idents(&initial_language_from_url_param);
                    let initial_language_from_url_param_to_localstorage =
                        lit_bool_expr_or_idents(
                            &initial_language_from_url_param_to_localstorage,
                        );
                    let initial_language_from_url_param_to_sessionstorage =
                        lit_bool_expr_or_idents(
                            &initial_language_from_url_param_to_sessionstorage,
                        );
                    let initial_language_from_url_param_to_cookie_quote =
                        lit_bool_expr_or_idents(&initial_language_from_url_param_to_cookie);
                    let initial_language_from_url_param_to_server_function_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_url_param_to_server_function,
                        );
                    let set_language_to_url_param_quote =
                        lit_bool_expr_or_idents(&set_language_to_url_param);
                    let localstorage_key_quote = litstr_or_default(
                        &localstorage_key.lit,
                        &localstorage_key.expr,
                        "lang",
                    );
                    let initial_language_from_localstorage_quote =
                        lit_bool_expr_or_idents(&initial_language_from_localstorage);
                    let initial_language_from_localstorage_to_cookie_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_localstorage_to_cookie,
                        );
                    let initial_language_from_localstorage_to_sessionstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_localstorage_to_sessionstorage,
                        );
                    let initial_language_from_localstorage_to_server_function_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_localstorage_to_server_function,
                        );
                    let set_language_to_localstorage_quote =
                        lit_bool_expr_or_idents(&set_language_to_localstorage);
                    let sessionstorage_key_quote = litstr_or_default(
                        &sessionstorage_key.lit,
                        &sessionstorage_key.expr,
                        "lang",
                    );
                    let initial_language_from_sessionstorage_quote =
                        lit_bool_expr_or_idents(&initial_language_from_sessionstorage);
                    let initial_language_from_sessionstorage_to_cookie_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_sessionstorage_to_cookie,
                        );
                    let initial_language_from_sessionstorage_to_localstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_sessionstorage_to_localstorage,
                        );
                    let initial_language_from_sessionstorage_to_server_function_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_sessionstorage_to_server_function,
                        );
                    let set_language_to_sessionstorage_quote =
                        lit_bool_expr_or_idents(&set_language_to_sessionstorage);
                    let initial_language_from_navigator_quote =
                        lit_bool_expr_or_idents(&initial_language_from_navigator);
                    let initial_language_from_navigator_to_localstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_navigator_to_localstorage,
                        );
                    let initial_language_from_navigator_to_sessionstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_navigator_to_sessionstorage,
                        );
                    let initial_language_from_navigator_to_cookie_quote =
                        lit_bool_expr_or_idents(&initial_language_from_navigator_to_cookie);
                    let initial_language_from_navigator_to_server_function_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_navigator_to_server_function,
                        );
                    let set_language_from_navigator_quote =
                        lit_bool_expr_or_idents(&set_language_from_navigator);
                    let initial_language_from_accept_language_header_quote =
                        lit_bool_expr_or_idents(
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
                        lit_bool_expr_or_idents(&initial_language_from_cookie);
                    let initial_language_from_cookie_to_localstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_cookie_to_localstorage,
                        );
                    let initial_language_from_cookie_to_sessionstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_cookie_to_sessionstorage,
                        );
                    let initial_language_from_cookie_to_server_function_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_cookie_to_server_function,
                        );
                    let set_language_to_cookie_quote =
                        lit_bool_expr_or_idents(&set_language_to_cookie);
                    let initial_language_from_server_function_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_server_function,
                        );
                    let initial_language_from_server_function_to_cookie_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_server_function_to_cookie,
                        );
                    let initial_language_from_server_function_to_localstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_server_function_to_localstorage,
                        );
                    let set_language_to_server_function_quote =
                        lit_bool_expr_or_idents(&set_language_to_server_function);
                    let url_path_quote = if url_path.is_some() {quote!{true}} else {quote!{false}};
                    let initial_language_from_url_path_quote =
                        lit_bool_expr_or_idents(&initial_language_from_url_path);
                    let initial_language_from_url_path_to_cookie_quote =
                        lit_bool_expr_or_idents(&initial_language_from_url_path_to_cookie);
                    let initial_language_from_url_path_to_localstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_url_path_to_localstorage,
                        );
                    let initial_language_from_url_path_to_sessionstorage_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_url_path_to_sessionstorage,
                        );
                    let initial_language_from_url_path_to_server_function_quote =
                        lit_bool_expr_or_idents(
                            &initial_language_from_url_path_to_server_function,
                        );

                    let system_quote = {
                        #[cfg(not(feature = "system"))]
                        quote! {}

                        #[cfg(feature = "system")]
                        {
                            let initial_language_from_system_quote =
                                lit_bool_expr_or_idents(&initial_language_from_system);
                            let initial_language_from_data_file_quote =
                                lit_bool_expr_or_idents(&initial_language_from_data_file);
                            let initial_language_from_system_to_data_file_quote =
                                lit_bool_expr_or_idents(
                                    &initial_language_from_system_to_data_file,
                                );
                            let set_language_to_data_file_quote =
                                lit_bool_expr_or_idents(&set_language_to_data_file);
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
                            default_language: #default_language_quote,
                            translations: #translations_quote,
                            check_translations: #check_translations_quote,
                            fill_translations: #fill_translations_quote,
                            sync_html_tag_lang: #sync_html_tag_lang_quote,
                            sync_html_tag_dir: #sync_html_tag_dir_quote,
                            url_param: #url_param_quote,
                            initial_language_from_url_param: #initial_language_from_url_param_quote,
                            initial_language_from_url_param_to_localstorage: #initial_language_from_url_param_to_localstorage,
                            initial_language_from_url_param_to_sessionstorage: #initial_language_from_url_param_to_sessionstorage,
                            initial_language_from_url_param_to_cookie: #initial_language_from_url_param_to_cookie_quote,
                            initial_language_from_url_param_to_server_function: #initial_language_from_url_param_to_server_function_quote,
                            set_language_to_url_param: #set_language_to_url_param_quote,
                            localstorage_key: #localstorage_key_quote,
                            initial_language_from_localstorage: #initial_language_from_localstorage_quote,
                            initial_language_from_localstorage_to_cookie: #initial_language_from_localstorage_to_cookie_quote,
                            initial_language_from_localstorage_to_sessionstorage: #initial_language_from_localstorage_to_sessionstorage_quote,
                            initial_language_from_localstorage_to_server_function: #initial_language_from_localstorage_to_server_function_quote,
                            set_language_to_localstorage: #set_language_to_localstorage_quote,
                            sessionstorage_key: #sessionstorage_key_quote,
                            initial_language_from_sessionstorage: #initial_language_from_sessionstorage_quote,
                            initial_language_from_sessionstorage_to_cookie: #initial_language_from_sessionstorage_to_cookie_quote,
                            initial_language_from_sessionstorage_to_localstorage: #initial_language_from_sessionstorage_to_localstorage_quote,
                            initial_language_from_sessionstorage_to_server_function: #initial_language_from_sessionstorage_to_server_function_quote,
                            set_language_to_sessionstorage: #set_language_to_sessionstorage_quote,
                            initial_language_from_navigator: #initial_language_from_navigator_quote,
                            initial_language_from_navigator_to_localstorage: #initial_language_from_navigator_to_localstorage_quote,
                            initial_language_from_navigator_to_sessionstorage: #initial_language_from_navigator_to_sessionstorage_quote,
                            initial_language_from_navigator_to_cookie: #initial_language_from_navigator_to_cookie_quote,
                            initial_language_from_navigator_to_server_function: #initial_language_from_navigator_to_server_function_quote,
                            set_language_from_navigator: #set_language_from_navigator_quote,
                            initial_language_from_accept_language_header: #initial_language_from_accept_language_header_quote,
                            cookie_name: #cookie_name_quote,
                            cookie_attrs: #cookie_attrs_quote,
                            initial_language_from_cookie: #initial_language_from_cookie_quote,
                            initial_language_from_cookie_to_localstorage: #initial_language_from_cookie_to_localstorage_quote,
                            initial_language_from_cookie_to_sessionstorage: #initial_language_from_cookie_to_sessionstorage_quote,
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
                            initial_language_from_url_path_to_sessionstorage: #initial_language_from_url_path_to_sessionstorage_quote,
                            initial_language_from_url_path_to_server_function: #initial_language_from_url_path_to_server_function_quote,
                            provide_meta_context: true,
                            #system_quote
                        };
                        ::leptos::context::provide_context::<::leptos_fluent::LeptosFluentMeta>(meta);
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
        #sync_language_with_server_function_quote
        #sync_language_with_localstorage_quote
        #sync_language_with_sessionstorage_quote
        #sync_language_with_url_param_quote
        #sync_language_with_cookie_quote
        #sync_language_with_data_file_quote
        #set_language_from_navigator_quote
        #files_tracker_quote
        #leptos_fluent_provide_meta_context_quote
    };

    let initial_language_index = match default_language {
        Some((_, index)) => index,
        None => 0,
    };

    let (fluent_templates_quote, translations_quote) = match translations {
        Some(ref translations) => (quote!(), quote!(#translations)),
        None => {
            let fallback_language =
                &languages[initial_language_index].0.to_string();

            let core_locales_quote = match &core_locales_path {
                Some(ref path) => quote!(core_locales: #path,),
                None => quote!(),
            };

            #[cfg(feature = "disable-unicode-isolating-marks")]
            let customise_quote = quote! {
                customise: |bundle| bundle.set_use_isolating(false),
            };
            #[cfg(not(feature = "disable-unicode-isolating-marks"))]
            let customise_quote = quote!();

            (
                quote! {
                    use ::leptos_fluent::fluent_templates;
                    ::leptos_fluent::fluent_templates::static_loader! {
                        static TRS = {
                            locales: #locales_path,
                            fallback_language: #fallback_language,
                            #core_locales_quote
                            #customise_quote
                        };
                    }
                },
                quote!(vec![&TRS]),
            )
        }
    };

    let init_quote = quote! {
        {
            let mut lang: Option<&'static ::leptos_fluent::Language> = None;
            #initial_language_quote;

            let initial_lang = if let Some(l) = lang {
                l
            } else {
                LANGUAGES[#initial_language_index]
            };

            #fluent_templates_quote;

            let i18n = ::leptos_fluent::I18n {
                language: ::leptos::prelude::RwSignal::new(initial_lang),
                languages: &LANGUAGES,
                translations: ::leptos::prelude::Signal::derive(move || #translations_quote),
            };
            ::leptos::context::provide_context::<::leptos_fluent::I18n>(i18n);
            i18n
        }
    };

    let children_quote: proc_macro2::TokenStream = children
        .iter()
        .map(|param| {
            let expr = param.expr.as_ref().unwrap();
            match param.exprpath {
                Some(ref path) => quote!(#path{#expr}),
                None => quote!(#expr),
            }
        })
        .collect();

    let quote = quote! {
        let i18n = {
            const LANGUAGES: [&::leptos_fluent::Language; #n_languages] =
                #languages_quote;
            let i18n = #init_quote;
            #other_quotes
            i18n
        };
        {
            use ::leptos::context::Provider;
            ::leptos::prelude::view! {
                <Provider value={i18n}>
                    #sync_html_tag_quote
                    {#children_quote}
                </Provider>
            }
        }
    };

    #[cfg(feature = "debug")]
    debug(&format!("\n{}", &quote.to_string()));

    #[cfg(feature = "tracing")]
    tracing::trace!("{}", &quote.to_string());

    proc_macro::TokenStream::from(quote)
}

#[cfg(test)]
mod tests {
    use trybuild;

    #[test]
    fn leptos_fluent_trybuild_pass() {
        let t = trybuild::TestCases::new();

        #[cfg(feature = "nightly")]
        t.pass("tests/ui/leptos_fluent/nightly/pass/*.rs");

        // some tests are flaky on nightly
        #[cfg(not(feature = "nightly"))]
        {
            t.pass("tests/ui/leptos_fluent/nightly/pass/*.rs");
            t.pass("tests/ui/leptos_fluent/stable/pass/*.rs");
        }
    }

    #[cfg(not(feature = "nightly"))]
    #[test]
    fn leptos_fluent_trybuild_fail() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/leptos_fluent/stable/fail/*.rs");
    }
}
