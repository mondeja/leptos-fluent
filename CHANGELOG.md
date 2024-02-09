# CHANGELOG

## 2024-02-09 - v0.0.3

### New features

Added some parameters to `leptos_fluent!` macro:

- `initial_language_from_url`
- `initial_language_from_url_param`
- `initial_language_from_url_to_localstorage`
- `initial_language_from_localstorage`
- `initial_language_from_navigator`
- `localstorage_key`

### Breaking changes

Now `provide_context` accepts a `Option<&'static Language>` instead of `&'static Language` as the initial language of the user.

## 2024-02-08 - v0.0.2

- Added `i18n()` function.

## 2024-02-08 - v0.0.1

First alpha release.
