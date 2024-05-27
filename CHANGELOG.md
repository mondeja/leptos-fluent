# CHANGELOG

## 2024-05-27 - [0.0.25]

### Breaking changes

- Rename `set_to_localstorage` macro parameter as
  `set_language_to_localstorage`.

## 2024-05-20 - [0.0.24]

### Enhancements

- Add `I18n.is_active_language` method.
- Add `I18n.language_key` method to return a hash for the current
  language with their active status for usage in `For` components.
- Add `set_to_localstorage` parameter to `leptos_fluent!` macro.
- Add `use_i18n` and `expect_i18n` function.

### Breaking changes

- Replace `I18n.set_language_with_localstorage` method with
  `I18n.set_language`. Use `set_to_localstorage` macro parameter
  and `I18n.set_language` instead.
- Remove `csr` feature.

### Bug fixes

- Fix errors getting initial language from URL parameter and local storage.

## 2024-05-18 - [0.0.23]

- Add `axum` feature to integrate with Axum web framework.

## 2024-05-01 - [0.0.22]

- Fix error about required translation tree files in main example of documentation.

## 2024-03-23 - [0.0.21]

- Minimum Leptos version set to `0.6`.
- Fix hydration mode.
- Allow to get the initial language on SSR from the `Accept-Language` header.
- Added `actix` feature to integrate with Actix Web framework.

## 2024-03-10 - [0.0.20]

### Breaking changes

- One of the features `csr`, `hydrate` or `ssr` must be enabled.

### New features

- Add SSR support.
- Add hydration support.
- Added features `csr`, `hydrate` and `ssr`.

### Enhancements

- `I18n` context is now `Copy`able.
- Crates now offers READMEs.

## 2024-03-07 - [0.0.17]

- Don't require to install `unic_langid`.
- Needs `fluent-templates` >= v0.9.0.
- Constricted ranges of dependencies.
- Fixed errors in documentation.
- Added `kab` and `cpp` dialects.

## 2024-03-02 - [0.0.15]

- Added all ISO-639-1 and ISO-639-2 languages.

[0.0.25]: https://github.com/mondeja/leptos-fluent/compare/v0.0.24...v0.0.25
[0.0.24]: https://github.com/mondeja/leptos-fluent/compare/v0.0.23...v0.0.24
[0.0.23]: https://github.com/mondeja/leptos-fluent/compare/v0.0.22...v0.0.23
[0.0.22]: https://github.com/mondeja/leptos-fluent/compare/v0.0.21...v0.0.22
[0.0.21]: https://github.com/mondeja/leptos-fluent/compare/v0.0.20...v0.0.21
[0.0.20]: https://github.com/mondeja/leptos-fluent/compare/v0.0.17...v0.0.20
[0.0.17]: https://github.com/mondeja/leptos-fluent/compare/v0.0.15...v0.0.17
[0.0.15]: https://github.com/mondeja/leptos-fluent/compare/v0.0.1...v0.0.15
