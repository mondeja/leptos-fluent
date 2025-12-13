# Basic usage

<!-- toc -->

## Minimal skeleton

The most basic CSR app is reproduced here:

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
select-a-language = Select a language:
language-selected-is = The selected language is { $lang }.
```

```ftl
# locales/es/main.ftl
select-a-language = Selecciona un idioma:
language-selected-is = El idioma seleccionado es { $lang }.
```

```rust
// src/lib.rs
use leptos::prelude::*;
use leptos_fluent::{I18n, leptos_fluent, move_tr, Language};

#[component]
pub fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "en",
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18nProvider>
            <LanguageSelector/>
        </I18nProvider>
    }
}

#[component]
fn LanguageSelector() -> impl IntoView {
    let i18n = expect_context::<I18n>();

    view! {
        <p>{move_tr!("select-a-language")}</p>
        <fieldset>
            {move || {
                i18n.languages.iter().map(|lang| render_language(lang)).collect::<Vec<_>>()
            }}
        </fieldset>
        <p>
            {move_tr!(
                 "language-selected-is",
                 { "lang" => i18n.language.get().name }
            )}
        </p>
    }
}

fn render_language(lang: &'static Language) -> impl IntoView {
    let i18n = expect_context::<I18n>();

    // Passed as atrribute, `Language` is converted to their code,
    // so `<input id=lang` becomes `<input id=lang.id.to_string()`
    view! {
        <div>
            <label for=lang>{lang.name}</label>
            <input
                id=lang
                value=lang
                name="language"
                checked=i18n.language.get() == lang
                on:click=move |_| i18n.language.set(lang)
                type="radio"
            />
        </div>
    }
}
```

```rust
// src/main.rs
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(minimal_example::App);
}
```

```toml
# Cargo.toml
[package]
name = "minimal-example"
edition = "2021"
version = "0.1.0"

[lib]
name = "minimal_example"
path = "src/lib.rs"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos-fluent = "0.2"
console_error_panic_hook = "0.1"

# Using cargo-leptos
[package.metadata.leptos]
watch-additional-files = ["locales"]
```

## Translating messages

Use the [`move_tr!`] macro to translate a string. The macro takes the key of the
translation and an optional object with the variables to interpolate:

```rust
move_tr!("select-a-language")

move_tr!("language-selected-is", { "lang" => i18n.language.get().name })
```

Additionally, use the [`tr!`] macro to translate a string inside
a reactive context. Note that if is not inside a reactive context,
the translation won't be updated on the fly when the language changes.
This can lead to warnings in console output like:

```admonish warning

    At `./path/to/file.rs:ln`, you access a signal or memo (defined at
    `./path/to/file.rs:ln`) outside of a reactive context. This might mean your
    app is not responding to changes in signal values in the way you expect.

```

Can be fixed by replacing calls to [`tr!`] with [`move_tr!`] or wrapping the
[`tr!`] calls in a reactive context.

The previous code _could_ be rewritten as:

```rust
move || tr!("select-a-language")

move || tr!("language-selected-is", { "lang" => i18n.language.get().name })
```

The main difference is that [`move_tr!`] encapsulates the movement in a
[`leptos::Signal`], strictly would be rewritten as:

```rust
Signal::derive(move || tr!("select-a-language"))
```

## Retrieving the [`I18n`] context

```rust
use leptos::prelude::expect_context;
use leptos_fluent::I18n;
```

```rust
let i18n = expect_context::<I18n>();
```

```rust
let i18n = use_context::<I18n>().expect("No `I18n` context found");
```

## Using the [`I18n`] context

The i18n context has the following fields:

- [`language`]: A read-write signal with a pointer to the static current active language.
- [`languages`]: A pointer to a static list of pointers of the static available languages.
- [`translations`]: A signal to the vector of [fluent-templates] loaders that stores
  the translations.

### Update language

To update the language, use the `set` method of [`language`]:

```rust
use leptos::prelude::expect_context;
use leptos_fluent::I18n;

let i18n = expect_context::<I18n>();
i18n.language.set(lang);
```

```admonish tip title='Nightly'
When `nightly` feature is enabled, can be updated passing a new language to the
context as a function with:

    let i18n = expect_context::<I18n>();
    i18n(lang);

```

### Get active language

To get the current active language, use `get` method of [`language`] field:

```rust
use leptos::prelude::*;
use leptos_fluent::I18n;

let i18n = expect_context::<I18n>();
let lang = i18n.language.get();
```

```admonish tip title='Nightly'
With `nightly` feature enabled, can get the active language calling the context
as a function:

    use leptos::prelude::*;
    use leptos_fluent::I18n;

    let i18n = expect_context::<I18n>();
    let lang = i18n();

```

### Get available languages

To get the available languages, iterate over the [`languages`] field:

```rust
i18n.languages.iter()
```

### Check if a language is active

To check if a language is the active one, use:

```rust
use leptos::prelude::*;
use leptos_fluent::I18n;

let i18n = expect_context::<I18n>();
lang == i18n.language.get()
```

[`tr!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.tr.html
[`move_tr!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.move_tr.html
[`I18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html
[`language`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.language
[`languages`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.languages
[`translations`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.translations
[fluent-templates]: https://docs.rs/fluent-templates/latest/fluent_templates
[`leptos::Signal`]: https://docs.rs/reactive_graph/0.1.0/reactive_graph/wrappers/read/struct.Signal.html
