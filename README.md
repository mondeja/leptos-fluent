# leptos-fluent

<!-- This file has been autogenerated.
To update it, change the content of `leptos-fluent/src/lib.rs`
and run `pre-commit run -a cargo-readme`
-->

[![Crates.io](https://img.shields.io/crates/v/leptos-fluent?logo=rust)](https://crates.io/crates/leptos-fluent)
[![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
[![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/ci.yml?label=tests&logo=github)](https://github.com/mondeja/leptos-fluent/actions)
[![Book](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/.github%2Fworkflows%2Fci.yml?logo=github&label=book)](https://mondeja.github.io/leptos-fluent/)
[![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)](https://docs.rs/leptos-fluent)
[![Crates.io downloads](https://img.shields.io/crates/d/leptos-fluent)](https://crates.io/crates/leptos-fluent)
[![Discord channel](https://img.shields.io/badge/discord-grey?logo=discord&logoColor=white)](https://discord.com/channels/1031524867910148188/1251579884371705927)

Internationalization framework for [Leptos] using [fluent-templates].

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
leptos-fluent = "0.1"
fluent-templates = "0.11"

[features]
hydrate = [
  "leptos-fluent/hydrate"
]
ssr = [
  "leptos-fluent/ssr",
  "leptos-fluent/actix",  # actix and axum are supported
]
```

If you're using `cargo-leptos` to build your project, watch the
_locales/_ folder with:

```toml
[package.metadata.leptos]
watch-additional-files = ["locales"]  # Relative to Cargo.toml
```

## Usage

Giving the following directory structure:

```plaintext
.
├── 📄 Cargo.toml
├── 📁 locales
│   ├── 📁 en
│   │   └── 📄 main.ftl
│   └── 📁 es
│       └── 📄 main.ftl
└── 📁 src
    ├── 📄 main.rs
    └── 📄 lib.rs
```

```ftl
# locales/en/main.ftl
hello-world = Hello, world!
hello-args = Hello, { $arg1 } and { $arg2 }!
```

```ftl
# locales/es/main.ftl
hello-world = ¡Hola, mundo!
hello-args = ¡Hola, { $arg1 } y { $arg2 }!
```

You can use `leptos-fluent` as follows:

```rust
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
fn App() -> impl IntoView {
    // See all options in the reference at
    // https://mondeja.github.io/leptos-fluent/leptos_fluent.html
    leptos_fluent! {
        // Path to the locales directory, relative to Cargo.toml.
        locales: "./locales",
        // Static translations struct provided by fluent-templates.
        translations: [TRANSLATIONS],
        // Check translations correctness in the specified files.
        #[cfg(debug_assertions)]
        check_translations: "./src/**/*.rs",

        // Next options are all opt-in and can be enabled
        // separately as needed.

        // Client side options
        // -------------------
        // Synchronize `<html lang="...">` attribute with
        // current active language.
        sync_html_tag_lang: true,
        // Synchronize `<html dir="...">` attribute with `"ltr"`,
        // `"rtl"` or `"auto"` depending on active language.
        sync_html_tag_dir: true,
        // Update language on URL parameter when changes.
        set_language_to_url_param: true,
        // Set initial language of user from URL in local storage.
        initial_language_from_url_param_to_localstorage: true,
        // Set initial language of user from URL in a cookie.
        initial_language_from_url_param_to_cookie: true,
        // Key used to get and set the current language of the
        // user on local storage. By default is `"lang"`.
        localstorage_key: "language",
        // Get initial language from local storage if not found
        // in an URL param.
        initial_language_from_localstorage: true,
        // Set the initial language of the user from
        // local storage to a cookie.
        initial_language_from_localstorage_to_cookie: true,
        // Update language on local storage when changes.
        set_language_to_localstorage: true,
        // Get initial language from `navigator.languages`
        // if not found in local storage.
        initial_language_from_navigator: true,
        // Set initial language of user from navigator to local storage.
        initial_language_from_navigator_to_localstorage: true,
        // Set initial language of user from navigator to a cookie.
        initial_language_from_navigator_to_cookie: true,
        // Attributes to set for language cookie.
        // By default `""`.
        cookie_attrs: "Secure; Path=/; Max-Age=600",
        // Update language on cookie when the language changes.
        set_language_to_cookie: true,
        // Set initial language from a cookie to local storage.
        initial_language_from_cookie_to_localstorage: true,

        // Server side options
        // -------------------
        // Set initial language from the `Accept-Language`
        // header of the request.
        initial_language_from_accept_language_header: true,

        // Server and client side options
        // ------------------------------
        // Name of the cookie to get and set the current active
        // language. By default `"lf-lang"`.
        cookie_name: "lang",
        // Set initial language from cookie.
        initial_language_from_cookie: true,
        // URL parameter to use setting the language in the URL.
        // By default `"lang"`.
        url_param: "lang",
        // Set initial language of the user from an URL parameter.
        initial_language_from_url_param: true,

        // Desktop applications (feature `system`)
        // ---------------------------------------
        // Set initial language from the system locale.
        initial_language_from_system: true,
        // Set initial language of the user from
        // the system locale to a data file.
        initial_language_from_system_to_data_file: true,
        // Get initial language from a data file.
        initial_language_from_data_file: true,
        // Key to use to name the data file. Should be unique per
        // application. By default `"leptos-fluent"`.
        data_file_key: "my-app",
        // Set the language selected to a data file.
        set_language_to_data_file: true,
    };

    view! {
        <TranslatableComponent />
        <LanguageSelector />
    }
}

#[component]
fn TranslatableComponent() -> impl IntoView {
    // Use `tr!` and `move_tr!` macros to translate strings:
    view! {
        <p>
            <span>{move || tr!("hello-world")}</span>
            <span>{move_tr!("hello-args", {
                "arg1" => "foo",
                "arg2" => "bar",
            })}</span>
        </p>
    }

    // The `tr!` macro must be inside a reactive context or the
    // translation will not be updated on the fly when the language changes.
}

#[component]
fn LanguageSelector() -> impl IntoView {
    // `expect_i18n()` to get the i18n context
    // `i18n.languages` is a static array with the available languages
    // `i18n.language.get()` to get the current language
    // `lang.activate()` to set the current language
    // `lang.is_active()` to check if a language is the current selected one

    view! {
        <fieldset>
            {
                move || expect_i18n().languages.iter().map(|lang| {
                    view! {
                        <div>
                            <input
                                type="radio"
                                id=lang
                                name="language"
                                value=lang
                                checked=lang.is_active()
                                on:click=move |_| lang.activate()
                            />
                            <label for=lang>{lang.name}</label>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </fieldset>
    }
}
```

### Features

- **Server Side Rendering**: `ssr`
- **Hydration**: `hydrate`
- **Actix Web integration**: `actix`
- **Axum integration**: `axum`
- **Nightly toolchain**: `nightly`
- **Desktop applications**: `system`
- **JSON languages file**: `json`
- **YAML languages file**: `yaml`
- **JSON5 languages file**: `json5`
- **Tracing support**: `tracing`
- **Debugging**: `debug`

## Resources

- [Book]
- [Quickstart]
- [Documentation]
- [Examples]

[leptos]: https://leptos.dev/
[fluent-templates]: https://github.com/XAMPPRocky/fluent-templates
[quickstart]: https://mondeja.github.io/leptos-fluent/leptos_fluent.html
[examples]: https://github.com/mondeja/leptos-fluent/tree/master/examples
[book]: https://mondeja.github.io/leptos-fluent/
[documentation]: https://docs.rs/leptos-fluent
