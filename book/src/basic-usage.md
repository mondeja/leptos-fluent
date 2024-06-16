# Basic usage

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
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr};

static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: TRANSLATIONS,
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
        <p>
            {
                move || {
                    tr!(
                        "language-selected-is",
                        { "lang" => i18n.language.get().name }
                    )
                }
            }
        </p>
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
```

## Translating messages

Use the `move_tr!` macro to translate a string. The macro takes the key of the
translation and an optional object with the variables to interpolate:

```rust
move_tr!("select-a-language")

move_tr!("language-selected-is", { "lang" => i18n.language.get().name })
```

Additionally, you can use the `tr!` macro to translate a string inside
a reactive context. Note that if you're not inside a reactive context,
the translation won't be updated on the fly when the language changes:

The previous code _could_ be rewritten as:

```rust
move || tr!("select-a-language")

move || tr!("language-selected-is", { "lang" => i18n.language.get().name })
```

The main difference is that `move_tr!` encapsulates the movement in a
`leptos::Signal`, strictly would be rewritten as:

```rust
leptos::Signal::derive(move || tr!("select-a-language"))
```

## Retrieving the i18n context

Use the `expect_i18n` function to get the current i18n context:

```rust
let i18n = expect_i18n();
```

It is exported as `i18n()` too:

```rust
let i18n = leptos_fluent::i18n();
```

With the function `use_i18n()` context you'll get an `Option` with the current
i18n context:

```rust
use leptos_fluent::use_i18n;

let i18n = use_i18n().expect("No i18n context found");
```

## Using the i18n context

The i18n context has the following fields:

- `language`: A read-write signal with a pointer to the static current active language.
- `languages`: A pointer to a static list of pointers of the static available languages.
- `translations`: A pointer to the [fluent-templates] loader that stores the translations.

[fluent-templates]: https://docs.rs/fluent-templates/latest/fluent_templates

To update the language, use the `set` method of the `language` field:

```rust
i18n.language.set(lang)
```

To get the current active language, use the `get` method of the `language` field:

```rust
i18n.language.get()
```

To get the available languages, iterate over the `languages` field:

```rust
i18n.languages.iter()
```

To check if a language is the active one, use the `is_active` method of a
`Language` struct:

```rust
lang.is_active()
```
