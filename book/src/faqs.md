<!-- markdownlint-disable MD001 -->

# FAQs

### How to get `unic_langid::LanguageIdentifier` if not installed?

```rust
use fluent_templates::LanguageIdentifier;
```

`fluent-templates` also depends externally on [`fluent-bundle`] whichs
provides utilities for parsing the Fluent syntax.

[`fluent-bundle`]: https://docs.rs/fluent-bundle/latest/fluent_bundle/

### How to get the fallback language

From fluent-templates `v0.9.5` onwards you can get it from your translations.

```rust
let fallback_language = expect_i18n().translations.get()[0].fallback();
```

### Custom cookie attributes are invalid

Use an expression to set the cookie attributes and will not be validated.

```rust
let attrs = "SameSite=Strict; MyCustomAttr=MyCustomValue;";
leptos_fluent! {{
    cookie_attrs: attrs,

    // ... other options
}}
```

### Why examples don't use `<For/>` component?

There are some cases in which the `For` component is not reproducible between
SSR and hydrate modes leading to different renders, so decided to use a
simple vector to not bring confusion to main examples.

In any case, the `<For/>` component is secure on CSR contexts and
`leptos_fluent::Language`s implement `Hash` and `Eq` traits to be
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
