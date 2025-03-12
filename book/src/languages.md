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
  [ISO 639 language code], without a region code.
- `es-ES` is built with the name `"Español (España)"` because it's defined
  as an [ISO 639 language code] and a [ISO 3166 region code].

This enforces that an user will always be able to select their language in
their own language, and not in the current language of the application.

## Inferred language names

When not using a languages file, the language names are inferred from
the language codes used for `locales` directories following the next rules:

1. If only a language code is provided, use "Language name"
   (see [full-list][list-1]).
2. If a region code is provided but the language is not repeated,
   use "Language name".
3. If a region code is provided and the language is repeated,
   use "Language name (region name)" (see [full list][list-2]).

[list-1]: https://github.com/mondeja/leptos-fluent/blob/427712b05a5d42d765967e1edf01fd4d666e8c25/leptos-fluent-macros/src/languages.rs#L1860
[list-2]: https://github.com/mondeja/leptos-fluent/blob/427712b05a5d42d765967e1edf01fd4d666e8c25/leptos-fluent-macros/src/languages.rs#L598

## The languages file

The languages array can be fully customized by defining a `languages` parameter
in the [`leptos_fluent!`] macro pointing to a languages file. This file must
be relative to the _Cargo.toml_ file.

```rust
leptos_fluent! {
    languages: "./locales/languages.json",
    // ...
}
```

```json
[
  ["en", "English"],
  ["es-ES", "Spanish (Spain)", "auto"]
]
```

### Languages file format

In order to use a languages file, you must enable the feature for the file
format you want to use in the _Cargo.toml_ file:

```toml
[dependencies]
leptos-fluent = { version = "0.2", features = ["json5"] }
```

Available features for languages file formats are:

- `json`: JSON
- `yaml`: YAML
- `json5`: JSON5

### Languages file layout

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

The first one in the languages file will used as the initial of the user when
no other initialization language is discovered. Use the same as the one
configured as `fallback_language` in `static_loader!`.

```rust
static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json5",
    };
}
```

```json5
// ./locales/languages.json5
[
  ["en", "English"],
  ["es-ES", "Español (España)"],
]
```

[ISO 639 language code]: https://en.wikipedia.org/wiki/ISO_639
[ISO 3166 region code]: https://en.wikipedia.org/wiki/ISO_3166-1
[`leptos_fluent!`]: https://mondeja.github.io/leptos-fluent/leptos_fluent.html
