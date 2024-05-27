//! [![Crates.io](https://img.shields.io/crates/v/leptos-fluent)](https://crates.io/crates/leptos-fluent)
//! [![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
//! [![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/ci.yml?label=tests&logo=github)](https://github.com/mondeja/leptos-fluent/actions)
//! [![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)][documentation]
//!
//! Internationalization framework for [Leptos] using [fluent-templates].
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! leptos-fluent = "0.0.26"
//! fluent-templates = "0.9"
//!
//! [features]
//! hydrate = [
//!   "leptos-fluent/hydrate"
//! ]
//! ssr = [
//!   "leptos-fluent/ssr",
//!   "leptos-fluent/actix",  # actix and axum are supported
//! ]
//! ```
//!
//! # Usage
//!
//! Giving the following directory structure:
//!
//! ```plaintext
//! .
//! ├── 📄 Cargo.toml
//! ├── 📁 locales
//! │   ├── 📁 en
//! │   │   └── 📄 main.ftl
//! │   └── 📁 es
//! │       └── 📄 main.ftl
//! └── 📁 src
//!     ├── 📄 main.rs
//!     └── 📄 lib.rs
//! ```
//!
//! With Fluent files _en.ftl_ and _es.ftl_:
//!
//! ```ftl
//! foo = Hello, world!
//! bar = Hello, { $arg1 } and { $arg2 }!
//! ```
//!
//! ```ftl
//! foo = ¡Hola, mundo!
//! bar = ¡Hola, { $arg1 } y { $arg2 }!
//! ```
//!
//! You can use `leptos-fluent` as follows:
//!
//! ```rust,ignore
//! use fluent_templates::static_loader;
//! use leptos::*;
//! use leptos_fluent::{leptos_fluent, tr, move_tr};
//!
//! static_loader! {
//!     static TRANSLATIONS = {
//!         locales: "./locales",
//!         fallback_language: "en",
//!     };
//! }
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     leptos_fluent! {{
//!         // Path to the locales directory, relative to Cargo.toml file.
//!         locales: "./locales",
//!         // Static translations struct provided by fluent-templates.
//!         translations: TRANSLATIONS,
//!
//!         // Client side options (for `csr` and `hydrate`)
//!         // ---------------------------------------------
//!         // Synchronize `<html lang="...">` attribute with the current
//!         // language using `leptos::create_effect`. By default, it is `false`.
//!         sync_html_tag_lang: true,
//!         // URL parameter name to use discovering the initial language
//!         // of the user. By default is `"lang"`.
//!         url_param: "lang",
//!         // Discover the initial language of the user from the URL.
//!         // By default, it is `false`.
//!         initial_language_from_url_param: true,
//!         // Set the discovered initial language of the user from
//!         // the URL in local storage. By default, it is `false`.
//!         initial_language_from_url_param_to_localstorage: true,
//!         // Update the language on URL parameter when using the method
//!         // `I18n.set_language`. By default, it is `false`.
//!         set_language_to_url_param: true,
//!         // Name of the field in local storage to get and set the
//!         // current language of the user. By default, it is `"lang"`.
//!         localstorage_key: "language",
//!         // Get the initial language from local storage if not found
//!         // in an URL param. By default, it is `false`.
//!         initial_language_from_localstorage: true,
//!         // Update the language on local storage when using the method
//!         // `I18n.set_language`. By default, it is `false`.
//!         set_language_to_localstorage: true,
//!         // Get the initial language from `navigator.languages` if not
//!         // found in the local storage. By default, it is `false`.
//!         initial_language_from_navigator: true,
//!
//!         // Server side options (for `ssr`)
//!         // -------------------------------
//!         // Set the initial language from the Accept-Language header of the
//!         // request. By default, it is `false`.
//!         initial_language_from_accept_language_header: true,
//!     }};
//!
//!     view! {
//!         <ChildComponent />
//!     }
//! }
//!
//! #[component]
//! fn ChildComponent() -> impl IntoView {
//!     // Use `tr!` and `move_tr!` macros to translate strings.
//!     view! {
//!         <p>
//!             <span>{move || tr!("foo")}</span>
//!             <span>{move_tr!("bar", {
//!                 "arg1" => "value1",
//!                 "arg2" => "value2",
//!             })}</span>
//!         </p>
//!     }
//! }
//! ```
//!
//! ## Features
//!
//! - **Server Side Rendering**: Use the `leptos-fluent/ssr` feature.
//! - **Hydration**: Use the `leptos-fluent/hydrate` feature.
//! - **Actix Web integration**: Use the `leptos-fluent/actix` feature.
//! - **Axum integration**: Use the `leptos-fluent/axum` feature.
//!
//! # Resources
//!
//! - [Quickstart]
//! - [Examples]
//! - [Documentation]
//!
//! # Roadmap
//!
//! Leptos-fluent is currently ready for most use cases. However, it is still in an
//! early stage of development and the API may contain breaking changes through
//! v0.0.X releases. I'm trying to release the API at v0.1.0 as stable as possible.
//!
//! [leptos]: https://leptos.dev/
//! [fluent-templates]: https://github.com/XAMPPRocky/fluent-templates
//! [quickstart]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.leptos_fluent.html
//! [examples]: https://github.com/mondeja/leptos-fluent/tree/master/examples
//! [documentation]: https://docs.rs/leptos-fluent

#[doc(hidden)]
pub mod http_header;
#[doc(hidden)]
pub mod localstorage;
#[doc(hidden)]
pub mod url;

use core::str::FromStr;
use fluent_templates::{
    fluent_bundle::FluentValue, loader::Loader, LanguageIdentifier,
    StaticLoader,
};
use leptos::{use_context, RwSignal, SignalGet, SignalSet};
pub use leptos_fluent_macros::leptos_fluent;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Each language supported by your application.
#[derive(Clone, Debug)]
pub struct Language {
    /// Language identifier
    ///
    /// Can be any valid language tag, such as `en`, `es`, `en-US`, `es-ES`, etc.
    pub id: LanguageIdentifier,
    /// Language name
    ///
    /// The name of the language, such as `English`, `Español`, etc.
    /// This name will be intended to be displayed in the language selector.
    pub name: &'static str,
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Internationalization context.
///
/// This context is used to provide the current language, the available languages
/// and all the translations. It is capable of doing what is needed to translate
/// and manage translations in a whole application.
///
/// If you need to separate the translations of different parts of the application,
/// you can wrap this context in another struct and provide it to Leptos as a context.
#[derive(Clone, Copy)]
pub struct I18n {
    /// Signal that holds the current language.
    pub language: RwSignal<&'static Language>,
    /// Available languages for the application.
    pub languages: &'static [&'static Language],
    /// leptos-fluent translations loader.
    pub translations: &'static Lazy<StaticLoader>,
    /// Local storage key to store the current language.
    pub localstorage_key: &'static str,
    /// Whether to use local storage to store the current language
    /// when the user changes it.
    pub use_localstorage: bool,
    /// URL parameter to use to store the current language.
    pub use_url_param: Option<&'static str>,
}

impl I18n {
    /// Translate a text identifier to the current language.
    ///
    /// ```rust,ignore
    /// use leptos_fluent::i18n;
    ///
    /// i18n().tr("hello-world")
    /// ```
    pub fn tr(&self, text_id: &str) -> String {
        let lang_id = &self.language.get().id;
        self.translations.lookup(lang_id, text_id)
    }

    /// Translate a text identifier to the current language with arguments.
    ///
    /// ```rust,ignore
    /// use leptos_fluent::i18n;
    /// use std::collections::HashMap;
    ///
    /// i18n().trs("will-be-removed-at", &{
    ///    let mut map = HashMap::new();
    ///    map.insert("icon".to_string(), title().into());
    ///    map.insert("version".to_string(), removal_at_version.into());
    ///    map
    /// })
    /// ```
    pub fn trs(
        &self,
        text_id: &str,
        args: &HashMap<String, FluentValue<'_>>,
    ) -> String {
        let lang_id = &self.language.get().id;
        self.translations.lookup_with_args(lang_id, text_id, args)
    }

    /// Get the default language.
    ///
    /// The default language is the first language in the list of available languages.
    pub fn default_language(&self) -> &'static Language {
        self.languages[0]
    }

    /// Get the language from a language identifier.
    ///
    /// This function will try to match the language identifier with the available
    /// languages. If it doesn't find an exact match, it will try to match the
    /// language identifier without the region. If it doesn't find a match, it will
    /// return `None`.
    pub fn language_from_str(&self, code: &str) -> Option<&'static Language> {
        language_from_str_between_languages(code, self.languages)
    }

    #[doc(hidden)]
    #[inline(always)]
    pub fn set_language_not_use_url_param(&self, lang: &'static Language) {
        self.language.set(lang);
        if self.use_localstorage {
            localstorage::set(self.localstorage_key, &lang.id.to_string());
        }
    }

    /// Set the current language in the signal of the context and in local storage
    /// (if using local storage).
    pub fn set_language(&self, lang: &'static Language) {
        self.set_language_not_use_url_param(lang);
        if let Some(use_url_param) = self.use_url_param {
            url::set(use_url_param, &lang.id.to_string());
        }
    }

    /// Get a hash for a language including their active status.
    pub fn language_key(&self, lang: &'static Language) -> String {
        format!(
            "{}{}",
            lang.id,
            if lang == self.language.get() {
                "1"
            } else {
                "0"
            }
        )
    }

    /// Get wether a language is the active language.
    pub fn is_active_language(&self, lang: &'static Language) -> bool {
        lang == self.language.get()
    }
}

