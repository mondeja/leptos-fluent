<!-- markdownlint-disable MD033 MD038 -->

# `leptos_fluent!`

The `leptos_fluent!` macro is used to load the translations and set the current
locale. It is used in the root component of the application.

```rust
leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],
}}
```

<!-- toc -->

## Common configurations

### <span style="opacity:.5">CSR </span> | Local storage from navigator

```rust
leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],

    set_language_to_localstorage: true,
    initial_language_from_localstorage: true,
    initial_language_from_navigator: true,
    initial_language_from_navigator_to_localstorage: true,
    initial_language_from_url_param: true,
    initial_language_from_url_param_to_localstorage: true,
    localstorage_key: "lang",
}}
```

### <span style="opacity:.5">CSR + SSR </span> | Cookie from navigator and header

```rust
leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],

    set_language_to_cookie: true,
    initial_language_from_cookie: true,
    initial_language_from_navigator: true,
    initial_language_from_navigator_to_cookie: true,
    initial_language_from_url_param: true,
    initial_language_from_url_param_to_cookie: true,
    initial_language_from_accept_language_header: true,
    cookie_name: "lf-lang",
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | Data files on Desktop applications

<!-- markdownlint-enable MD013 -->

```rust
leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],

    initial_language_from_system: true,
    initial_language_from_system_to_data_file: true,
    initial_language_from_data_file: true,
    set_language_to_data_file: true,
    data_file_key: "system-language-example",
}}
```

## Processing steps

There is four kind of parameters for all the possible configurations and are
executed in the next order:

### Order

1. Get the initial languaje from a source or target: `initial_language_from_*`
2. Obtain the initial language and set to a target: `initial_language_from_*_to_*`
3. Synchronize the current language with a target: `set_language_to_*`
4. The name of a source or a target: `cookie_name`, `localstorage_key`, `navigator`...

### Sources and targets

Sources are read-only and targets are read-write.

- Sources: `navigator`, `system`, `accept_language_header`
- Targets: `cookie_name`, `localstorage_key`, `url_param`, `data_file`

### Commented example

````admonish example
```rust
leptos_fluent! {{
    // ..
    // Get the initial language from the source operative system
    initial_language_from_system: true,
    // and set to the target file.
    initial_language_from_system_to_data_file: true,
    // If a target data file exists, get the initial language from it.
    initial_language_from_data_file: true,
    // When the language is updated, set it to the file.
    set_language_to_data_file: true,
    // Unique file name to set the language for this app:
    data_file_key: "system-language-example",
}};
```
````

## Parameters

### `translations`

Set the translations to be used in the application. It must be a reference
to a static array of [`fluent_templates::static_loader!`] instances.

```rust
use fluent_templates::static_loader;
use leptos_fluent::leptos_fluent;

static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],
    // ^^^^^^^^^^^^^^^^^^^^^^^^^
}}
```

Must be the same identifier used in the [`fluent_templates::static_loader!`]
macro, which returns an [`once_cell:sync::Lazy`] variable.

### `locales`

Set the path to the locales directory which contain the Fluent
files for the translations. Must be relative to the _Cargo.toml_ file, the same
used in the [`fluent_templates::static_loader!`] macro.

```rust
leptos_fluent! {{
    locales: "./locales",
    // ^^^^^^^^^^^^^^^^^
    translations: [TRANSLATIONS],
}}
```

### `core_locales`

Common locale resources that are shared across all locales.
Must be relative to the _Cargo.toml_ file, the same
used in the [`fluent_templates::static_loader!`] macro:

```rust
static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        core_locales: "./locales/core",
    };
}

