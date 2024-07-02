#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Graphical user interface for leptos-fluent.
//!
//! See [leptos-fluent] for more information.
//!
//! [leptos-fluent]: https://crates.io/crates/leptos-fluent
//!

mod projects;

use fluent_templates::static_loader;
use leptos::*;
use crate::leptos_fluent;
use leptos_meta::provide_meta_context;
use projects::Projects;

#[cfg(feature = "gui")]
#[macro_export]
#[doc(hidden)]
macro_rules! ctr {
    ($text_id:literal$(,)?) => {$crate::tr_impl($text_id)};
    ($text_id:literal, {$($key:literal => $value:expr),*$(,)?}$(,)?) => {{
        $crate::tr_with_args_impl($text_id, &{
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.into());
            )*
            map
        })
    }}
}

#[cfg(feature = "gui")]
#[macro_export]
#[doc(hidden)]
macro_rules! move_ctr {
    ($text_id:literal$(,)?) => {
        ::leptos::Signal::derive(move || $crate::ctr!($text_id))
    };
    ($text_id:literal, {$($key:literal => $value:expr),*$(,)?}$(,)?) => {
        ::leptos::Signal::derive(move || $crate::ctr!($text_id, {
            $(
                $key => $value,
            )*
        }))
    };
}

static_loader! {
    static TRANSLATIONS = {
        locales: "./src/gui/locales",
        fallback_language: "en",
    };
}

/// Main entry point for the GUI.
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
#[component]
pub fn LeptosFluentGui() -> impl IntoView {
    provide_meta_context();

    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "./src/gui/locales",
        leptos_fluent_prefix: crate,
        #[cfg(debug_assertions)]
        check_translations: "./src/gui/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; max-age=1209600",  // one week
        initial_language_from_cookie: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_cookie: true,
        initial_language_from_accept_language_header: true,
        initial_language_from_url_param: true,
        url_param: "lang",
        initial_language_from_url_param_to_cookie: true,
        #[cfg(feature = "system")]
        data_file_key: "leptos-fluent-gui",
    }};

    view! {
        <style>{include_str!("./mod.css")}</style>
        <div id="app">
            <Projects/>
            <Main/>
        </div>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! {
        <lf-main>
            <h1>Main</h1>
        </lf-main>
    }
}
