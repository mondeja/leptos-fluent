#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(fn_traits))]
#![cfg_attr(feature = "nightly", feature(unboxed_closures))]
#![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]

//! [![Crates.io](https://img.shields.io/crates/v/leptos-fluent?logo=rust)](https://crates.io/crates/leptos-fluent)
//! [![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE)
//! [![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/ci.yml?label=tests&logo=github)](https://github.com/mondeja/leptos-fluent/actions)
//! [![Book](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/.github%2Fworkflows%2Fci.yml?logo=github&label=book)][book]
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
//! leptos-fluent = "0.2"
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
//! use leptos::prelude::*;
//! use leptos_fluent::{I18n, leptos_fluent, move_tr, tr};
//!
//! #[component]
//! fn I18nProvider(children: Children) -> impl IntoView {
//!     // See all options in the reference at
//!     // https://mondeja.github.io/leptos-fluent/latest/leptos_fluent.html
//!     leptos_fluent! {
//!         children: children(),
//!         // Path to the locales directory, relative to Cargo.toml.
//!         locales: "./locales",
//!         // Initial language when the user don't load any with
//!         // the provided configuration.
//!         default_language: "en",
//!         // Check translations correctness in the specified files.
//!         #[cfg(debug_assertions)]
//!         check_translations: "./src/**/*.rs",
//!
//!         // Client side options
//!         // -------------------
//!         // Synchronize `<html lang="...">` attribute with
//!         // current active language.
//!         sync_html_tag_lang: true,
//!         // Synchronize `<html dir="...">` attribute with `"ltr"`,
//!         // `"rtl"` or `"auto"` depending on active language.
//!         sync_html_tag_dir: true,
//!         // Update language on URL parameter when changes.
//!         set_language_to_url_param: true,
//!         // Set initial language of user from URL in local storage.
//!         initial_language_from_url_param_to_local_storage: true,
//!         // Set initial language of user from URL in a cookie.
//!         initial_language_from_url_param_to_cookie: true,
//!         // Key used to get and set the current language of the
//!         // user on local storage. By default is `"lang"`.
//!         local_storage_key: "language",
//!         // Get initial language from local storage if not found
//!         // in an URL param.
//!         initial_language_from_local_storage: true,
//!         // Set the initial language of the user from
//!         // local storage to a cookie.
//!         initial_language_from_local_storage_to_cookie: true,
//!         // Update language on local storage when changes.
//!         set_language_to_local_storage: true,
//!         // Get initial language from `navigator.languages`
//!         // if not found in local storage.
//!         initial_language_from_navigator: true,
//!         // Set initial language of user from navigator to local storage.
//!         initial_language_from_navigator_to_local_storage: true,
//!         // Set initial language of user from navigator to a cookie.
//!         initial_language_from_navigator_to_cookie: true,
//!         // Attributes to set for language cookie.
//!         // By default `""`.
//!         cookie_attrs: "Secure; Path=/; Max-Age=600",
//!         // Update language on cookie when the language changes.
//!         set_language_to_cookie: true,
//!         // Set initial language from a cookie to local storage.
//!         initial_language_from_cookie_to_local_storage: true,
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
//!         // language. By default `"lf-lang"`.
//!         cookie_name: "lang",
//!         // Set initial language from cookie.
//!         initial_language_from_cookie: true,
//!         // URL parameter to use setting the language in the URL.
//!         // By default `"lang"`.
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
//!         // application. By default `"leptos-fluent"`.
//!         data_file_key: "my-app",
//!         // Set the language selected to a data file.
//!         set_language_to_data_file: true,
//!     }
//! }
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     view! {
//!         <I18nProvider>
//!             <TranslatableComponent/>
//!             <LanguageSelector/>
//!         </I18nProvider>
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
//!     // `expect_context::<leptos_fluent::I18n>()` to get the i18n context
//!     // `i18n.languages` exposes a static array with the available languages
//!     // `i18n.language.get()` to get the active language
//!     // `i18n.language.set(lang)` to set the active language
//!
//!     let i18n = expect_context::<I18n>();
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
//!                                 checked=&i18n.language.get() == lang
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
//! - **Nightly toolchain**: `nightly`
//! - **Desktop applications**: `system`
//! - **JSON languages file**: `json`
//! - **YAML languages file**: `yaml`
//! - **JSON5 languages file**: `json5`
//! - **Tracing support**: `tracing`
//! - **Debugging**: `debug`
//! - **Disable Unicode isolating marks**: `disable-unicode-isolating-marks`
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
//! [quickstart]: https://mondeja.github.io/leptos-fluent/latest/leptos_fluent.html
//! [examples]: https://github.com/mondeja/leptos-fluent/tree/master/examples
//! [book]: https://mondeja.github.io/leptos-fluent/latest/
//! [documentation]: https://docs.rs/leptos-fluent

#[doc(hidden)]
pub mod cookie;
#[cfg(feature = "system")]
#[doc(hidden)]
pub mod data_file;
#[doc(hidden)]
pub mod http_header;
#[doc(hidden)]
pub mod local_storage;
#[doc(hidden)]
pub mod session_storage;
#[doc(hidden)]
pub mod url;

#[doc(hidden)]
#[cfg(feature = "system")]
pub extern crate current_locale;
#[doc(hidden)]
pub extern crate fluent_templates;
#[doc(hidden)]
pub extern crate leptos_meta;
#[doc(hidden)]
pub extern crate web_sys;
pub use leptos_fluent_macros::leptos_fluent;

use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::str::FromStr;
use fluent_templates::{
    fluent_bundle::FluentValue, loader::Loader, LanguageIdentifier,
    StaticLoader,
};
#[cfg(feature = "nightly")]
use leptos::prelude::Get;
use leptos::{
    attr::AttributeValue,
    prelude::{
        guards::ReadGuard, use_context, Read, RwSignal, Set, Signal, With,
    },
};
use std::borrow::Cow;
use std::sync::LazyLock;

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
#[derive(Clone, Debug)]
pub struct Language {
    /// Language identifier
    ///
    /// Can be any valid language tag, such as `en`, `es`, `en-US`, `es-ES`, etc.
    pub id: &'static LanguageIdentifier,
    /// Language name
    ///
    /// The name of the language, such as `English`, `Español`, etc.
    /// This name will be intended to be displayed in the language selector.
    pub name: &'static str,
    /// Writing direction of the language
    pub dir: &'static WritingDirection,
    /// Flag of the country of the language as emoji (if any)
    pub flag: Option<&'static str>,
}

impl Language {
    /// Get if the language is the active language.
    #[deprecated(
        since = "0.2.13",
        note = "will be removed in v0.3.0. Use `&i18n.language.get() == lang` instead of `lang.is_active()`."
    )]
    #[inline(always)]
    pub fn is_active(&'static self) -> bool {
        self == leptos::prelude::expect_context::<I18n>().language.read()
    }

    /// Set the language as the active language.
    #[deprecated(
        since = "0.2.13",
        note = "will be removed in v0.3.0. Use `&i18n.language.set(lang)` instead of `lang.activate()`."
    )]
    #[inline(always)]
    pub fn activate(&'static self) {
        leptos::prelude::expect_context::<I18n>().language.set(self);
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Implementation for `&Language == ReadGuard<&Language, Plain<&Language>>`
//
// This implementation is required to ensure symmetry with
// `ReadGuard<&Language, Plain<&Language>> == &Language`, implemented by Leptos.
// That implementation cannot be included in Leptos as it would generate
// the problem of transitive chains that criss-cross crate boundaries.
// See `PartiaEq` documentation.
impl<'a, Inner> PartialEq<ReadGuard<&'a Language, Inner>> for &Language
where
    Inner: Deref<Target = &'a Language>,
{
    fn eq(&self, other: &ReadGuard<&'a Language, Inner>) -> bool {
        self == other.deref()
    }
}

impl Eq for Language {}

impl Hash for Language {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: `<For/>` component's `key` hashes doesn't seem to be working
        // between different hydrate and SSR contexts, so implement `Language`s
        // is currently discouraged. This needs to be fully debugged and open
        // an issue in the `leptos` repository if necessary.
        let current_lang =
            leptos::prelude::expect_context::<I18n>().language.read();
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
        language_from_str_between_languages(
            s,
            leptos::prelude::expect_context::<I18n>().languages,
        )
        .ok_or(())
        .cloned()
    }
}

