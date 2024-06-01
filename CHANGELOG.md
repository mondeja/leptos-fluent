# CHANGELOG

## 2024-06-01 - [0.0.28]

### New features

- Add `cookie_name`, `initial_language_from_cookie` and
  `set_language_to_cookie` parameters to `leptos_fluent!` macro.

## 2024-05-31 - [0.0.27]

### New features

- Add `yaml` feature to read languages from a YAML file.
- Add `json` feature to read languages from a JSON file (enabled by default).

### Breaking changes

- The method `I18n.set_language` has been removed.
  Use `i18n.language.set(lang)` instead of `i18n.set_language(lang)`.
- The method `I18n.language_key` has been removed.
  Use `<For ... key=move |lang| *lang ... />` instead of
  `<For ... key=move |lang| i18n.language_key(lang)  ... />`.
- The method `I18n.language_from_str` has been removed.
  Use `Language::from_str` instead of `I18n.language_from_str`.

### Enhancements

- Add `impl Hash` for `Language` struct.
- Add `impl IntoAttribute` for `Language` struct.
  Use `<input ... id=lang />` instead of
  `<input ... id=lang.id.to_string() />`.

## 2024-05-28 - [0.0.26]

### Breaking changes

- Rename `initial_language_from_url` parameter of `leptos_fluent!` macro as
  `initial_language_from_url_param`.
- Rename `initial_language_from_url_param` parameter of `leptos_fluent!` macro
  as `url_param`.
- Rename `initial_language_from_url_to_localstorage` parameter of
  `leptos_fluent!` macro as `initial_language_from_url_param_to_localstorage`.

### New features

- Add `set_language_to_url_param` macro parameter to `leptos_fluent!` macro.

### Enhancements

- Drop `leptos_router` as a dependency.

## 2024-05-27 - [0.0.25]

### Breaking changes

- Rename `set_to_localstorage` parameter of `leptos_fluent!` macro as
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
  and `I18n.set_language` method instead.
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

[0.0.27]: https://github.com/mondeja/leptos-fluent/compare/v0.0.26...v0.0.27
[0.0.26]: https://github.com/mondeja/leptos-fluent/compare/v0.0.25...v0.0.26
[0.0.25]: https://github.com/mondeja/leptos-fluent/compare/v0.0.24...v0.0.25
[0.0.24]: https://github.com/mondeja/leptos-fluent/compare/v0.0.23...v0.0.24
[0.0.23]: https://github.com/mondeja/leptos-fluent/compare/v0.0.22...v0.0.23
[0.0.22]: https://github.com/mondeja/leptos-fluent/compare/v0.0.21...v0.0.22
[0.0.21]: https://github.com/mondeja/leptos-fluent/compare/v0.0.20...v0.0.21
[0.0.20]: https://github.com/mondeja/leptos-fluent/compare/v0.0.17...v0.0.20
[0.0.17]: https://github.com/mondeja/leptos-fluent/compare/v0.0.15...v0.0.17
[0.0.15]: https://github.com/mondeja/leptos-fluent/compare/v0.0.1...v0.0.15
