<!-- markdownlint-disable MD013 MD033 MD055 MD056 -->

# Strategies

All the features of the framework are optional, following a declarative
"opt-in" configuration method.

<!-- toc -->

## Loading the initial language of the user

The initial language of the user can be set in different ways:

| Strategy                        | CSR | SSR | Desktop | [`leptos_fluent!`]                             |
| :------------------------------ | :-: | :-: | :-----: | :--------------------------------------------- |
| [URL parameter]                 | ✅  | ✅  |   ❌    | `initial_language_from_url_param`              |
| [URL path]                      | ✅  | ✅  |   ❌    | `initial_language_from_url_path`               |
| [Cookie]                        | ✅  | ✅  |   ❌    | `initial_language_from_cookie`                 |
| [Server function]               | ✅  | ✅  |   ❌    | `initial_language_from_server_function`        |
| Browser [local storage]         | ✅  | ❌  |   ❌    | `initial_language_from_localstorage`           |
| Browser [session storage]       | ✅  | ❌  |   ❌    | `initial_language_from_sessionstorage`         |
| Browser [`navigator.languages`] | ✅  | ❌  |   ❌    | `initial_language_from_navigator`              |
| [`Accept-Language`] header      | ❌  | ✅  |   ❌    | `initial_language_from_accept_language_header` |
| [System language]               | ❌  | ❌  |   ✅    | `initial_language_from_system`                 |
| Data file                       | ❌  | ❌  |   ✅    | `initial_language_from_data_file`              |

All of them can be defined at the same time or just one of them at the same
time. The first language source found will be used and nexts discharged.
The order of precedence is:

- **SSR**
  1. [Server function]
  1. [URL parameter]
  1. [URL path]
  1. [Cookie]
  1. [`Accept-Language`] header
- **CSR**
  1. [Server function]
  1. [URL parameter]
  1. [URL path]
  1. [Cookie]
  1. Browser [local storage]
  1. Browser [session storage]
  1. Browser [`navigator.languages`]
- **Desktop** ([`system` feature][desktop-applications])
  1. Data file
  1. [System language]

## <span style="opacity:.5">CSR </span> | Updating the language on the client

When the user changes the language and `I18n::language.set` is called, the
framework can perform a side effect to update the language in the client. The
following strategies are available:

| Strategy                  | [`leptos_fluent!`]                |
| :------------------------ | :-------------------------------- |
| [URL parameter]           | `set_language_to_url_param`       |
| [Cookie]                  | `set_language_to_cookie`          |
| Browser [local storage]   | `set_language_to_localstorage`    |
| Browser [session storage] | `set_language_to_sessionstorage`  |
| [Server function]         | `set_language_to_server_function` |

When the user changes the language in the browser settings, the framework can
perform a side effect to reflect the language change in the client.
The following strategies are available:

| Strategy                        | [`leptos_fluent!`]            |
| :------------------------------ | :---------------------------- |
| Browser [`navigator.languages`] | `set_language_from_navigator` |

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | Desktop applications

| Strategy  | [`leptos_fluent!`]          |
| :-------- | :-------------------------- |
| Data file | `set_language_to_data_file` |

## <span style="opacity:.5">CSR </span> | Updating the language from initialization on the client

When a language is loaded from initialization, the framework can perform a side
effect to persistently storage the language in the client. The following strategies
are available:

| Strategy                                     | [`leptos_fluent!`]                                        |
| :------------------------------------------- | :-------------------------------------------------------- |
| [URL parameter] to [cookie]                  | `initial_language_from_url_param_to_cookie`               |
| [URL parameter] to [local storage]           | `initial_language_from_url_param_to_localstorage`         |
| [URL path] to [cookie]                       | `initial_language_from_url_path_to_cookie`                |
| [URL path] to [local storage]                | `initial_language_from_url_path_to_localstorage`          |
| [Cookie] to [local storage]                  | `initial_language_from_cookie_to_localstorage`            |
| [Local storage] to [cookie]                  | `initial_language_from_localstorage_to_cookie`            |
| [Local storage] to [server function]         | `initial_language_from_localstorage_to_server_function`   |
| [Session storage] to [cookie]                | `initial_language_from_sessionstorage_to_cookie`          |
| [Session storage] to [local storage]         | `initial_language_from_sessionstorage_to_localstorage`    |
| [Session storage] to [server function]       | `initial_language_from_sessionstorage_to_server_function` |
| [`navigator.languages`] to [cookie]          | `initial_language_from_navigator_to_cookie`               |
| [`navigator.languages`] to [local storage]   | `initial_language_from_navigator_to_localstorage`         |
| [`navigator.languages`] to [server function] | `initial_language_from_navigator_to_server_function`      |
| [Server function] to [local storage]         | `initial_language_from_server_function_to_localstorage`   |