macro_rules! impl_attr_value_for_language {
    () => {
        type State = <String as AttributeValue>::State;
        type AsyncOutput = String;
        type Cloneable = String;
        type CloneableOwned = String;

        fn html_len(&self) -> usize {
            self.id.to_string().len()
        }

        fn to_html(self, key: &str, buf: &mut String) {
            <&str as AttributeValue>::to_html(
                self.id.to_string().as_str(),
                key,
                buf,
            );
        }

        fn to_template(_key: &str, _buf: &mut String) {}

        fn hydrate<const FROM_SERVER: bool>(
            self,
            key: &str,
            el: &leptos::tachys::renderer::types::Element,
        ) -> Self::State {
            <String as AttributeValue>::hydrate::<FROM_SERVER>(
                self.id.to_string(),
                key,
                el,
            )
        }

        fn build(
            self,
            el: &leptos::tachys::renderer::types::Element,
            key: &str,
        ) -> Self::State {
            <String as AttributeValue>::build(self.id.to_string(), el, key)
        }

        fn rebuild(self, key: &str, state: &mut Self::State) {
            <String as AttributeValue>::rebuild(self.id.to_string(), key, state)
        }

        fn into_cloneable(self) -> Self::Cloneable {
            self.id.to_string()
        }

        fn into_cloneable_owned(self) -> Self::CloneableOwned {
            self.id.to_string()
        }

        fn dry_resolve(&mut self) {}

        async fn resolve(self) -> Self::AsyncOutput {
            self.id.to_string()
        }
    };
}

