# Languages

**leptos-fluent** follows a default strategy to generate the languages
of the application. This strategy is based on the _locales/_ directory.

Giving the next directory structure:

```plaintext
.
└── 📁 locales
    ├── 📁 en
    │   └── 📄 main.ftl
    └── 📁 es-ES
        └── 📄 main.ftl
```

The framework will generate something like the following languages array at
compile time:

```rust
let LANGUAGES = [
  leptos_fluent::Language {
    id: unic_langid::langid!("en"),
    name: "English",
    dir: leptos_fluent::WritingDirection::Ltr,
  },
  leptos_fluent::Language {
    id: unic_langid::langid!("es-ES"),
    name: "Español (España)",
    dir: leptos_fluent::WritingDirection::Ltr,
  },
]
```

- `en` is built with the name `"English"` because it's defined as an
  [ISO 639-1 code], without a region code.
- `es-ES` is built with the name `"Español (España)"` because it's defined
  as an [ISO 639-1 code] and a region code.

This enforces that an user will always be able to select their language in
their own language, and not in the current language of the application.

```admonish abstract title='Order'
The order of the languages will be defined based on the alphabetical
order of their names, not their codes.
```

## The languages file

The languages array can be fully customized by defining a `languages` parameter
in the [`leptos_fluent!`] macro pointing to a languages file. This file must
be relative to the _Cargo.toml_ file.

```rust
leptos_fluent! {{
    languages: "./locales/languages.json",
    // ...
}}
```

```json
[
  ["en", "English"],
  ["es-ES", "Spanish (Spain)", "auto"]
]
```

The languages file must expose an array of arrays with the structure:

```json5
[
  // Code, Name,            "ltr"/"rtl"/"auto" (optional)
  ["code", "Language name", "Writing direction"],
]
```

```admonish abstract title='Order'
The order of the languages in
[`leptos_fluent::I18n::languages`](https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.languages)
will be the same as in the file regardless of the alphabetical order
of the names.
```

The first in the languages file will used as the initial of the user when any
other initialization value is discovered. Use the same as the one configured
as `fallback_language` in `static_loader!`.

```rust
static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json5",
    }};
}
```

```json5
// ./locales/languages.json5
[
  ["en", "English"],
  ["es-ES", "Español (España)"],
]
```

### File format

By default, the `json` feature is enabled, which only allows to set the
languages file in JSON format. To use other formats, disable the feature
and enable another.

```toml
[dependencies]
leptos-fluent = { version = "*", default-features = false, features = ["json5"] }
```

Available features for languages file formats are:

- `json`: JSON (default)
- `yaml`: YAML
- `json5`: JSON5

## Tracking locales files with [`cargo leptos`]

Using [`cargo leptos`] the _locales/_ folder must be configured to be watched:

```toml
# Relative to Cargo.toml file
[package.metadata.leptos]
watch-additional-files = ["locales"]
```

When inside a workspace, use the full path to the folder from the
workspace _Cargo.toml_ file:

```toml
 # Relative to workspace Cargo.toml file
[package.metadata.leptos]
watch-additional-files = ["examples/csr/locales"]
```

[ISO 639-1 code]: https://en.wikipedia.org/wiki/ISO_639-1
[`cargo leptos`]: https://github.com/leptos-rs/cargo-leptos
[`leptos_fluent!`]: https://mondeja.github.io/leptos-fluent/leptos_fluent.html