leptos_fluent! {{
    locales: "./locales",
    core_locales: "./locales/core",
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    translations: [TRANSLATIONS],
}}
```

### `languages`

Path to a file containing the list of languages supported by the application.
Must be relative to the _Cargo.toml_ file.

```rust
leptos_fluent! {{
    locales: "./locales",
    languages: "./locales/languages.json",
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    translations: [TRANSLATIONS],
}}
```

```admonish tip
See [**4. Languages**](https://mondeja.github.io/leptos-fluent/languages.html)
```

The languages file should contain an array of arrays where each inner array
contains a language identifier and a language name, respectively, at least.

The language identifier should be a valid language tag, such as `en-US`, `en`,
`es-ES`, etc. By default, the languages file should be a JSON with a `.json`
extension because the `json` feature is enabled. For example:

```json
[
  ["en-US", "English (United States)"],
  ["es-ES", "Español (España)"]
]
```

Set `default-features = false` and enable the `yaml` or the `json5` feature
to use a YAML or JSON5 files. For example:

```yaml
# locales/languages.yaml
- - en-US
  - English (United States)
- - es-ES
  - Español (España)
```

```json5
// locales/languages.json5
[
  ["en-US", "English (United States)"],
  ["es-ES", "Español (España)"],
]
```

Define a third element in the inner array with the direction of the language,
to use it in the [`<html dir="...">` attribute] (see [`sync_html_tag_dir`]).
For example:

```json
[
  ["en-US", "English (United States)", "ltr"],
  ["es-ES", "Español (España)", "auto"],
  ["ar", "العربية", "rtl"],
  ["it", "Italiano"]
]
```

### `check_translations`

Check the translations at compile time. It is useful to ensure that all
translations are correct and that there are no missing translations.

Must be a [glob] relative to the _Cargo.toml_ file.

- For single crate projects:

  ```rust
  leptos_fluent! {{
      locales: "./locales",
      translations: [TRANSLATIONS],
      check_translations: "./src/**/*.rs",
      // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  }}
  ```

- For workspace projects:

  ```rust
  leptos_fluent! {{
      locales: "./locales",
      translations: [TRANSLATIONS],
      check_translations: "../{app,components}/src/**/*.rs",
      // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  }}
  ```

### <span style="opacity:.5">CSR </span> | `sync_html_tag_lang`

Synchronize the global [`<html lang="...">` attribute] with current language
using [`leptos::create_effect`]. Can be a literal boolean or an expression
that will be evaluated at runtime.

```rust
leptos_fluent! {{
    // ...
    sync_html_tag_lang: true,
}}
```

### <span style="opacity:.5">CSR </span> | `sync_html_tag_dir`

Synchronize the global [`<html dir="...">` attribute] with current language
using [`leptos::create_effect`].

Can be a literal boolean or an expression that will be evaluated at runtime.

```rust
leptos_fluent! {{
    // ...
    sync_html_tag_dir: true,
}}
```

For custom languages from a languages file, specify a third element in
the inner array with the direction of the language, which can be `"auto"`,
`"ltr"`, or `"rtl"`. Discovered languages will be defined depending on the
language.

```admonish example
- Arabic (`ar`): `"rtl"`
- English (`en`): `"ltr"`
- Japanese (`ja`): `"auto"`
```

<!-- markdownlint-disable MD013 -->

### <span style="opacity:.5">CSR + SSR </span> | `url_param: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">"lang"</span>

<!-- markdownlint-enable MD013 -->

Name of [URL parameter] used to manage the current language.

```rust
leptos_fluent! {{
    // ...
    url_param: "lang",
}}
```

### <span style="opacity:.5">CSR + SSR </span> | `initial_language_from_url_param`

Set initial language from [URL parameter].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_url_param_to_localstorage`

Get initial language from [URL parameter] and save it to [local storage].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_url_param_to_cookie`

Get initial language from [URL parameter] and save it to a [cookie].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param_to_cookie: true,
}}
```

### <span style="opacity:.5">CSR </span> | `set_language_to_url_param`

Synchronize current language with [URL parameter].

```rust
leptos_fluent! {{
    // ...
    set_language_to_url_param: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <span style="opacity:.5">CSR </span> | `localstorage_key: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">"lang"</span>

<!-- markdownlint-enable MD013 -->

[Local storage] key to manage the current language.

```rust
leptos_fluent! {{
    // ...
    localstorage_key: "lang",
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_localstorage`

Get initial language from [local storage].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_localstorage_to_cookie`

Get initial language from [local storage] and save it to a [cookie].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_localstorage_to_cookie: true,
}}
```

### <span style="opacity:.5">CSR </span> | `set_language_to_localstorage`

Set the current language to [local storage].

```rust
leptos_fluent! {{
    // ...
    set_language_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_navigator`

Get the initial language from [`navigator.languages`].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_navigator: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_navigator_to_localstorage`

Get the initial language from [`navigator.languages`] and save it in the [local storage].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_navigator_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_navigator_to_cookie`

Get the initial language from [`navigator.languages`] and save it in a [cookie].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_navigator_to_cookie: true,
}}
```

### <span style="opacity:.5">SSR </span> | `initial_language_from_accept_language_header`

Get the initial language from the [`Accept-Language`] header.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_accept_language_header: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <span style="opacity:.5">CSR + SSR </span> | `cookie_name: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">"lf-lang"</span>

<!-- markdownlint-enable MD013 -->

Name of the [cookie] that will be used to manage the current language. By default
it is `"lf-lang"`.

```rust
leptos_fluent! {{
    // ...
    cookie_name: "lang",
}}
```

<!-- markdownlint-disable MD013 -->

### <span style="opacity:.5">CSR </span> | `cookie_attrs: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">""</span>

<!-- markdownlint-enable MD013 -->

[Cookie attributes] to set on the language [cookie].

```rust
leptos_fluent! {{
    // ...
    cookie_attrs: "SameSite=Strict; Secure",
}}
```

If value is an expression the [cookie] will not be validated at compile time:

```rust
let attrs = "SameSite=Strict; Secure; MyCustomCookie=value"
leptos_fluent! {{
    // ...
    cookie_attrs: attrs,
}}
```

### <span style="opacity:.5">CSR + SSR </span> | `initial_language_from_cookie`

Get the initial language from the cookie.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_cookie: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_cookie_to_localstorage`

Get the initial language from the cookie and save it in the [local storage].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_cookie_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `set_language_to_cookie`

Set the current language to the cookie.

```rust
leptos_fluent! {{
    // ...
    set_language_to_cookie: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | `initial_language_from_system`

<!-- markdownlint-enable MD013 -->

Get the initial language from the system.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_system: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | `initial_language_from_data_file`

<!-- markdownlint-enable MD013 -->

Get the initial language from a data file.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_data_file: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | `initial_language_from_system_to_data_file`

<!-- markdownlint-enable MD013 -->

Get the initial language from the system and save it in a data file.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_system_to_data_file: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | `set_language_to_data_file`

<!-- markdownlint-enable MD013 -->

Set the current language to a data file.

```rust
leptos_fluent! {{
    // ...
    set_language_to_data_file: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | `data_file_key: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">"leptos-fluent"</span>

<!-- markdownlint-enable MD013 -->

Key to manage the current language in the data file. It should be unique
per application.

```rust
leptos_fluent! {{
    // ...
    data_file_key: "my-app",
}}
```

### `provide_meta_context`

Provide the macro meta information at runtime as a context.
Get it using `I18n::meta`:

```rust
let i18n = leptos_fluent! {{
    // ...
    provide_meta_context: true,
}};

println!("Macro parameters: {:?}", i18n.meta().unwrap());
```

## Unreleased

### `initial_language_from_server_function`

Get the initial language from a [server function].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_server_function: initial_language_server_function,
}}

/// Server function to set the initial language
#[server(InitialLanguage, "/api")]
pub async fn initial_language_server_function(
) -> Result<Option<String>, ServerFnError> {
    // .. replace with your own logic
    Ok(Some("es".to_string()))
}
```

This parameter type is not like the `initial_language_from_*` parameters, it
takes an identifier to the [server function] that will be called to get the
initial language.

The function must return a `Result<Option<String>, ServerFnError>`.

### `set_language_to_server_function`

Set the current language to a [server function].

```rust
leptos_fluent! {{
    // ...
    set_language_to_server_function: set_language_server_function,
}}

/// Server function to update the current language
#[server(SetLanguage, "/api")]
pub async fn set_language_server_function(
    language: String,
) -> Result<(), ServerFnError> {
    // .. replace with your own logic
    Ok(())
}
```

This parameter type is not like the `set_language_to_*` parameters, it
takes an identifier to the [server function] that will be called to update
the current language.

The function must return a `Result<(), ServerFnError>`.

### `initial_language_from_localstorage_to_server_function`

Get the initial language from [local storage] and set it to a
[server function].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_localstorage_to_server_function: set_language_server_function,
}}

#[server(SetLanguage, "/api")]
pub async fn set_language_server_function(
    language: String,
) -> Result<(), ServerFnError> {
    // .. replace with your own logic
    Ok(())
}
```

### `initial_language_from_cookie_to_server_function`

Get the initial language from a [cookie] and set it to a
[server function].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_cookie_to_server_function: set_language_server_function,
}}

#[server(SetLanguage, "/api")]
pub async fn set_language_server_function(
    language: String,
) -> Result<(), ServerFnError> {
    // .. replace with your own logic
    Ok(())
}
```

### `initial_language_from_navigator_to_server_function`

Get the initial language from [`navigator.languages`] and set it to a
[server function].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_navigator_to_server_function: set_language_server_function,
}}

#[server(SetLanguage, "/api")]
pub async fn set_language_server_function(
    language: String,
) -> Result<(), ServerFnError> {
    // .. replace with your own logic
    Ok(())
}
```

### `initial_language_from_url_param_to_server_function`

Get the initial language from a [URL parameter] and set it to a
[server function].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param_to_server_function: set_language_server_function,
}}

#[server(SetLanguage, "/api")]
pub async fn set_language_server_function(
    language: String,
) -> Result<(), ServerFnError> {
    // .. replace with your own logic
    Ok(())
}
```

[`fluent_templates::static_loader!`]: https://docs.rs/fluent-templates/latest/fluent_templates/macro.static_loader.html
[`once_cell:sync::Lazy`]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html
[`<html lang="...">` attribute]: https://developer.mozilla.org/es/docs/Web/HTML/Global_attributes/lang
[`<html dir="...">` attribute]: https://developer.mozilla.org/es/docs/Web/HTML/Global_attributes/dir
[local storage]: https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
[`navigator.languages`]: https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages
[`leptos::create_effect`]: https://docs.rs/leptos/latest/leptos/fn.create_effect.html
[cookie attributes]: https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie#write_a_new_cookie
[`Accept-Language`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Language
[glob]: https://docs.rs/globwalk/latest/globwalk/fn.glob.html
[URL parameter]: https://developer.mozilla.org/es/docs/Web/API/URLSearchParams
[`sync_html_tag_dir`]: #csr---sync_html_tag_dir
[cookie]: https://developer.mozilla.org/docs/Web/HTTP/Cookies
[Server function]: https://book.leptos.dev/server/25_server_functions.html