impl AttributeValue for &Language {
    impl_attr_value_for_language!();
}

impl AttributeValue for &&Language {
    impl_attr_value_for_language!();
}

/// Internationalization context.
///
/// Used to provide the current language, the available languages and all
/// the translations. It is capable of doing what is needed to translate
/// and manage translations in a whole application.
#[derive(Clone, Copy, Debug)]
pub struct I18n {
    /// Signal that holds the current language.
    pub language: RwSignal<&'static Language>,
    /// Available languages for the application.
    pub languages: &'static [&'static Language],
    /// Signal with a vector of fluent-templates static loaders.
    pub translations: Signal<Vec<&'static LazyLock<StaticLoader>>>,
}

impl I18n {
    /// Get meta information about the i18n context.
    ///
    /// Useful to get at runtime the parameters that created the context
    /// when invoking the `leptos_fluent!` macro. The context needs to be
    /// created activating the `provide_meta_context` or this method will
    /// raise an error message.
    ///
    /// ```rust,ignore
    /// use leptos::prelude::*;
    /// use leptos_fluent::{leptos_fluent, I18n};
    ///
    /// leptos_fluent! {
    ///     // ...
    ///     provide_meta_context: true,
    /// };
    ///
    /// // ... later in the code
    /// leptos::logging::log!("Macro parameters: {:?}", expect_context::<I18n>().meta().unwrap());
    /// ```
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", err(Debug))
    )]
    pub fn meta(&self) -> Result<LeptosFluentMeta, &'static str> {
        leptos::prelude::use_context::<LeptosFluentMeta>().ok_or(concat!(
            "You need to call `leptos_fluent!` with the parameter",
            " 'provide_meta_context' enabled to provide the meta context",
            " for the macro."
        ))
    }

    /// Get the translation of a text identifier to the current language.
    ///
    /// ```rust,ignore
    /// use leptos::prelude::expect_context;
    /// use leptos_fluent::I18n;
    ///
    /// let text_id = "hello-world";
    /// expect_context::<I18n>().tr(text_id);
    /// ```
    ///
    /// Calls to this function will not be checked by the `check_translations` option
    /// in the `leptos_fluent!` macro. To avoid the warning for a specific identifier
    /// or to use dynamic variables for translation data, use this function directly.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all)
    )]
    pub fn tr(&self, text_id: &str) -> String {
        let found = self.translations.with(|translations| {
            self.language.with(|language| {
                translations
                    .iter()
                    .find_map(|tr| tr.try_lookup(language.id, text_id))
            })
        });

        #[cfg(feature = "tracing")]
        {
            if found.is_none() {
                tracing::warn!(
                "Localization message \"{text_id}\" not found in any translation"
            );
            } else {
                tracing::trace!(
                    "{}",
                    format!(
                        concat!(
                        "Localization message \"{}\" found in a translation.",
                        " Translated to \"{}\"."
                    ),
                        text_id,
                        found.as_ref().unwrap()
                    ),
                );
            }
        }

        found.unwrap_or(format!("Unknown localization {text_id}"))
    }

    /// Get the translation of a text identifier to the current language with arguments.
    ///
    /// ```rust,ignore
    /// use leptos::prelude::expect_context;
    /// use std::collections::HashMap;
    /// use leptos_fluent::I18n;
    ///
    /// let i18n = expect_context::<I18n>();
    /// let mut args = HashMap::new();
    /// args.insert("name".into(), "John".into());
    /// i18n.tr_with_args("hello-args", &args);
    /// ```
    ///
    /// Calls to this function will not be checked by the `check_translations` option
    /// in the `leptos_fluent!` macro. To avoid the warning for a specific identifier
    /// or to use dynamic variables for translation data, use this function directly.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all)
    )]
    pub fn tr_with_args(
        &self,
        text_id: &str,
        args: &std::collections::HashMap<Cow<'static, str>, FluentValue>,
    ) -> String {
        let found = self.translations.with(|translations| {
            self.language.with(|language| {
                translations.iter().find_map(|tr| {
                    tr.try_lookup_with_args(language.id, text_id, args)
                })
            })
        });

        #[cfg(feature = "tracing")]
        {
            if found.is_none() {
                tracing::warn!(
                "Localization message \"{text_id}\" not found in any translation"
            );
            } else {
                tracing::trace!(
                    "{}",
                    format!(
                        concat!(
                        "Localization message \"{}\" found in a translation.",
                        " Translated to \"{}\"."
                    ),
                        text_id,
                        found.as_ref().unwrap()
                    ),
                );
            }
        }

        found.unwrap_or(format!("Unknown localization {text_id}"))
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
#[deprecated(
    since = "0.2.13",
    note = "will be removed in v0.3.0. Use `use_context::<leptos_fluent::I18n>()` instead of `use_i18n()`."
)]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
#[inline(always)]
pub fn use_i18n() -> Option<I18n> {
    use_context::<I18n>()
}

