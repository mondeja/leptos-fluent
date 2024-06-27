#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(fn_traits))]
#![cfg_attr(feature = "nightly", feature(unboxed_closures))]

//! [![Crates.io](https://img.shields.io/crates/v/leptos-fluent?logo=rust)](https://crates.io/crates/leptos-fluent)
//! [![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
//! [![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/ci.yml?label=tests&logo=github)](https://github.com/mondeja/leptos-fluent/actions)
//! [![Book](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/.github%2Fworkflows%2Fci.yml?logo=github&label=book)](https://mondeja.github.io/leptos-fluent/)
//! [![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)](https://docs.rs/leptos-fluent)
//! [![Crates.io downloads](https://img.shields.io/crates/d/leptos-fluent)](https://crates.io/crates/leptos-fluent)
//! [![Discord channel](https://img.shields.io/badge/discord-grey?logo=discord&logoColor=white)](https://discord.com/channels/1031524867910148188/1251579884371705927)
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
//! watch-additional-files = ["locales"]  # Relative to Cargo.toml
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
//!         // Path to the locales directory, relative to Cargo.toml.
//!         locales: "./locales",
//!         // Static translations struct provided by fluent-templates.
//!         translations: [TRANSLATIONS],
//!         // Check translations correctness in the specified files.
//!         check_translations: "./src/**/*.rs",
//!
//!         // Next options are all opt-in and can be enabled
//!         // separately as needed.
//!
//!         // Client side options
//!         // -------------------
//!         // Synchronize `<html lang="...">` attribute with the
//!         // current active language.
//!         sync_html_tag_lang: true,
//!         // Synchronize `<html dir="...">` attribute with `"ltr"`,
//!         // `"rtl"` or `"auto"` depending on active language.
//!         sync_html_tag_dir: true,
//!         // Update language on URL parameter when changes.
//!         set_language_to_url_param: true,
//!         // Set initial language of the user from
//!         // URL in local storage.
//!         initial_language_from_url_param_to_localstorage: true,
//!         // Set initial language of the user from
//!         // URL in a cookie.
//!         initial_language_from_url_param_to_cookie: true,
//!         // Key used to get and set the current language of the
//!         // user on local storage. By default is `"lang"`.
//!         localstorage_key: "language",
//!         // Get initial language from local storage if not found
//!         // in an URL param.
//!         initial_language_from_localstorage: true,
//!         // Set the initial language of the user from
//!         // local storage to a cookie.
//!         initial_language_from_localstorage_to_cookie: true,
//!         // Update language on local storage when changes.
//!         set_language_to_localstorage: true,
//!         // Get initial language from `navigator.languages`
//!         // if not found in local storage.
//!         initial_language_from_navigator: true,
//!         // Set initial language of the user from
//!         // the navigator to local storage.
//!         initial_language_from_navigator_to_localstorage: true,
//!         // Set initial language of the user from
//!         // the navigator to a cookie.
//!         initial_language_from_navigator_to_cookie: true,
//!         // Attributes to set for language cookie.
//!         // By default is `""`.
//!         cookie_attrs: "Secure; Path=/; Max-Age=600",
//!         // Update language on cookie when the language changes.
//!         set_language_to_cookie: true,
//!         // Set initial language from a cookie to local storage.
//!         initial_language_from_cookie_to_localstorage: true,
//!
//!         // Server side options
//!         // -------------------
//!         // Set initial language from the `Accept-Language`
//!         // header of the request.
//!         initial_language_from_accept_language_header: true,
//!
//!         // Server and client side options
//!         // ------------------------------
//!         // Name of the cookie to get and set the current active
//!         // language. By default, it is `"lf-lang"`.
//!         cookie_name: "lang",
//!         // Set initial language from cookie.
//!         initial_language_from_cookie: true,
//!         // URL parameter to use setting the language in the URL.
//!         // By default is `"lang"`.
//!         url_param: "lang",
//!         // Set initial language of the user from an URL parameter.
//!         initial_language_from_url_param: true,
//!
//!         // Desktop applications (feature `system`)
//!         // ---------------------------------------
//!         // Set initial language from the system locale.
//!         initial_language_from_system: true,
//!         // Set initial language of the user from
//!         // the system locale to a data file.
//!         initial_language_from_system_to_data_file: true,
//!         // Get initial language from a data file.
//!         initial_language_from_data_file: true,
//!         // Key to use to name the data file. Should be unique per
//!         // application. By default is `"leptos-fluent"`.
//!         data_file_key: "my-app",
//!         // Set the language selected to a data file.
//!         set_language_to_data_file: true,
//!     }};
//!
//!     view! {
//!         <TranslatableComponent />
//!         <LanguageSelector />
//!     }
//! }
//!
//! #[component]
//! fn TranslatableComponent() -> impl IntoView {
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
//!     // The `tr!` macro must be inside a reactive context or the
//!     // translation will not be updated on the fly when the language changes.
//! }
//!
//! #[component]
//! fn LanguageSelector() -> impl IntoView {
//!     // `expect_i18n()` to get the i18n context
//!     // `i18n.languages` is a static array with the available languages
//!     // `i18n.language.get()` to get the current language
//!     // `lang.activate()` to set the current language
//!     // `lang.is_active()` to check if a language is the current selected one
//!
//!     view! {
//!         <fieldset>
//!             {
//!                 move || expect_i18n().languages.iter().map(|lang| {
//!                     view! {
//!                         <div>
//!                             <input
//!                                 type="radio"
//!                                 id=lang
//!                                 name="language"
//!                                 value=lang
//!                                 checked=lang.is_active()
//!                                 on:click=move |_| lang.activate()
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
//! - **Nightly toolchain**: `nightly`
//! - **Desktop applications**: `system`
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
//! [quickstart]: https://mondeja.github.io/leptos-fluent/leptos_fluent.html
//! [examples]: https://github.com/mondeja/leptos-fluent/tree/master/examples
//! [book]: https://mondeja.github.io/leptos-fluent/
//! [documentation]: https://docs.rs/leptos-fluent

