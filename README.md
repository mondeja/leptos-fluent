# leptos-fluent

[![Crates.io](https://img.shields.io/crates/v/leptos-fluent)](https://crates.io/crates/leptos-fluent)
[![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
[![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)](https://docs.rs/leptos-fluent)

Internationalization framework for [Leptos] using [fluent-templates].

## Installation

```sh
cargo add leptos leptos-fluent fluent-templates unic-langid
```

## Quickstart

````rust,ignore
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{i18n, leptos_fluent, I18n, Language};
use std::collections::HashMap;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
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
        languages: "./locales/languages.json",
        // Synchronize <html lang="..."> attribute with the current language
        // using `leptos::create_effect`.
        // By default, it is `false`.
        sync_html_tag_lang: true,
        // Discover the initial language of the user from the URL. By default, the
        // name of the URL parameter is `"lang"`. See `initial_language_from_url_param`.
        // By default, it is `false`.
        initial_language_from_url: true,
        // URL parameter name to use discovering the initial language of the user.
        // By default is `"lang"`.
        initial_language_from_url_param: "lang",
        // Set the discovered initial language of the user from the URL in
        // the local storage. By default, it is `false`.
        initial_language_from_url_to_localstorage: true,
        // Get the initial language from the local storage if not found in an URL pram.
        // By default, it is `false`.
        initial_language_from_localstorage: true,
        // Get the initial language from `navigator.languages` if not found in the local
        // storage. By default, it is `false`.
        initial_language_from_navigator: true,
        // Name of the field in the local storage to get and set the current language
        // of the user. By default, it is `"lang"`.
        localstorage_key: "lang",
    }};
    ctx.provide_context(None);

    // You can pass a `Some(&'static Language)` to `provide_context`
    // to set the initial language manually.
    let initial_language = move |ctx: &I18n| {
        // Get the initial language of the user from a server, for example.
        // ...
        ctx.default_language()
    };

    // ctx.provide_context(Some(initial_language(&ctx)));

    view! {
        <OtherComponent />
    }
}

#[component]
fn OtherComponent() -> impl IntoView {
    view! {
        <p>
            <span>{move || i18n().tr("foo")}</span>
            <span>{move || i18n().trs("bar-with-args"), &{
                let mut map = HashMap::new();
                map.insert("arg1".to_string(), "value1".into());
                map.insert("arg2".to_string(), "value2".into());
                map
            }}</span>
        </p>
    }
}
````

[leptos]: https://leptos.dev/
[fluent-templates]: https://github.com/XAMPPRocky/fluent-templates