const EXPECT_I18N_ERROR_MESSAGE: &str = concat!(
    "I18n context is missing, use the `leptos_fluent!` macro to provide it.\n\n",
    "If you're sure that the context has been provided probably the invocation",
    " resides outside of the reactive ownership tree, thus the context is not",
    " reachable. Use instead:\n",
    "  - `tr!(i18n, \"text-id\")` instead of `tr!(\"text-id\")`.\n",
    "  - `move_tr!(i18n, \"text-id\")` instead of `move_tr!(\"text-id\")`.\n",
    "  - `i18n.language.set(lang)` instead of `lang.activate()`.\n",
    "  - `lang == i18n.language.get()` instead of `lang.is_active()`.\n",
    "  - Copy `i18n` context instead of getting it with `expect_context::<leptos_fluent::I18n>()`.",
);

/// Expect the current context for localization.
#[deprecated(
    since = "0.2.13",
    note = "will be removed in v0.3.0. Use `expect_context::<leptos_fluent::I18n>()` instead of `expect_i18n()`."
)]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
#[inline]
pub fn expect_i18n() -> I18n {
    if let Some(i18n) = use_context::<I18n>() {
        i18n
    } else {
        #[cfg(feature = "tracing")]
        tracing::error!(EXPECT_I18N_ERROR_MESSAGE);
        panic!("{}", EXPECT_I18N_ERROR_MESSAGE)
    }
}

/// Expect the current context for localization.
#[deprecated(
    since = "0.2.13",
    note = "will be removed in v0.3.0. Use `expect_context::<leptos_fluent::I18n>()` instead of `i18n()`."
)]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
#[inline(always)]
pub fn i18n() -> I18n {
    #[allow(deprecated)]
    expect_i18n()
}

/// Get the translation of a text identifier to the current language.
#[deprecated(
    since = "0.2.7",
    note = "will be removed in v0.3.0. Use `i18n.tr(text_id)` instead of `tr_impl(i18n, text_id)`."
)]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[inline(always)]
pub fn tr_impl(i18n: I18n, text_id: &str) -> String {
    i18n.tr(text_id)
}

