#![deny(missing_docs)]
#![forbid(unsafe_code)]
//! [![Crates.io](https://img.shields.io/crates/v/leptos-fluent?logo=rust)](https://crates.io/crates/leptos-fluent)
//! [![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
//! [![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/ci.yml?label=tests&logo=github)](https://github.com/mondeja/leptos-fluent/actions)
//! [![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)][documentation]
//! [![Crates.io downloads](https://img.shields.io/crates/d/leptos-fluent)](https://crates.io/crates/leptos-fluent)
//! [![Discord channel](https://img.shields.io/badge/-Discord-5865F2?style=flat-square&logo=discord&logoColor=white)](https://discord.com/channels/1031524867910148188/1251579884371705927)
//!
//! Internationalization framework for [Leptos] using [fluent-templates].
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! leptos-fluent = "0.1"
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
//! If you're using `cargo-leptos` to build your project, watch the
//! _locales/_ folder with:
//!
//! ```toml
//! [package.metadata.leptos]
//! watch-additional-files = ["locales"]  # Relative from Cargo.toml file
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
//! ```ftl
//! # locales/en/main.ftl
//! hello-world = Hello, world!
//! hello-args = Hello, { $arg1 } and { $arg2 }!
//! ```
//!
//! ```ftl
//! # locales/es/main.ftl
//! hello-world = ¡Hola, mundo!
//! hello-args = ¡Hola, { $arg1 } y { $arg2 }!
//! ```
//!
//! You can use `leptos-fluent` as follows:
//!
//! ```rust,ignore
//! use fluent_templates::static_loader;
//! use leptos::*;
//! use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr};
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
//!         translations: [TRANSLATIONS],
//!         // Check translations correctness in the specified files.
//!         check_translations: "./src/**/*.rs",
//!
//!         // Client side options
//!         // -------------------
//!         // Synchronize `<html lang="...">` attribute with the current active
//!         // language. By default, it is `false`.
//!         sync_html_tag_lang: true,
//!         // Synchronize `<html dir="...">` attribute setting `"ltr"`,
//!         // `"rtl"` or `"auto"` depending on the current active language.
//!         sync_html_tag_dir: true,
//!         // Update the language on URL parameter when using the method
//!         // `I18n.set_language`. By default, it is `false`.
//!         set_language_to_url_param: true,
//!         // Set the discovered initial language of the user from
//!         // the URL in local storage. By default, it is `false`.
//!         initial_language_from_url_param_to_localstorage: true,
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
//!         // Attributes to set for the language cookie. By default is `""`.
//!         cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
//!         // Update the language on cookie when using the method `I18n.set_language`.
//!         // By default, it is `false`.
//!         set_language_to_cookie: true,
//!
//!         // Server side options
//!         // -------------------
//!         // Set the initial language from the `Accept-Language` header of the
//!         // request. By default, it is `false`.
//!         initial_language_from_accept_language_header: true,
//!
//!         // Server and client side options
//!         // ------------------------------
//!         // Name of the cookie to get and set the current language of the user.
//!         // By default, it is `"lf-lang"`.
//!         cookie_name: "lang",
//!         // Get the initial language from cookie. By default, it is `false`.
//!         initial_language_from_cookie: true,
//!         // URL parameter name to use discovering the initial language
//!         // of the user. By default is `"lang"`.
//!         url_param: "lang",
//!         // Discover the initial language of the user from the URL.
//!         // By default, it is `false`.
//!         initial_language_from_url_param: true,
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
//!                 "arg1" => "foo",
//!                 "arg2" => "bar",
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
//!     // Use `expect_i18n()` to get the current i18n context:
//!     let i18n = expect_i18n();
//!
//!     // `i18n.languages` is a static array with the available languages
//!     // `i18n.language.get()` to get the current language
//!     // `i18n.language.set(lang)` to set the current language
//!     // `lang.is_active()` to check if a language is the current selected one
//!
//!     view! {
//!         <fieldset>
//!             {
//!                 move || i18n.languages.iter().map(|lang| {
//!                     view! {
//!                         <div>
//!                             <input
//!                                 type="radio"
//!                                 id=lang
//!                                 name="language"
//!                                 value=lang
//!                                 checked=lang.is_active()
//!                                 on:click=move |_| i18n.language.set(lang)
//!                             />
//!                             <label for=lang>{lang.name}</label>
//!                         </div>
//!                     }
//!                 }).collect::<Vec<_>>()
//!             }
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
//! - **JSON5 languages file**: `json5`
//!
//! # Resources
//!
//! - [Book]
//! - [Quickstart]
//! - [Documentation]
//! - [Examples]
//!
//! [leptos]: https://leptos.dev/
//! [fluent-templates]: https://github.com/XAMPPRocky/fluent-templates
//! [quickstart]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.leptos_fluent.html
//! [examples]: https://github.com/mondeja/leptos-fluent/tree/master/examples
//! [documentation]: https://docs.rs/leptos-fluent
//! [book]: https://mondeja.github.io/leptos-fluent/

#[doc(hidden)]
pub mod cookie;
#[doc(hidden)]
pub mod http_header;
#[doc(hidden)]
pub mod localstorage;
#[doc(hidden)]
pub mod url;

