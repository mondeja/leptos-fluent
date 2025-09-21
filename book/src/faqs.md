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

### Custom [cookie attributes] are invalid

Use an expression to set the cookie attributes and will not be validated.

```rust
let attrs = "SameSite=Strict; MyCustomAttr=MyCustomValue;";
leptos_fluent! {
    cookie_attrs: attrs,
    // ...
};
```

[cookie attributes]: https://developer.mozilla.org/docs/Web/API/Document/cookie#write_a_new_cookie

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
use leptos_fluent::{tr, Language, I18n};

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
    let i18n = expect_context::<I18n>();

    // Call on click to server action with a client-side translated
    // "hello-world" message
    let on_click = move |_| {
        i18n.language.set(lang);
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
                checked=i18n.language.get() == lang
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
leptos_fluent! {
    // ...
    provide_meta_context: true,
};

// ... later
let i18n = expect_context::<I18n>();
println!("Macro parameters: {:?}", i18n.meta().unwrap());
```

[`<For/>`]: https://docs.rs/leptos/latest/leptos/control_flow/fn.For.html
[`leptos_fluent::Language`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.Language.html
[`I18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html
