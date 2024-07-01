# CHANGELOG

## 2024-07-01 - [0.1.8]

### New features

- Add `tracing` feature to enable building with `tracing` support.

### Enhancements

- Stop depending on `wasm-bindgen` from `leptos-fluent` crate.

## 2024-06-29 - [0.1.7]

### New features

- Add next parameters to `leptos_fluent!` macro:
  - `initial_language_from_server_function` to set the initial language from a
    server function.
  - `set_language_to_server_function` set language updates to a server
    function.
  - `initial_language_from_localstorage_to_server_function` to set the initial
    language from local storage to a server function.
  - `initial_language_from_cookie_to_server_function` to set the initial language
    from a cookie to a server function.
  - `initial_language_from_navigator_to_server_function` to set the initial
    language from the browser language to a server function.
  - `initial_language_from_url_param_to_server_function` to set the initial
    language from URL parameter to a server function.
  - `initial_language_from_server_function_to_cookie` to set the initial language
    from a server function to a cookie.
  - `initial_language_from_server_function_to_localstorage` to set the initial
    language from a server function to local storage.

### Enhancements

- Accept `#[cfg(debug_assertions)]` and `#[cfg(not(debug_assertions))]`
  conditional checks for the next `leptos_fluent!` macro parameters:
  - `check_translations`
  - `languages`
  - `core_locales`

## 2024-06-27 - [0.1.6]

### New features

- Add `provide_meta_context` parameter to `leptos_fluent!` to provide meta
  information about the macro caller arguments. Use, for example,
  `leptos_fluent::i18n().meta().unwrap().cookie_name` to get the value
  `cookie_name` used as parameter for the macro.
- Add `flag` field to `leptos_fluent::Language` storing emoji flag
  automatic discovered for each language identifier with a country code.

### Bug fixes

- Accept [configuration conditional checks] directly in most macro parameters:

  ```rust
  leptos_fluent! {{
      // ...
      #[cfg(debug_assertions)]
      initial_language_from_url_param: true,
      #[cfg(debug_assertions)]
      set_language_to_url_param: true,
  }}
  ```

[configuration conditional checks]: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html

## 2024-06-26 - [0.1.5]

### New features

- Add `leptos_fluent::SsrHtmlTag` component to render it on SSR to sync
  global attributes of `<html>` tag with the current language.
- Add new feature `system` to enable functionalities that require system
  information. Useful on non wasm targets like desktop applications.
  See [GTK example].
- Add `initial_language_from_system` parameter to `leptos_fluent!` macro to set
  the initial language from the system language. Useful for desktop
  applications. Must be enabled the new feature `system` to use it.
- Add `initial_language_from_data_file` parameter to `leptos_fluent!` macro to
  set the initial language from a data file when using `system` feature.
- Add `set_language_to_data_file` parameter to `leptos_fluent!` macro to set
  the current language to a data file when using `system` feature.
- Add `data_file_key` parameter to `leptos_fluent!` macro to specify the file
  key to store the current language when using `system` feature.
- Add `initial_language_from_system_to_data_file` parameter to `leptos_fluent!`
  macro to set the initial language from the system language to a data file
  when using `system` feature.

[GTK example]: https://github.com/mondeja/leptos-fluent/tree/master/examples/system-gtk

### Enhancements

- Use files tracker API instead of `include_bytes!` quirk to track files
  when `nightly` feature is enabled.

## 2024-06-25 - [0.1.4]

### New features

- Add `initial_language_from_navigator_to_localstorage` parameter to
  `leptos_fluent!` macro to set the initial language from the browser language
  to local storage.
- Add `initial_language_from_navigator_to_cookie` parameter to `leptos_fluent!`
  macro to set the initial language from the browser language to a cookie.

## 2024-06-24 - [0.1.3]

### New features

- Add `initial_language_from_localstorage_to_cookie` parameter to
  `leptos_fluent!` macro to set the initial language from local storage to
  a cookie.
- Allow to call `leptos_fluent::I18n` context to get and set the current active
  language with `i18n()` to get and `i18n(lang)` to set when the feature
  `nightly` is enabled.

## 2024-06-22 - [0.1.2]

### New features

- Add `initial_language_from_url_param_to_cookie` parameter to `leptos_fluent!`
  macro to set the initial language from URL parameter to a cookie.
- Add `initial_language_from_cookie_to_localstorage` parameter to
  `leptos_fluent!` macro to set the initial language from a cookie to
  local storage.
- Add `nightly` feature to enable functionalities that require a nightly
  toolchain.

### Enhancements

- Validate cookie attributes in `leptos_fluent!` when defined using string
  literals.
- Show locations of `tr!` macros with translations checker when using the
  `nightly` feature.

