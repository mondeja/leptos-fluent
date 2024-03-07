# leptos-fluent

[![Crates.io](https://img.shields.io/crates/v/leptos-fluent)](https://crates.io/crates/leptos-fluent)
[![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
[![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/leptos-fluent/ci.yml?label=tests&logo=github)](https://github.com/mondeja/leptos-fluent/actions)
[![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)][documentation]

Internationalization framework for [Leptos] using [fluent-templates].

## Installation

```sh
cargo add leptos leptos-fluent fluent-templates unic-langid
```

## Usage

```rust,ignore
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{leptos_fluent, tr};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        locales: "./locales",
        // Translations provided by fluent-templates.
        translations: TRANSLATIONS,
        // Synchronize `<html lang="...">` attribute with the current
        // language using `leptos::create_effect`. By default, it is `false`.
        sync_html_tag_lang: true,
        // Discover the initial language of the user from the URL.
        // By default, it is `false`.
        initial_language_from_url: true,
        // URL parameter name to use discovering the initial language
        // of the user. By default is `"lang"`.
        initial_language_from_url_param: "lang",
        // Set the discovered initial language of the user from
        // the URL in local storage. By default, it is `false`.
        initial_language_from_url_to_localstorage: true,
        // Get the initial language from local storage if not found
        // in an URL param. By default, it is `false`.
        initial_language_from_localstorage: true,
        // Get the initial language from `navigator.languages` if not
        // found in the local storage. By default, it is `false`.
        initial_language_from_navigator: true,
        // Name of the field in local storage to get and set the
        // current language of the user. By default, it is `"lang"`.
        localstorage_key: "language",
    }};

    view! {
        <OtherComponent />
    }
}

#[component]
fn OtherComponent() -> impl IntoView {
    view! {
        <p>
            <span>{move || tr!("foo")}</span>
            <span>{move || tr!("bar-with-args", {
                "arg1" => "value1",
                "arg2" => "value2",
            })}</span>
        </p>
    }
}
```

## Resources

- [Quickstart]
- [Examples]
- [Documentation]

## Current state

Leptos-fluent is currently ready to use for most use cases. However, it is still in an early stage of development and the API may contain breaking changes in v0.0.X releases. I'm trying to release
the API at v0.1.0 as stable as possible.

[leptos]: https://leptos.dev/
[fluent-templates]: https://github.com/XAMPPRocky/fluent-templates
[quickstart]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.leptos_fluent.html
[examples]: https://github.com/mondeja/leptos-fluent/tree/master/examples
[documentation]: https://docs.rs/leptos-fluent