/// Get the translation of a text identifier to the current language with arguments.
#[deprecated(
    since = "0.2.7",
    note = "will be removed in v0.3.0. Use `i18n.tr_with_args(text_id, args)` instead of `tr_with_args_impl(i18n, text_id, args)`."
)]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[inline(always)]
pub fn tr_with_args_impl(
    i18n: I18n,
    text_id: &str,
    args: &std::collections::HashMap<Cow<'static, str>, FluentValue>,
) -> String {
    i18n.tr_with_args(text_id, args)
}

/// Translate a text identifier to the current language.
///
/// ```rust,ignore
/// use leptos_fluent::tr;
///
/// tr!("hello-world")
/// tr!("hello-world", { "name" => "John" });
///
/// let name = "John";
/// tr!("hello-world", { "name" => name, "age" => 30 });
/// ```
///
/// Needs to be placed inside a reactive context to update the translation
/// on the fly when the language changes.
///
/// ```rust,ignore
/// use leptos_fluent::tr;
///
/// let text_signal = move || tr!("hello-world");
/// ```
///
/// When using `check_translations` option in the `leptos_fluent!` macro,
/// the usage of `tr!` will raise a warning if the translation data does not
/// match against the translations files. To avoid the warning for a specific
/// identifier or to use dynamic variables for translation data, use `i18n.tr`
/// or `i18n.tr_with_args` methods directly.
#[macro_export]
macro_rules! tr {
    ($text_id:literal$(,)?) => {::leptos::prelude::expect_context::<$crate::I18n>().tr($text_id)};
    (
        $text_id:literal,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {{
        ::leptos::prelude::expect_context::<$crate::I18n>().tr_with_args(
            $text_id,
            $( #[$args_cfgs] )*
            &{
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert($key.into(), $value.into());
                )*
                map
            }
        )
    }};
    ($i18n:ident, $text_id:literal$(,)?) => {$i18n.tr($text_id)};
    (
        $i18n:ident,
        $text_id:literal,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {{
        $i18n.tr_with_args(
            $text_id,
            $( #[$args_cfgs] )*
            &{
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert($key.into(), $value.into());
                )*
                map
            }
        )
    }};

    //
    // Next dynamic syntax is accepted by the translations checker.
    //
    // The cases are numbered to track them easily as it can grow in the future.
    //

    // 001 if_elseif_else
    //   only text_id
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal }
        $(,)?
    ) => {
        $(#[$if_cfgs])*
        if $condition {
            $crate::tr!($text_id)
        } $(else if $else_if_condition {
            $crate::tr!($else_if_text_id)
        })* else {
            $crate::tr!($else_text_id)
        }
    };
    //   with i18n
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal }
        $(,)?
    ) => {
        $( #[$if_cfgs] )*
        if $condition {
            $i18n.tr($text_id)
        } $(else if $else_if_condition {
            $i18n.tr($else_if_text_id)
        })* else {
            $i18n.tr($else_text_id)
        }
    };
    //   with args
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal },
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {

        {
            $( #[$args_cfgs] )*
            let map = {
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert($key.into(), $value.into());
                )*
                map
            };
            $( #[$if_cfgs] )*
            if $condition {
                ::leptos::prelude::expect_context::<$crate::I18n>().tr_with_args($text_id, &map)
            } $(else if $else_if_condition {
                ::leptos::prelude::expect_context::<$crate::I18n>().tr_with_args($else_if_text_id, &map)
            })* else {
                ::leptos::prelude::expect_context::<$crate::I18n>().tr_with_args($else_text_id, &map)
            }
        }
    };
    //   with i18n and args
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal },
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        {
            $( #[$args_cfgs] )*
            let map = {
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert($key.into(), $value.into());
                )*
                map
            };
            $( #[$if_cfgs] )*
            if $condition {
                $i18n.tr_with_args($text_id, &map)
            } $(else if $else_if_condition {
                $i18n.tr_with_args($else_if_text_id, &map)
            })* else {
                $i18n.tr_with_args($else_text_id, &map)
            }
        }
    };

    //   no else
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };
    //   no else with i18n
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };
    //   no else with args
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };
    //   no else with i18n and args
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };

    // It seems that the next branches are excluded by the compiler and instead is triggering
    // a compile error with the message 'argument must be a string literal'.
    (
        $( #[$id_cfgs:meta] )*
        $text_id:expr$(,)?
    ) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
    (
        $( #[$id_cfgs:meta] )*
        $text_id:expr,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}$(,)?
    ) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
    (
        $i18n:expr,
        $( #[$id_cfgs:meta] )*
        $text_id:expr$(,)?
    ) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
    (
        $i18n:expr,
        $( #[$id_cfgs:meta] )*
        $text_id:expr,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}$(,)?
    ) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
}

/// [Leptos's `Signal`] that translates a text identifier to the current language.
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
/// When using `check_translations` option in the `leptos_fluent!` macro,
/// the usage of `move_tr!` will raise a warning if the translation data does not
/// match against the translations files. To avoid the warning for a specific
/// identifier or to use dynamic variables for translation data, use `i18n.tr`
/// or `i18n.tr_with_args` methods directly.
///
/// [Leptos's `Signal`]: https://docs.rs/reactive_graph/0.1.0/reactive_graph/wrappers/read/struct.Signal.html
#[macro_export]
macro_rules! move_tr {
    ($text_id:literal$(,)?) => {
        ::leptos::prelude::Signal::derive(move || $crate::tr!($text_id))
    };
    (
        $text_id:literal,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        ::leptos::prelude::Signal::derive(move || $crate::tr!(
            $text_id,
            $( #[$args_cfgs] )*
            {
                $(
                    $key => $value,
                )*
            }
        ))
    };
    ($i18n:ident, $text_id:literal$(,)?) => {
        ::leptos::prelude::Signal::derive(move || $crate::tr!($i18n, $text_id))
    };
    (
        $i18n:ident,
        $text_id:literal,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        ::leptos::prelude::Signal::derive(move || $crate::tr!(
            $i18n,
            $text_id,
            $( #[$args_cfgs] )*
            {
                $(
                    $key => $value,
                )*
            }
        ))
    };

    // 001 if_elseif_else
    //   only text id
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal }
        $(,)?
    ) => {
        ::leptos::prelude::Signal::derive(move || {
            $( #[$if_cfgs] )*
            if $condition {
                $crate::tr!($text_id)
            } $(else if $else_if_condition {
                $crate::tr!($else_if_text_id)
            })* else {
                $crate::tr!($else_text_id)
            }
        })
    };
    //   with i18n
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal }
        $(,)?
    ) => {
        ::leptos::prelude::Signal::derive(move || {
            $( #[$if_cfgs] )*
            if $condition {
                $i18n.tr($text_id)
            } $(else if $else_if_condition {
                $i18n.tr($else_if_text_id)
            })* else {
                $i18n.tr($else_text_id)
            }
        })
    };
    //   with args
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal },
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        ::leptos::prelude::Signal::derive(move || {
            $( #[$args_cfgs] )*
            let map = {
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert($key.into(), $value.into());
                )*
                map
            };
            $( #[$if_cfgs] )*
            if $condition {
                ::leptos::prelude::expect_context::<$crate::I18n>().tr_with_args($text_id, &map)
            } $(else if $else_if_condition {
                ::leptos::prelude::expect_context::<$crate::I18n>().tr_with_args($else_if_text_id, &map)
            })* else {
                ::leptos::prelude::expect_context::<$crate::I18n>().tr_with_args($else_text_id, &map)
            }
        })
    };
    //   with i18n and args
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })* else { $else_text_id:literal },
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        ::leptos::prelude::Signal::derive(move || {
            $( #[$args_cfgs] )*
            let map = {
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert($key.into(), $value.into());
                )*
                map
            };
            $( #[$if_cfgs] )*
            if $condition {
                $i18n.tr_with_args($text_id, &map)
            } $(else if $else_if_condition {
                $i18n.tr_with_args($else_if_text_id, &map)
            })* else {
                $i18n.tr_with_args($else_text_id, &map)
            }
        })
    };

    //   no else
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };
    //   no else with i18n
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };
    //   no else with args
    (
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };
    //   no else with i18n and args
    (
        $i18n:ident,
        $( #[$if_cfgs:meta] )*
        if $condition:tt {
            $text_id:literal
        } $(else if $else_if_condition:tt {
            $else_if_text_id:literal
        })*,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        compile_error!("Expected `else` branch")
    };

    ($text_id:expr$(,)?) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
    (
        $text_id:expr,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
    ($i18n:expr, $text_id:expr$(,)?) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
    (
        $i18n:expr,
        $text_id:expr,
        $( #[$args_cfgs:meta] )*
        {$($key:literal => $value:expr),*$(,)?}
        $(,)?
    ) => {
        compile_error!(format!("Expected a string literal, got an expression '{}'", stringify!($text_id)))
    };
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[doc(hidden)]
pub fn language_from_str_between_languages(
    code: &str,
    languages: &'static [&Language],
) -> Option<&'static Language> {
    #[cfg(feature = "tracing")]
    tracing::trace!(
        concat!(
            "Searching for language with code \"{}\".\n",
            " Available languages: {}",
        ),
        code,
        languages
            .iter()
            .map(|lang| format!("\"{}\"", lang.id))
            .collect::<Vec<_>>()
            .join(", ")
    );

    match LanguageIdentifier::from_str(code) {
        Ok(target_lang) => match languages
            .iter()
            .find(|lang| lang.id.matches(&target_lang, false, false))
        {
            Some(lang) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Language with code \"{}\" found with exact search: \"{}\"",
                    code,
                    lang.id
                );

                Some(lang)
            }
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
                    Some(lang) => {
                        #[cfg(feature = "tracing")]
                        tracing::trace!(
                            "Language with code \"{}\" found with fuzzy search: \"{}\"",
                            code,
                            lang.id
                        );

                        Some(lang)
                    }
                    None => {
                        #[cfg(feature = "tracing")]
                        tracing::trace!(
                            "Language with code \"{}\" not found",
                            code
                        );

                        None
                    }
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

/// Parameters passed to `leptos_fluent!` macro at creation of `i18n` context
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct LeptosFluentMeta {
    pub locales: &'static str,
    pub core_locales: Option<&'static str>,
    pub languages: Option<&'static str>,
    pub default_language: Option<&'static str>,
    pub translations: bool,       // *
    pub check_translations: bool, // * (maybe bool or str) TODO: improve
    pub fill_translations: Option<&'static str>,
    pub provide_meta_context: bool,
    pub sync_html_tag_lang: bool,
    pub sync_html_tag_dir: bool,
    pub url_param: &'static str,
    pub initial_language_from_url_param: bool,
    pub initial_language_from_url_param_to_local_storage: bool,
    pub initial_language_from_url_param_to_session_storage: bool,
    pub initial_language_from_url_param_to_cookie: bool,
    pub initial_language_from_url_param_to_server_function: bool, // *
    pub set_language_to_url_param: bool,
    pub local_storage_key: &'static str,
    pub initial_language_from_local_storage: bool,
    pub initial_language_from_local_storage_to_cookie: bool,
    pub initial_language_from_local_storage_to_session_storage: bool,
    pub initial_language_from_local_storage_to_server_function: bool, // *
    pub set_language_to_local_storage: bool,
    pub session_storage_key: &'static str,
    pub initial_language_from_session_storage: bool,
    pub initial_language_from_session_storage_to_cookie: bool,
    pub initial_language_from_session_storage_to_local_storage: bool,
    pub initial_language_from_session_storage_to_server_function: bool, // *
    pub set_language_to_session_storage: bool,
    pub initial_language_from_navigator: bool,
    pub initial_language_from_navigator_to_local_storage: bool,
    pub initial_language_from_navigator_to_session_storage: bool,
    pub initial_language_from_navigator_to_cookie: bool,
    pub initial_language_from_navigator_to_server_function: bool, // *
    pub set_language_from_navigator: bool,
    pub initial_language_from_accept_language_header: bool,
    pub cookie_name: &'static str,
    pub cookie_attrs: &'static str,
    pub initial_language_from_cookie: bool,
    pub initial_language_from_cookie_to_local_storage: bool,
    pub initial_language_from_cookie_to_session_storage: bool,
    pub initial_language_from_cookie_to_server_function: bool, // *
    pub set_language_to_cookie: bool,
    pub initial_language_from_server_function: bool, // *
    pub initial_language_from_server_function_to_cookie: bool,
    pub initial_language_from_server_function_to_local_storage: bool,
    pub set_language_to_server_function: bool, // *
    pub url_path: bool,                        // *
    pub initial_language_from_url_path: bool,
    pub initial_language_from_url_path_to_cookie: bool,
    pub initial_language_from_url_path_to_local_storage: bool,
    pub initial_language_from_url_path_to_session_storage: bool,
    pub initial_language_from_url_path_to_server_function: bool, // *
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
    // * not really bools but not usable as functions
}