## 2024-06-21 - [0.1.1]

### Enhancements

- Add method `activate` to `leptos_fluent::Language` struct to set a language
  active. Use `lang.activate()` instead of `expect_i18n().language.set(lang)`.

## 2024-06-20 - [0.1.0]

### Breaking changes

- The parameter `translations` of the `leptos_fluent!` macro must now be
  an array of translations. Replace `translations: TRANSLATIONS` by
  `translations: [TRANSLATIONS]`.

### New features

- Add `sync_html_tag_dir` parameter to `leptos_fluent!` macro to sync the `dir`
  global attribute of the `<html>` tag with the current language direction.
- Multiple translations can be passed to the `leptos_fluent!` macro.

## Versioning

**leptos-fluent** will include breaking changes in minor versions during
the v0.x.0 series until v1.0.0 version is reached. Is safe to pin the
version to `0.1` during installation.

## 2024-06-16 - [0.0.37]

### Bug fixes

- Don't panic parsing raw string literals of `tr!` macros checking
  translations.

### Enhancements

- Notify invalid Fluent message identifiers checking translations.

### New features

- Add `cookie_attrs` parameter to `leptos_fluent!` macro to set cookie
  attributes.

## 2024-06-15 - [0.0.36]

### Bug fixes

- Fix error building files tracker when multiple files for each language.

## 2024-06-15 - [0.0.35]

### Bug fixes

- Get the initial language from URL parameter on server side rendering.

## 2024-06-09 - [0.0.34]

### New features

- Allow to read languages from a JSON5 file with a new feature `json5`.
- Add `core_locales` argument to `leptos_fluent!` macro to specify the
  file where the core locales are located.

### Enhancements

- Improved error messages when reading languages from files.

## 2024-06-04 - [0.0.33]

### Bug fixes

- Fluent syntax errors found checking translations are now reported
  instead of panicking.

## 2024-06-03 - [0.0.32]

### Breaking changes

- `locales` argument of `leptos_fluent!` macro is now required.

### Bug fixes

- Fix a lot of bugs checking translations.

### Enhancements

- Glob passed to `check_translations` argument of `leptos_fluent!` macro
  accepts brace expansion.
- Locale and language files are now tracked.

## 2024-06-03 - [0.0.31]

### Bug fixes

- Fix error setting cookies.

## 2024-06-03 - [0.0.30]

### Bug fixes

- Fix trait not in scope error when using `tr!` and `move_tr!` macros.

## 2024-06-03 - [0.0.29]

### Breaking changes

- Replace `I18n.is_active_language(lang)` method by `Language.is_active()`.
  Use `<input ... checked=lang.is_active() ... />` instead of
  `<input ... checked=i18n.is_active_language(lang) ... />`.
- Removed `I18n.default_language()` method. Use `i18n.languages[0]`.
- Removed `I18n.tr()` and `I18n.trs()` methods. Use the `tr!` macro.
- `tr!` and `move_tr!` macros only accepts literal strings as the message name
  (first argument) and in the keys of translation arguments.

### New features

- Add `check_translations` argument to `leptos_fluent!` macro to check
  translations at compile time.

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

[0.1.8]: https://github.com/mondeja/leptos-fluent/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/mondeja/leptos-fluent/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/mondeja/leptos-fluent/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/mondeja/leptos-fluent/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/mondeja/leptos-fluent/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/mondeja/leptos-fluent/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/mondeja/leptos-fluent/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/mondeja/leptos-fluent/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mondeja/leptos-fluent/compare/v0.0.37...v0.1.0
[0.0.37]: https://github.com/mondeja/leptos-fluent/compare/v0.0.36...v0.0.37
[0.0.36]: https://github.com/mondeja/leptos-fluent/compare/v0.0.35...v0.0.36
[0.0.35]: https://github.com/mondeja/leptos-fluent/compare/v0.0.34...v0.0.35
[0.0.34]: https://github.com/mondeja/leptos-fluent/compare/v0.0.33...v0.0.34
[0.0.33]: https://github.com/mondeja/leptos-fluent/compare/v0.0.32...v0.0.33
[0.0.32]: https://github.com/mondeja/leptos-fluent/compare/v0.0.31...v0.0.32
[0.0.31]: https://github.com/mondeja/leptos-fluent/compare/v0.0.30...v0.0.31
[0.0.30]: https://github.com/mondeja/leptos-fluent/compare/v0.0.29...v0.0.30
[0.0.29]: https://github.com/mondeja/leptos-fluent/compare/v0.0.28...v0.0.29
[0.0.28]: https://github.com/mondeja/leptos-fluent/compare/v0.0.27...v0.0.28
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
