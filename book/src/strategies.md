<!-- markdownlint-disable MD013 -->

# Features

All the features of the framework are optional, following a declarative
"opt-in" configuration method.

## Loading the initial language of the user

The initial language of the user can be set in different ways:

| Strategy                                   | CSR | SSR | [`leptos_fluent!`] parameter                   |
| :----------------------------------------- | :-: | :-: | :--------------------------------------------- |
| [URL query parameter]                      | ✅  | ✅  | `initial_language_from_url_param`              |
| [Cookie]                                   | ✅  | ✅  | `initial_language_from_cookie`                 |
| Browser [local storage]                    | ✅  | ❌  | `initial_language_from_localstorage`           |
| Browser language ([`navigator.languages`]) | ✅  | ❌  | `initial_language_from_navigator`              |
| [`Accept-Language`] header                 | ❌  | ✅  | `initial_language_from_accept_language_header` |

All of them can be used at the same time or just one of them. The first setting
found will be used. The order of precedence is:

- **SSR**
  1. [URL query parameter].
  2. [Cookie].
  3. [`Accept-Language`] header.
- **CSR**
  1. [URL query parameter].
  2. [Cookie].
  3. Browser [local storage].
  4. Browser language from [`navigator.languages`].

## Updating the language on the client

When the user changes the language and `I18n::language.set` is called, the
framework can perform a side effect to update the language in the client. The
following strategies are available:

| Strategy                | [`leptos_fluent!`] parameter   |
| :---------------------- | :----------------------------- |
| [URL query parameter]   | `set_language_to_url_param`    |
| [Cookie]                | `set_language_to_cookie`       |
| Browser [local storage] | `set_language_to_localstorage` |

### Updating the language from initialization on the client

When a language is loaded from initialization, the framework can perform a side
effect to persistently storage the language in the client. The following strategies
are available:

| Strategy                                 | [`leptos_fluent!`] parameter                      |
| :--------------------------------------- | :------------------------------------------------ |
| [URL query parameter] to [local storage] | `initial_language_from_url_param_to_localstorage` |

## Client side effects

When the user updates the language, the framework can perform side effects to
update the language in the client. The following side effects are available:

| Side effect                     | [`leptos_fluent!`] parameter |
| :------------------------------ | :--------------------------- |
| [`<html lang="...">`] attribute | `sync_html_tag_lang`         |
| [`<html dir="...">`] attribute  | `sync_html_tag_dir`          |

[`<html lang="...">`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang
[`<html dir="...">`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/dir

## Configuring names

The names of the settings can be configured using the following parameters:

| Strategy                | [`leptos_fluent!`] parameter | Default value |
| :---------------------- | :--------------------------- | :------------ |
| [Cookie]                | `cookie_name`                | `"lf-lang"`   |
| [Cookie attributes]     | `cookie_attrs`               | `""`          |
| Browser [local storage] | `localstorage_key`           | `"lang"`      |
| [URL query parameter]   | `url_param`                  | `"lang"`      |

[`leptos_fluent!`]: https://docs.rs/leptos-fluent-macros/latest/leptos_fluent_macros/macro.leptos_fluent.html
[local storage]: https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
[`navigator.languages`]: https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages
[`Accept-Language`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Language
[Cookie]: https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie
[Cookie attributes]: https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie#write_a_new_cookie
[URL query parameter]: https://developer.mozilla.org/es/docs/Web/API/URLSearchParams
