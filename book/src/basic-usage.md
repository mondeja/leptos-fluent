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
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, Language};

static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "./locales",
    }};

    view! { <LanguageSelector/> }
}

#[component]
fn LanguageSelector() -> impl IntoView {
    // Use `expect_i18n()` to get the current i18n context:
    let i18n = expect_i18n();

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
    // Passed as atrribute, `Language` is converted to their code,
    // so `<input id=lang` becomes `<input id=lang.id.to_string()`
    view! {
        <div>
            <label for=lang>{lang.name}</label>
            <input
                id=lang
                value=lang
                name="language"
                checked=lang.is_active()
                on:click=move |_| lang.activate()
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
    leptos::mount_to_body(minimal_example::App);
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
leptos = { version = "0.6.12", features = ["csr"] }
leptos-fluent = "0.1"
fluent-templates = "0.9"
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

Additionally, you can use the [`tr!`] macro to translate a string inside
a reactive context. Note that if you're not inside a reactive context,
the translation won't be updated on the fly when the language changes.
This can lead to warnings in console output like:

````admonish warning
```shell
At `./path/to/file.rs:ln`, you access a signal or memo (defined at
`./path/to/file.rs:ln`) outside of a reactive context. This might mean your
app is not responding to changes in signal values in the way you expect.
```
````

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
leptos::Signal::derive(move || tr!("select-a-language"))
```

## Retrieving the [`I18n`] context

Use the [`expect_i18n`] function to get the current i18n context:

```rust
let i18n = leptos_fluent::expect_i18n();
```

It is exported as [`i18n`][`i18n`-f] too:

```rust
let i18n = leptos_fluent::i18n();
```

The function [`use_i18n`] returns an `Option` with the current i18n context:

```rust
let i18n = leptos_fluent::use_i18n().expect("No `I18n` context found");
```

## Using the [`I18n`] context

The i18n context has the following fields:

- [`language`]: A read-write signal with a pointer to the static current active language.
- [`languages`]: A pointer to a static list of pointers of the static available languages.
- [`translations`]: A signal to the vector of [fluent-templates] loaders that stores
  the translations.

### Update language

To update the language, use [`lang.activate`] or the `set` method of [`language`]:

```rust
lang.activate();

expect_i18n().language.set(lang);
```

````admonish tip title='Nightly'
When `nightly` feature is enabled, you update it passing a new language to the
context as a function with:

```rust
let i18n = leptos_fluent::i18n();
i18n(lang);
```
````

### Get active language

To get the current active language, use `get` method of [`language`] field:

```rust
let i18n = leptos_fluent::i18n();
let lang = i18n.language.get();
```

````admonish tip title='Nightly'
When `nightly` enabled, you can get the active language with:

```rust
let i18n = leptos_fluent::i18n();
let lang = i18n();
```
````

### Get available languages

To get the available languages, iterate over the [`languages`] field:

```rust
i18n.languages.iter()
```

### Check if a language is active

To check if a language is the active one, use [`is_active`] method of a
[`leptos_fluent::Language`] struct:

```rust
lang.is_active()
```

[`tr!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.tr.html
[`move_tr!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.move_tr.html
[`I18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html
[`leptos_fluent::Language`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.Language.html
[`lang.activate`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.Language.html#method.activate
[`language`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.language
[`languages`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.languages
[`translations`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.translations
[fluent-templates]: https://docs.rs/fluent-templates/latest/fluent_templates
[`leptos::Signal`]: https://docs.rs/leptos/latest/leptos/struct.Signal.html
[`expect_i18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/fn.expect_i18n.html
[`i18n`-f]: https://docs.rs/leptos-fluent/latest/leptos_fluent/fn.i18n.html
[`use_i18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/fn.use_i18n.html
[`is_active`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.Language.html#method.is_active