#[doc(hidden)]
pub mod cookie;
#[cfg(feature = "system")]
#[doc(hidden)]
pub mod data_file;
#[doc(hidden)]
pub mod http_header;
#[doc(hidden)]
pub mod localstorage;
#[doc(hidden)]
pub mod url;

#[cfg(feature = "system")]
pub use current_locale::current_locale;
#[doc(hidden)]
pub use web_sys;

use core::hash::{Hash, Hasher};
use core::str::FromStr;
use fluent_templates::{
    fluent_bundle::FluentValue, loader::Loader, once_cell::sync::Lazy,
    LanguageIdentifier, StaticLoader,
};
use leptos::{
    component, use_context, Attribute, IntoAttribute, IntoView, Oco, RwSignal,
    Signal, SignalGet, SignalSet, SignalWith,
};
#[cfg(feature = "ssr")]
use leptos::{view, SignalGetUntracked};
pub use leptos_fluent_macros::leptos_fluent;
#[cfg(feature = "ssr")]
use leptos_meta::Html;

/// Direction of the text
#[derive(Debug)]
pub enum WritingDirection {
    /// Left to right
    Ltr,
    /// Right to left
    Rtl,
    /// Auto
    Auto,
}

impl WritingDirection {
    /// Get the string representation of the writing direction
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
    /// Flag of the country of the language (if any) as emoji
    pub flag: Option<&'static str>,
}

impl Language {
    /// Get if the language is the active language.
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self == expect_i18n().language.get()
    }

    /// Set the language as the active language.
    #[inline(always)]
    pub fn activate(&'static self) {
        expect_i18n().language.set(self);
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
        /// Convert a language to an HTML attribute passing the language identifier.
        ///
        /// ```rust,ignore
        /// // The following code:
        /// <input id={lang} ... >
        /// // is the same as
        /// <input id={lang.id.to_string()} ... >
        /// ```
        #[inline(always)]
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
/// Used to provide the current language, the available languages and all
/// the translations. It is capable of doing what is needed to translate
/// and manage translations in a whole application.
#[derive(Clone, Copy)]
pub struct I18n {
    /// Signal that holds the current language.
    pub language: RwSignal<&'static Language>,
    /// Available languages for the application.
    pub languages: &'static [&'static Language],
    /// Signal with a vector of fluent-templates static loaders.
    pub translations: Signal<Vec<&'static Lazy<StaticLoader>>>,
}

impl I18n {
    /// Returns a context with meta information about the i18n context.
    ///
    /// Useful to get at runtime the parameters that created the context
    /// when invoking the `leptos_fluent!` macro. The context needs to be
    /// created activating the `provide_meta_context` or this method will
    /// raise an error message.
    ///
    /// ```rust,ignore
    /// let i18n = leptos_fluent! {{
    ///     // ...
    ///     provide_meta_context: true,
    /// }};
    ///
    /// leptos::logging::log!("Macro parameters: {:?}", i18n.meta().unwrap());
    /// ```
    pub fn meta(&self) -> Result<LeptosFluentMeta, String> {
        leptos::use_context::<LeptosFluentMeta>().ok_or(
            concat!(
                "You need to call `leptos_fluent!` with the parameter",
                " 'provide_meta_context' enabled to provide the meta context",
                " for the macro."
            )
            .to_string(),
        )
    }
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

// get language
#[cfg(feature = "nightly")]
impl FnOnce<()> for I18n {
    type Output = &'static Language;
    #[inline]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.language.get()
    }
}

