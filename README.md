# leptos-fluent

[![Crates.io](https://img.shields.io/crates/v/leptos-fluent)](https://crates.io/crates/leptos-fluent)
[![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)

Internationalization framework for [Leptos] using [fluent-templates].

## Installation

```sh
cargo add leptos leptos-fluent fluent-templates
```

## Quickstart

````rust
use leptos::*;
use leptos_fluent::{leptos_fluent, I18n, i18n, Language};
use fluent_templates::static_loader;

static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

pub fn initial_language(ctx: &I18n) -> &'static Language {
    // Get the initial language from URL, local storage, etc.
    //
    // By default, the first in `languages.json` is used.
    ctx.default_language()
}

#[component]
pub fn App() -> impl IntoView {
    let ctx = leptos_fluent! {{
        locales: LOCALES,
        // Path to the JSON file with the list of languages in the form:
        // ```json
        // [
        //   ["en-US", "English"],
        //   ["es-ES", "Español"]
        // ]
        // ```
        languages_json: "./locales/languages.json",
        // Synchronize <html lang="..."> attribute with the current language
        sync_html_tag_lang: true,
    }};
    ctx.provide_context(initial_language(&ctx));

    view! {
        <OtherComponent />
    }
}

#[component]
fn OtherComponent() -> impl IntoView {
    view! {
        <p>{move || i18n().tr("hello-world")}</p>
    }
}
````

[leptos]: https://leptos.dev/
[fluent-templates]: https://github.com/XAMPPRocky/fluent-templates