/// Get the current context for localization.
#[inline(always)]
pub fn use_i18n() -> Option<I18n> {
    use_context::<I18n>()
}

/// Expect the current context for localization.
#[inline(always)]
pub fn expect_i18n() -> I18n {
    use_context::<I18n>().expect(
        "I18n context is missing, use the leptos_fluent!{} macro to provide it.",
    )
}

/// Expect the current context for localization.
#[inline(always)]
pub fn i18n() -> I18n {
    use_context::<I18n>().expect(
        "I18n context is missing, use the leptos_fluent!{} macro to provide it.",
    )
}

/// Translate a text identifier to the current language.
///
/// ```rust,ignore
/// tr!("hello-world")
/// tr!("hello-world", { "name" => "John" })
/// ```
#[macro_export]
macro_rules! tr {
    ($text_id:expr$(,)?) => {
        $crate::expect_i18n().tr($text_id)
    };
    ($text_id:expr, {$($key:expr => $value:expr),*$(,)?}$(,)?) => {
        $crate::expect_i18n().trs($text_id, &{
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.into());
            )*
            map
        })
    }
}

/// [`leptos::Signal`] that translates a text identifier to the current language.
///
/// ```rust,ignore
/// move_tr!("hello-world")
/// move_tr!("hello-world", { "name" => "John" })
/// ```
///
/// [`leptos::Signal`]: https://docs.rs/leptos/latest/leptos/struct.Signal.html
#[macro_export]
macro_rules! move_tr {
    ($text_id:expr$(,)?) => {
        ::leptos::Signal::derive(move || $crate::tr!($text_id))
    };
    ($text_id:expr, {$($key:expr => $value:expr),*$(,)?}$(,)?) => {
        ::leptos::Signal::derive(move || $crate::tr!($text_id, {
            $(
                $key => $value,
            )*
        }))
    };
}

