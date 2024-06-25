<!-- markdownlint-disable MD033 -->

# `leptos_fluent!`

The `leptos_fluent!` macro is used to load the translations and set the current
locale. It is used in the root component of the application.

```rust
leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],
}}
```

## Parameters

### `translations`

Set the translations to be used in the application. It must be a reference
to a static array of `fluent_templates::StaticLoader` instances.

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

### `locales`

Set the path to the locales directory which contain the Fluent
files for the translations. Must be relative to the _Cargo.toml_ file.

```rust
leptos_fluent! {{
    locales: "./locales",
    // ^^^^^^^^^^^^^^^^^
    translations: [TRANSLATIONS],
}}
```

### `core_locales`

Common locale resources that are shared across all locales.
Must be relative to the _Cargo.toml_ file. Indicate the same as
in `core_locales` option of `static_loader!`:

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

See [Languages].

### `check_translations`

Check the translations at compile time. It is useful to ensure that all
translations are correct and that there are no missing translations.

Must be a glob relative to the _Cargo.toml_ file. For single crate projects:

```rust
leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],
    check_translations: "./src/**/*.rs",
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}}
```

For workspace projects:

```rust
leptos_fluent! {{
    locales: "./locales",
    translations: [TRANSLATIONS],
    check_translations: "../{app,components}/src/**/*.rs",
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}}
```

### <span style="opacity:.5">CSR </span> | `sync_html_tag_lang`

Synchronize the `lang` attribute of the `<html>` tag with the current locale.
It is useful for SEO purposes.

```rust
leptos_fluent! {{
    // ...
    sync_html_tag_lang: true,
}}
```

### <span style="opacity:.5">CSR </span> | `sync_html_tag_dir`

Synchronize the `dir` attribute of the `<html>` tag with the writing direction
of the current locale.

```rust
leptos_fluent! {{
    // ...
    sync_html_tag_dir: true,
}}
```

### <span style="opacity:.5">CSR + SSR </span> | `url_param`

Set the name of the URL parameter that will be used to manage the current
language. By default it is `"lang"`.

```rust
leptos_fluent! {{
    // ...
    url_param: "lang",
}}
```

### <span style="opacity:.5">CSR + SSR </span> | `initial_language_from_url_param`

Set the initial language from the URL parameter.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_url_param_to_localstorage`

Set the initial language from the URL parameter and save it in the local storage.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_url_param_to_cookie`

Set the initial language from the URL parameter and save it in a cookie.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_url_param_to_cookie: true,
}}
```

### <span style="opacity:.5">CSR </span> | `set_language_to_url_param`

Set the current language to the URL parameter.

```rust
leptos_fluent! {{
    // ...
    set_language_to_url_param: true,
}}
```

### <span style="opacity:.5">CSR </span> | `localstorage_key`

Key to manage the current language in the local storage. By default it is
`"lang"`.

```rust
leptos_fluent! {{
    // ...
    localstorage_key: "lang",
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_localstorage`

Get the initial language from the local storage.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_localstorage_to_cookie`

Get the initial language from the local storage and save it in a cookie.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_localstorage_to_cookie: true,
}}
```

### <span style="opacity:.5">CSR </span> | `set_language_to_localstorage`

Set the current language to the local storage.

```rust
leptos_fluent! {{
    // ...
    set_language_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_navigator`

Get the initial language from the navigator.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_navigator: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_navigator_to_localstorage`

Get the initial language from the navigator and save it in the local storage.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_navigator_to_localstorage: true,
}}
```

### <span style="opacity:.5">CSR </span> | `initial_language_from_navigator_to_cookie`

Get the initial language from the navigator and save it in a cookie.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_navigator_to_cookie: true,
}}
```

### <span style="opacity:.5">SSR </span> | `initial_language_from_accept_language_header`

Get the initial language from the `Accept-Language` header.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_accept_language_header: true,
}}
```

### <span style="opacity:.5">CSR + SSR </span> | `cookie_name`

Name of the cookie that will be used to manage the current language. By default
it is `"lf-lang"`.

```rust
leptos_fluent! {{
    // ...
    cookie_name: "lang",
}}
```

### <span style="opacity:.5">CSR </span> | `cookie_attrs`

[Cookie attributes] to set on the language cookie. By default it is `""` (empty):

```rust
leptos_fluent! {{
    // ...
    cookie_attrs: "SameSite=Strict; Secure",
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

Get the initial language from the cookie and save it in the local storage.

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

### <span style="opacity:.5">Desktop (system) </span> | `initial_language_from_system`

Get the initial language from the system.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_system: true,
}}
```

### <span style="opacity:.5">Desktop (system) </span> | `initial_language_from_data_file`

Get the initial language from a data file.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_data_file: true,
}}
```

### <span style="opacity:.5">Desktop (system) </span> | `initial_language_from_system_to_data_file`

Get the initial language from the system and save it in a data file.

```rust
leptos_fluent! {{
    // ...
    initial_language_from_system_to_data_file: true,
}}
```

### <span style="opacity:.5">Desktop (system) </span> | `set_language_to_data_file`

Set the current language to a data file.

```rust
leptos_fluent! {{
    // ...
    set_language_to_data_file: true,
}}
```

### <span style="opacity:.5">Desktop (system) </span> | `data_file_key`

Key to manage the current language in the data file. It should be unique
per application. By default it is `"leptos-fluent"`.

```rust
leptos_fluent! {{
    // ...
    data_file_key: "my-app",
}}
```

[Languages]: https://mondeja.github.io/leptos-fluent/languages.html
[Cookie attributes]: https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie#write_a_new_cookie
