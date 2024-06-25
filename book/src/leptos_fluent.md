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
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><span style="opacity:.9;color: #de935f;font-size: 11px; right: -2px; top: -12px;position:relative; transform: rotate(-45deg);display:inline-block">feat</span></a><span style="opacity:.5;padding-right: -10px">system</span> | Data files on Desktop applications

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

You can set `default-features = false` and enable the `yaml` or the `json5` feature
to be able to use a YAML or JSON5 file. For example:

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

You can define a third element in the inner array with the direction of the language,
to use it in the [`<html dir="...">` attribute] (see `sync_html_tag_dir`). For example:

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

For custom languages from a languages file, you can specify a third element in
the inner array with the direction of the language, which can be `"auto"`,
`"ltr"`, or `"rtl"`. For automatic languages will be defined depending on the language.

```admonish example
- Arabic (`ar`): `"rtl"`
- English (`en`): `"ltr"`
- Japanese (`ja`): `"auto"`
```

<!-- markdownlint-disable MD013 -->

### <span style="opacity:.5">CSR + SSR </span> | `url_param: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">"lang"</span>

<!-- markdownlint-enable MD013 -->

Set the name of the [URL parameter] that will be used to manage the current
language.

```rust
leptos_fluent! {{
    // ...
    url_param: "lang",
}}
```

### <span style="opacity:.5">CSR + SSR </span> | `initial_language_from_url_param`

Set initial language from the [URL parameter].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_url_param_to_localstorage`

Get initial language from the [URL parameter] and save it to [local storage].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_url_param_to_cookie`

Get the initial language from the [URL parameter] and save it to a cookie.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param_to_cookie: true,
}}
```

### <span style="opacity:.5">CSR </span> | `set_language_to_url_param`

Set current language to the [URL parameter].

```rust
leptos_fluent! {{
    // ...
    set_language_to_url_param: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <span style="opacity:.5">CSR </span> | `localstorage_key: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">"lang"</span>

<!-- markdownlint-enable MD013 -->

Key to manage the current language in [local storage].

```rust
leptos_fluent! {{
    // ...
    localstorage_key: "lang",
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_localstorage`

Get the initial language from [local storage].

```rust
leptos_fluent! {{
    // ...
    initial_language_from_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_localstorage_to_cookie`

Get the initial language from [local storage] and save it to a cookie.

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

Get the initial language from [`navigator.languages`] and save it in a cookie.

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

Name of the cookie that will be used to manage the current language. By default
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

[Cookie attributes] to set on the language cookie.

```rust
leptos_fluent! {{
    // ...
    cookie_attrs: "SameSite=Strict; Secure",
}}
```

If the passed value is an expression the cookie will not be validated at
compile time:

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

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><span style="opacity:.9;color: #de935f;font-size: 11px; right: -2px; top: -12px;position:relative; transform: rotate(-45deg);display:inline-block">feat</span></a><span style="opacity:.5;padding-right: -10px">system</span> | `initial_language_from_system`

<!-- markdownlint-enable MD013 -->

Get the initial language from the system.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_system: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><span style="opacity:.9;color: #de935f;font-size: 11px; right: -2px; top: -12px;position:relative; transform: rotate(-45deg);display:inline-block">feat</span></a><span style="opacity:.5;padding-right: -10px">system</span> | `initial_language_from_data_file`

<!-- markdownlint-enable MD013 -->

Get the initial language from a data file.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_data_file: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><span style="opacity:.9;color: #de935f;font-size: 11px; right: -2px; top: -12px;position:relative; transform: rotate(-45deg);display:inline-block">feat</span></a><span style="opacity:.5;padding-right: -10px">system</span> | `initial_language_from_system_to_data_file`

<!-- markdownlint-enable MD013 -->

Get the initial language from the system and save it in a data file.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_system_to_data_file: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><span style="opacity:.9;color: #de935f;font-size: 11px; right: -2px; top: -12px;position:relative; transform: rotate(-45deg);display:inline-block">feat</span></a><span style="opacity:.5;padding-right: -10px">system</span> | `set_language_to_data_file`

<!-- markdownlint-enable MD013 -->

Set the current language to a data file.

```rust
leptos_fluent! {{
    // ...
    set_language_to_data_file: true,
}}
```

<!-- markdownlint-disable MD013 -->

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><span style="opacity:.9;color: #de935f;font-size: 11px; right: -2px; top: -12px;position:relative; transform: rotate(-45deg);display:inline-block">feat</span></a><span style="opacity:.5;padding-right: -10px">system</span> | `data_file_key: `<span style="color: #b5bd68;font-size: 16px; opacity:.9;">"leptos-fluent"</span>

<!-- markdownlint-enable MD013 -->

Key to manage the current language in the data file. It should be unique
per application.

```rust
leptos_fluent! {{
    // ...
    data_file_key: "my-app",
}}
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