### <span style="opacity:.5">CSR + SSR</span>

| Strategy                             | [`leptos_fluent!`]                                   |
| :----------------------------------- | :--------------------------------------------------- |
| [URL parameter] to [server function] | `initial_language_from_url_param_to_server_function` |
| [URL path] to [server function]      | `initial_language_from_url_path_to_server_function`  |
| [Cookie] to [server function]        | `initial_language_from_cookie_to_server_function`    |
| [Server function] to [cookie]        | `initial_language_from_server_function_to_cookie`    |

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | Desktop applications

| Strategy                       | [`leptos_fluent!`]                          |
| :----------------------------- | :------------------------------------------ |
| [System language] to data file | `initial_language_from_system_to_data_file` |

## <span style="opacity:.5">CSR </span> | Client side effects

When the user updates the language, the framework can perform side effects to
update the language in the client. The following side effects are available:

| Side effect                     | [`leptos_fluent!`]   |
| :------------------------------ | :------------------- |
| [`<html lang="...">`] attribute | `sync_html_tag_lang` |
| [`<html dir="...">`] attribute  | `sync_html_tag_dir`  |

[`<html lang="...">`]: https://developer.mozilla.org/docs/Web/HTML/Global_attributes/lang
[`<html dir="...">`]: https://developer.mozilla.org/docs/Web/HTML/Global_attributes/dir

## <span style="opacity:.5">CSR + SSR </span> | Names

The names of the settings can be configured using the following parameters:

| Strategy                  | [`leptos_fluent!`]   | Default value |
| :------------------------ | :------------------- | :-----------: |
| [Cookie]                  | `cookie_name`        |  `"lf-lang"`  |
| [Cookie attributes]       | `cookie_attrs`       |     `""`      |
| Browser [local storage]   | `localstorage_key`   |   `"lang"`    |
| Browser [session storage] | `sessionstorage_key` |   `"lang"`    |
| [URL parameter]           | `url_param`          |   `"lang"`    |
| [URL path] extractor fn   | `url_path`           |      ❌       |

### <a href="https://mondeja.github.io/leptos-fluent/install.html#desktop-applications"><img src="feat.png" width="23px" style="position:relative; bottom: 5px; left: 2px" alt="feat"></img></a><span style="opacity:.5;padding-right: -10px">system</span> | Desktop applications

| Strategy  | [`leptos_fluent!`] | Default value     |
| :-------- | :----------------- | :---------------- |
| Data file | `data_file_key`    | `"leptos-fluent"` |

[`leptos_fluent!`]: https://mondeja.github.io/leptos-fluent/leptos_fluent.html
[local storage]: https://developer.mozilla.org/docs/Web/API/Window/localStorage
[session storage]: https://developer.mozilla.org/docs/Web/API/Window/sessionStorage
[`navigator.languages`]: https://developer.mozilla.org/docs/Web/API/Navigator/languages
[`Accept-Language`]: https://developer.mozilla.org/docs/Web/HTTP/Headers/Accept-Language
[Cookie]: https://developer.mozilla.org/docs/Web/API/Document/cookie
[Cookie attributes]: https://developer.mozilla.org/docs/Web/API/Document/cookie#write_a_new_cookie
[URL parameter]: https://developer.mozilla.org/docs/Web/API/URLSearchParams
[URL path]: https://developer.mozilla.org/docs/Web/API/URL/pathname
[desktop-applications]: https://mondeja.github.io/leptos-fluent/install.html#desktop-applications
[System language]: https://github.com/i509VCB/current_locale
[Server function]: https://book.leptos.dev/server/25_server_functions.html