#[cfg(feature = "nightly")]
impl FnMut<()> for I18n {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.language.get()
    }
}

#[cfg(feature = "nightly")]
impl Fn<()> for I18n {
    #[inline]
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.language.get()
    }
}

// set language
#[cfg(feature = "nightly")]
impl FnOnce<(&'static Language,)> for I18n {
    type Output = ();
    #[inline]
    extern "rust-call" fn call_once(
        self,
        (lang,): (&'static Language,),
    ) -> Self::Output {
        self.language.set(&lang)
    }
}

#[cfg(feature = "nightly")]
impl FnMut<(&'static Language,)> for I18n {
    #[inline]
    extern "rust-call" fn call_mut(
        &mut self,
        (lang,): (&'static Language,),
    ) -> Self::Output {
        self.language.set(&lang)
    }
}

#[cfg(feature = "nightly")]
impl Fn<(&'static Language,)> for I18n {
    #[inline]
    extern "rust-call" fn call(
        &self,
        (lang,): (&'static Language,),
    ) -> Self::Output {
        self.language.set(&lang)
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

    found.unwrap_or(format!("Unknown localization {text_id}"))
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

    found.unwrap_or(format!("Unknown localization {text_id}"))
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

// Used by `leptos_fluent!` macro
#[doc(hidden)]
#[inline(always)]
pub fn l(
    code: &str,
    languages: &'static [&Language],
) -> Option<&'static Language> {
    language_from_str_between_languages(code, languages)
}

/// Reactive HTML tag to set attributes on SSR
///
/// Currently there is not a way to set the `dir` and `lang` attributes
/// of `<html>` tags on SSR. This components updates it on SSR. Must be
/// rendered in a view.
///
/// ```rust,ignore
/// use leptos_fluent::SsrHtmlTag;
///
/// view! {
///     <SsrHtmlTag/>
/// }
/// ```
#[component(transparent)]
#[cfg(feature = "ssr")]
pub fn SsrHtmlTag() -> impl IntoView {
    let lang = expect_i18n().language.get_untracked();
    view! { <Html lang=lang.id.to_string() dir=lang.dir.as_str()/> }
}

/// Reactive HTML tag to set attributes on SSR
///
/// Currently there is not a way to set the `dir` and `lang` attributes
/// of `<html>` tags on SSR. This components updates it on SSR. Must be
/// rendered in a view.
///
/// ```rust,ignore
/// use leptos_fluent::SsrHtmlTag;
///
/// view! {
///     <SsrHtmlTag/>
/// }
/// ```
#[component(transparent)]
#[cfg(not(feature = "ssr"))]
pub fn SsrHtmlTag() -> impl IntoView {}

/// Parameters passed to `leptos_fluent!` macro at creation of `i18n` context
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct LeptosFluentMeta {
    pub locales: &'static str,
    pub core_locales: Option<&'static str>,
    pub languages: Option<&'static str>,
    pub check_translations: Option<&'static str>,
    pub provide_meta_context: bool,
    pub sync_html_tag_lang: bool,
    pub sync_html_tag_dir: bool,
    pub url_param: &'static str,
    pub initial_language_from_url_param: bool,
    pub initial_language_from_url_param_to_localstorage: bool,
    pub initial_language_from_url_param_to_cookie: bool,
    pub set_language_to_url_param: bool,
    pub localstorage_key: &'static str,
    pub initial_language_from_localstorage: bool,
    pub initial_language_from_localstorage_to_cookie: bool,
    pub set_language_to_localstorage: bool,
    pub initial_language_from_navigator: bool,
    pub initial_language_from_navigator_to_localstorage: bool,
    pub initial_language_from_navigator_to_cookie: bool,
    pub initial_language_from_accept_language_header: bool,
    pub cookie_name: &'static str,
    pub cookie_attrs: &'static str,
    pub initial_language_from_cookie: bool,
    pub initial_language_from_cookie_to_localstorage: bool,
    pub set_language_to_cookie: bool,
    #[cfg(feature = "system")]
    pub initial_language_from_system: bool,
    #[cfg(feature = "system")]
    pub initial_language_from_data_file: bool,
    #[cfg(feature = "system")]
    pub initial_language_from_system_to_data_file: bool,
    #[cfg(feature = "system")]
    pub set_language_to_data_file: bool,
    #[cfg(feature = "system")]
    pub data_file_key: &'static str,
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
            major_and_minor_version(version.unwrap()),
            major_and_minor_version(env!("CARGO_PKG_VERSION")),
            concat!(
                "The version of leptos-fluent shown in the README at",
                " 'Installation' section is not updated."
            ),
        );
    }
}