use core::hash::{Hash, Hasher};
use core::str::FromStr;
use fluent_templates::{
    fluent_bundle::FluentValue, loader::Loader, once_cell::sync::Lazy,
    LanguageIdentifier, StaticLoader,
};
use leptos::{
    use_context, Attribute, IntoAttribute, Oco, RwSignal, Signal, SignalGet,
    SignalWith,
};
pub use leptos_fluent_macros::leptos_fluent;

#[doc(hidden)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum WritingDirection {
    Ltr,
    Rtl,
    Auto,
}

impl WritingDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            WritingDirection::Ltr => "ltr",
            WritingDirection::Rtl => "rtl",
            WritingDirection::Auto => "auto",
        }
    }
}

impl core::fmt::Display for WritingDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Each language supported by your application.
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
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
    /// Writing direction of the language
    pub dir: &'static WritingDirection,
}

impl Language {
    /// Check if the language is the active language.
    pub fn is_active(&self) -> bool {
        self == expect_i18n().language.get()
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Language {}

impl Hash for Language {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: `<For/>` component's `key` hashes doesn't seem to be working
        // between different hydrate and SSR contexts, so implement `Language`s
        // is currently discouraged. This needs to be fully debugged and open
        // an issue in the `leptos` repository if necessary.
        let current_lang = expect_i18n().language.get();
        let key = format!(
            "{}{}",
            self.id,
            if self == current_lang { "1" } else { "0" },
        );
        state.write(key.as_bytes());
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        language_from_str_between_languages(s, expect_i18n().languages)
            .ok_or(())
            .cloned()
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
#[derive(Clone, Copy)]
pub struct I18n {
    /// Signal that holds the current language.
    pub language: RwSignal<&'static Language>,
    /// Available languages for the application.
    pub languages: &'static [&'static Language],
    /// Static translations loader of fluent-templates.
    pub translations: Signal<Vec<&'static Lazy<StaticLoader>>>,
}

#[cfg(debug_assertions)]
impl core::fmt::Debug for I18n {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("I18n")
            .field("language", &self.language.get())
            .field("languages", &self.languages)
            .finish()
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

#[doc(hidden)]
pub fn tr_impl(text_id: &str) -> String {
    let i18n = expect_i18n();
    let lang_id = i18n.language.get().id.clone();
    let found = i18n.translations.with(|translations| {
        for tr in translations {
            if let Some(result) = tr.try_lookup(&lang_id, text_id) {
                return Some(result);
            }
        }

        None
    });

    found.unwrap_or("Unknown localization {text_id}".to_string())
}

#[doc(hidden)]
pub fn tr_with_args_impl(
    text_id: &str,
    args: &std::collections::HashMap<String, FluentValue>,
) -> String {
    let i18n = expect_i18n();
    let lang_id = i18n.language.get().id.clone();
    let found = i18n.translations.with(|translations| {
        for tr in translations {
            if let Some(result) =
                tr.try_lookup_with_args(&lang_id, text_id, args)
            {
                return Some(result);
            }
        }

        None
    });

    found.unwrap_or("Unknown localization {text_id}".to_string())
}

/// Translate a text identifier to the current language.
///
/// ```rust,ignore
/// tr!("hello-world")
/// tr!("hello-world", { "name" => "John" });
///
/// let name = "John";
/// tr!("hello-world", { "name" => name, "age" => 30 });
/// ```
#[macro_export]
macro_rules! tr {
    ($text_id:literal$(,)?) => {::leptos_fluent::tr_impl($text_id)};
    ($text_id:literal, {$($key:literal => $value:expr),*$(,)?}$(,)?) => {{
        ::leptos_fluent::tr_with_args_impl($text_id, &{
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.into());
            )*
            map
        })
    }}
}

/// [`leptos::Signal`] that translates a text identifier to the current language.
///
/// ```rust,ignore
/// move_tr!("hello-world")
/// move_tr!("hello-world", { "name" => "John" })
///
/// let name = "John";
/// move_tr!("hello-world", { "name" => name, "age" => 30 });
/// ```
///
/// The same as:
///
/// ```rust,ignore
/// Signal::derive(move || tr!("hello-world"));
/// Signal::derive(move || tr!("hello-world", { "name" => "John" }));
///
/// let name = "John";
/// Signal::derive(move || tr!("hello-world", { "name" => name, "age" => 30 }));
/// ```
///
/// [`leptos::Signal`]: https://docs.rs/leptos/latest/leptos/struct.Signal.html
#[macro_export]
macro_rules! move_tr {
    ($text_id:literal$(,)?) => {
        ::leptos::Signal::derive(move || $crate::tr!($text_id))
    };
    ($text_id:literal, {$($key:literal => $value:expr),*$(,)?}$(,)?) => {
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
                let lazy_target_lang =
                    LanguageIdentifier::from_raw_parts_unchecked(
                        target_lang.language,
                        None,
                        None,
                        None,
                    );
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
    fn major_and_minor_version(version: &str) -> String {
        version.split('.').take(2).collect::<Vec<_>>().join(".")
    }

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
            major_and_minor_version(&version.unwrap()),
            major_and_minor_version(env!("CARGO_PKG_VERSION")),
            concat!(
                "The version of leptos-fluent shown in the README at",
                " 'Installation' section is not updated."
            ),
        );
    }
}