#[doc(hidden)]
pub fn language_from_str_between_languages(
    code: &str,
    languages: &'static [&Language],
) -> Option<&'static Language> {
    match LanguageIdentifier::from_str(code) {
        Ok(target_lang) => match languages
            .iter()
            .find(|lang| lang.id.matches(&target_lang, false, false))
        {
            Some(lang) => Some(lang),
            None => {
                let mut lazy_target_lang = target_lang.clone();
                lazy_target_lang.region = None;
                match languages
                    .iter()
                    .find(|lang| lang.id.matches(&lazy_target_lang, true, true))
                {
                    Some(lang) => Some(lang),
                    None => None,
                }
            }
        },
        Err(_) => None,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_readme_leptos_fluent_version_is_updated() {
        let this_file = include_str!("./lib.rs");
        let mut version = None;
        for line in this_file.lines() {
            if line.starts_with("//! leptos-fluent = ") {
                version = Some(
                    line.split("leptos-fluent = \"")
                        .nth(1)
                        .unwrap()
                        .split('"')
                        .next()
                        .unwrap(),
                );
                break;
            }
        }

        assert!(
            version.is_some(),
            r#"leptos-fluent = "<version>" not found in leptos-fluent/src/lib.rs"#
        );
        assert_eq!(
            version.unwrap(),
            env!("CARGO_PKG_VERSION"),
            concat!(
                "The version of leptos-fluent shown in the README at",
                " 'Installation' section is not updated."
            ),
        );
    }
}
