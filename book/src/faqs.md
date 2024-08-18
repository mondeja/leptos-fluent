<!-- markdownlint-disable MD001 -->

# FAQs

<!-- toc -->

### How to get [`LanguageIdentifier`]?

```rust
use fluent_templates::LanguageIdentifier;
```

```admonish tip
`fluent-templates` also depends externally on
[`fluent-bundle`](https://docs.rs/fluent-bundle/latest/fluent_bundle/)
whichs provides utilities for parsing the Fluent syntax.
```

[`LanguageIdentifier`]: https://docs.rs/unic-langid/latest/unic_langid/struct.LanguageIdentifier.html

### How to get the [i18n context] at initialization?

```rust
use leptos_fluent::leptos_fluent;

let i18n = leptos_fluent! {{
    // ...
}};

leptos::logging::log!("i18n context: {i18n:?}");
```

[i18n context]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html

### Custom [cookie attributes] are invalid

Use an expression to set the cookie attributes and will not be validated.

```rust
let attrs = "SameSite=Strict; MyCustomAttr=MyCustomValue;";
leptos_fluent! {{
    cookie_attrs: attrs,
    // ...
}}
```

[cookie attributes]: https://developer.mozilla.org/docs/Web/API/Document/cookie#write_a_new_cookie

### How to get the fallback language

From fluent-templates `v0.10` onwards can be obtained from your translations.

```rust
let fallback_language = expect_i18n().translations.get()[0].fallback();
```

### Why examples don't use [`<For/>`] component?

```admonish bug
There are some cases in which the [`<For/>`] component is not reproducible between
SSR and hydrate modes leading to different renders, so decided to use a
simple vector to not bring confusion to main examples.
```

In any case, the [`<For/>`] component is safe on CSR contexts and
[`leptos_fluent::Language`] implement `Hash` and `Eq` traits to be
able to be passed directly to `key`s properties trigerring reactivity
depending on the current active language.

```rust
use leptos_fluent::{i18n, Language};

leptos::logging::warn!("[WARNING]: Not secure on SSR");
view! {
    <p>{move_tr!("select-a-language")}</p>
    <For
        each=move || i18n().languages
        key=|lang| *lang
        children=move |lang| render_language(lang)
    />
}

fn render_language(lang: &'static Language) -> impl IntoView { ... }
```

### How to manage translations on server actions

The translations reside on the client side, so the [`I18n`] can not be
accessed as context on server actions. Pass the translations as values
if the bandwidth is not a problem or use your own statics on server side.

```rust
use leptos::prelude::*;
use leptos_fluent::{tr, Language};

/// Server action showing client-side translated message on console
#[server(ShowHelloWorld, "/api")]
pub async fn show_hello_world(
    translated_hello_world: String,
    language: String,
) -> Result<(), ServerFnError> {
    println!("{translated_hello_world} ({language})");
    Ok(())
}

fn render_language(lang: &'static Language) -> impl IntoView {
    // Call on click to server action with a client-side translated
    // "hello-world" message
    let on_click = move |_| {
        lang.activate();
        spawn_local(async {
            _ = show_hello_world(
                tr!("hello-world"),
                lang.name.to_string(),
            ).await;
        });
    };

    view! {
        <div>
            <label for=lang>{lang.name}</label>
            <input
                id=lang
                name="language"
                value=lang
                checked=lang.is_active()
                on:click=on_click
                type="radio"
            />
        </div>
    }
}
```

### How to get values of `leptos_fluent!` macro at runtime?

Use `provide_meta_context` at the macro initialization and get them
with the method `I18n::meta`:

```rust
let i18n = leptos_fluent! {{
    // ...
    provide_meta_context: true,
}};

println!("Macro parameters: {:?}", i18n.meta().unwrap());
```

### [Configuration conditional checks]

```rust
leptos_fluent! {{
    // ...
    #[cfg(debug_assertions)]
    set_language_to_url_param: true,
    #[cfg(not(debug_assertions))]
    set_language_to_url_param: false,
}}
```

[configuration conditional checks]: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
[`<For/>`]: https://docs.rs/leptos/latest/leptos/fn.For.html
[`leptos_fluent::Language`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.Language.html
[`I18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html
