#![deny(missing_docs)]
#![forbid(unsafe_code)]
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
//! hello-world = Hello, world!
//! hello-args = Hello, { $arg1 } and { $arg2 }!
//! ```
//!
//! ```ftl
//! hello-world = ¡Hola, mundo!
//! hello-args = ¡Hola, { $arg1 } y { $arg2 }!
//! ```
//!
//! You can use `leptos-fluent` as follows:
//!
//! ```rust,ignore
//! use fluent_templates::static_loader;
//! use leptos::*;
//! use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr, Language};
//!
//! static_loader! {
//!     static TRANSLATIONS = {
//!         locales: "./locales",
//!         fallback_language: "en",
//!     };
//! }
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     leptos_fluent! {{
//!         // Path to the locales directory, relative to Cargo.toml file.
//!         locales: "./locales",
//!         // Static translations struct provided by fluent-templates.
//!         translations: TRANSLATIONS,
//!
//!         // Client side options
//!         // -------------------
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
//!         // Server side options
//!         // -------------------
//!         // Set the initial language from the Accept-Language header of the
//!         // request. By default, it is `false`.
//!         initial_language_from_accept_language_header: true,
//!     }};
//!
//!     view! {
//!         <ChildComponent />
//!         <LanguageSelector />
//!     }
//! }
//!
//! #[component]
//! fn ChildComponent() -> impl IntoView {
//!     // Use `tr!` and `move_tr!` macros to translate strings:
//!     view! {
//!         <p>
//!             <span>{move || tr!("hello-world")}</span>
//!             <span>{move_tr!("hello-args", {
//!                 "arg1" => "value1",
//!                 "arg2" => "value2",
//!             })}</span>
//!         </p>
//!     }
//!
//!     // You must use `tr!` inside a reactive context or the translation
//!     // will not be updated on the fly when the current language changes.
//! }
//!
//! #[component]
//! fn LanguageSelector() -> impl IntoView {
//!     // Use `expect_i18n` to get the current i18n context:
//!     let i18n = expect_i18n();
//!
//!     // `i18n.languages` is a static array with the available languages
//!     // `i18n.language` is a signal with the current language
//!     // `i18n.language.get()` to get the current language
//!     // `i18n.language.set(lang)` to set the current language
//!     // `i18n.is_active_language(lang)` to check if a language is active
//!
//!     view! {
//!         <fieldset>
//!             <For
//!                 each=move || i18n.languages
//!                 key=move |lang| *lang
//!                 children=move |lang: &&Language| {
//!                     view! {
//!                         <div>
//!                             <input
//!                                 type="radio"
//!                                 id=lang
//!                                 name="language"
//!                                 value=lang
//!                                 checked=i18n.is_active_language(lang)
//!                                 on:click=move |_| i18n.language.set(lang)
//!                             />
//!                             <label for=lang>{lang.name}</label>
//!                         </div>
//!                     }
//!                 }
//!             />
//!         </fieldset>
//!     }
//! }
//! ```
//!
//! ## Features
//!
//! - **Server Side Rendering**: `ssr`
//! - **Hydration**: `hydrate`
//! - **Actix Web integration**: `actix`
//! - **Axum integration**: `axum`
//! - **JSON languages file**: `json` (enabled by default)
//! - **YAML languages file**: `yaml`
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
//! v0.0.X releases.
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

use core::hash::{Hash, Hasher};
use core::str::FromStr;
use fluent_templates::{
    fluent_bundle::FluentValue, loader::Loader, LanguageIdentifier,
    StaticLoader,
};
use leptos::{use_context, Attribute, IntoAttribute, Oco, RwSignal, SignalGet};
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

impl Eq for Language {}

impl Hash for Language {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let current_lang = expect_i18n().language.get();
        let key = format!(
            "{}{}",
            self.id,
            if self == current_lang { "1" } else { "0" }
        );
        state.write(key.as_bytes());
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        language_from_str_between_languages(s, expect_i18n().languages)
            .ok_or(())
            .map(|lang| lang.clone())
    }
}

macro_rules! impl_into_attr_for_language {
    () => {
        fn into_attribute(self) -> Attribute {
            Attribute::String(Oco::Owned(self.id.to_string()))
        }

        #[inline(always)]
        fn into_attribute_boxed(self: Box<Self>) -> Attribute {
            self.into_attribute()
        }
    };
}

impl IntoAttribute for &'static Language {
    impl_into_attr_for_language!();
}

impl IntoAttribute for &&'static Language {
    impl_into_attr_for_language!();
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
