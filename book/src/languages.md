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

This standard enforces that an user will always be able to select their
language in their own language, and not in the current language of the
application.

Note that the order of the languages will be defined based on the alphabetical
order of their names, not their codes.

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
  // Code,     Name,            "ltr"/"rtl"/"auto" (optional)
  ["code", "Language name", "Writing direction"],
]
```

The order of the languages in [`leptos_fluent::I18n::languages`] will be
the same as in the file regardless of the alphabetical order of the names.

### File format

By default, the `leptos-fluent/json` feature is enabled, which only
allows to set the languages file in JSON format. If you want to use
other formats, you can disable the feature and define other feature.

```toml
[dependencies]
leptos-fluent = { version = "*", default-features = false, features = ["json5"] }
```

Available features for languages file formats are:

- `json`: JSON (default)
- `yaml`: YAML
- `json5`: JSON5

## Tracking locales file with `cargo leptos`

Using [`cargo-leptos`] the _locales/_ folder must be manually
configured to be watched:

```toml
# Relative from Cargo.toml file
[package.metadata.leptos]
watch-additional-files = ["locales"]
```

When inside a workspace, use the full path to the folder from the
workspace _Cargo.toml_ file:

```toml
 # Relative from workspace Cargo.toml file
[package.metadata.leptos]
watch-additional-files = ["examples/csr/locales"]
```

[ISO 639-1 code]: https://en.wikipedia.org/wiki/ISO_639-1
[`leptos_fluent::I18n::languages`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html#structfield.languages
[`cargo-leptos`]: https://github.com/leptos-rs/cargo-leptos
[`leptos_fluent!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.leptos_fluent.html
