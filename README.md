# leptos-fluent

<!-- This file has been autogenerated.
To update it, change the content of `leptos-fluent/src/lib.rs`
and run `pre-commit run -a cargo-readme`
-->

[![Crates.io](https://img.shields.io/crates/v/leptos-fluent?logo=rust)](https://crates.io/crates/leptos-fluent)
[![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
[![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/ci.yml?label=tests&logo=github)](https://github.com/mondeja/leptos-fluent/actions)
[![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)][documentation]
![Crates.io downloads](https://img.shields.io/crates/d/leptos-fluent)

Internationalization framework for [Leptos] using [fluent-templates].

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
leptos-fluent = "0.0.34"
fluent-templates = "0.9"

[features]
hydrate = [
  "leptos-fluent/hydrate"
]
ssr = [
  "leptos-fluent/ssr",
  "leptos-fluent/actix",  # actix and axum are supported
]
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

With Fluent files _en.ftl_ and _es.ftl_:

```ftl
hello-world = Hello, world!
hello-args = Hello, { $arg1 } and { $arg2 }!
```

```ftl
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
    leptos_fluent! {{
        // Path to the locales directory, relative to Cargo.toml file.
        locales: "./locales",
        // Static translations struct provided by fluent-templates.
        translations: TRANSLATIONS,
        // Check translations correctness in the specified files.
        check_translations: "./src/**/*.rs",

        // Client side options
        // -------------------
        // Synchronize `<html lang="...">` attribute with the current
        // language using `leptos::create_effect`. By default, it is `false`.
        sync_html_tag_lang: true,
        // URL parameter name to use discovering the initial language
        // of the user. By default is `"lang"`.
        url_param: "lang",
        // Discover the initial language of the user from the URL.
        // By default, it is `false`.
        initial_language_from_url_param: true,
        // Set the discovered initial language of the user from
        // the URL in local storage. By default, it is `false`.
        initial_language_from_url_param_to_localstorage: true,
        // Update the language on URL parameter when using the method
        // `I18n.set_language`. By default, it is `false`.
        set_language_to_url_param: true,
        // Name of the field in local storage to get and set the
        // current language of the user. By default, it is `"lang"`.
        localstorage_key: "language",
        // Get the initial language from local storage if not found
        // in an URL param. By default, it is `false`.
        initial_language_from_localstorage: true,
        // Update the language on local storage when using the method
        // `I18n.set_language`. By default, it is `false`.
        set_language_to_localstorage: true,
        // Get the initial language from `navigator.languages` if not
        // found in the local storage. By default, it is `false`.
        initial_language_from_navigator: true,

        // Server side options
        // -------------------
        // Set the initial language from the Accept-Language header of the
        // request. By default, it is `false`.
        initial_language_from_accept_language_header: true,

        // Server and client side options
        // ------------------------------
        // Name of the cookie to get and set the current language of the user.
        // By default, it is `"lf-lang"`.
        cookie_name: "lang",
        // Get the initial language from cookie. By default, it is `false`.
        initial_language_from_cookie: true,
        // Update the language on cookie when using the method `I18n.set_language`.
        // By default, it is `false`.
        set_language_to_cookie: true,
    }};

    view! {
        <ChildComponent />
        <LanguageSelector />
    }
}

#[component]
fn ChildComponent() -> impl IntoView {
    // Use `tr!` and `move_tr!` macros to translate strings:
    view! {
        <p>
            <span>{move || tr!("hello-world")}</span>
            <span>{move_tr!("hello-args", {
                "arg1" => "value1",
                "arg2" => "value2",
            })}</span>
        </p>
    }

    // You must use `tr!` inside a reactive context or the translation
    // will not be updated on the fly when the current language changes.
}

#[component]
fn LanguageSelector() -> impl IntoView {
    // Use `expect_i18n()` to get the current i18n context:
    let i18n = expect_i18n();

    // `i18n.languages` is a static array with the available languages
    // `i18n.language.get()` to get the current language
    // `i18n.language.set(lang)` to set the current language
    // `lang.is_active()` to check if a language is the current selected one

    view! {
        <fieldset>
            {
                move || i18n.languages.iter().map(|lang| {
                    view! {
                        <div>
                            <input
                                type="radio"
                                id=lang
                                name="language"
                                value=lang
                                checked=lang.is_active()
                                on:click=move |_| i18n.language.set(lang)
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
- **JSON languages file**: `json` (enabled by default)
- **YAML languages file**: `yaml`
- **JSON5 languages file**: `json5`

## Resources

- [Quickstart]
- [Examples]
- [Documentation]

## Roadmap

Leptos-fluent is currently ready for most use cases. However, it is still in an
early stage of development and the API may contain breaking changes through
v0.0.X releases.

[leptos]: https://leptos.dev/
[fluent-templates]: https://github.com/XAMPPRocky/fluent-templates
[quickstart]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.leptos_fluent.html
[examples]: https://github.com/mondeja/leptos-fluent/tree/master/examples
[documentation]: https://docs.rs/leptos-fluent
