# CHANGELOG

## Unreleased - [0.2.13]

### Enhancements

- Allow to pass a single file path to `check_translations` parameter of
  `leptos_fluent!` macro.
- Don't report syntax errors in translations checker. The Rust compiler should
  catch them instead.

## 2025-06-03 - [0.2.12]

### Enhancements

- The argument `translations` of `leptos_fluent!` macro is now optional. This
  means that `fluent-templates` is not required as a direct dependency anymore.
- Add `disable-unicode-isolating-marks` feature to disable Unicode isolating
  marks in translations when not using `translations` argument in
  `leptos_fluent!` macro.
- Performance improvements for translations checker.

### Bug fixes

- Fix typo in I18n context missing error message.

## 2025-05-29 - [0.2.11]

### New features

- Add `default_language` parameter to `leptos_fluent!` macro to set the initial
  language if the user does not load any with the provided configuration.

## 2025-03-30 - [0.2.10]

### Enhancements

- Allow to be installed with future Leptos v0.8 as is compatible
  with leptos-fluent v0.2 releases.

### Bug fixes

- Fix bug extracting language cookie in SSR mode for Axum integration when
  there are multiple cookies.

## 2025-03-13 - [0.2.9]

### Enhancements

- Allow to pass attributes to `tr!` macro parameters. For example:

  ```rust
  #![feature(stmt_expr_attributes)]

  _ = tr!(
      i18n,
      #[allow(unused_parens)]
      if (my_signal.get() && my_function()) {"foo"} else {"bar"},
      #[allow(unused_braces)]
      {
          "arg1" => {"value1"},
          "arg2" => "value2",
      }
  );
  ```

  Requires the nightly feature `stmt_expr_attributes`.

- Allow to pass `#[cfg(feature = ...]` only for `leptos-fluent` features to
  compile-time parameters of `leptos_fluent!` macro like `check_translations`.

  ```rust
  leptos_fluent! {
      // ...
      #[cfg(feature = "debug")]
      check_translations: "./src/**/*.rs",
  }
  ```

## 2025-03-11 - [0.2.8]

### New features

Allow to pass dynamic values to `tr!` and `move_tr!` macros ids following certain
patterns keeping translations checking. The following patterns are allowed:

```rust
use leptos_fluent::{tr, move_tr}

let (foo, bar) = (false, true);

_ = tr!(if foo { "foo" } else { "bar" });
_ = move_tr!(if foo { "foo" } else if bar { "bar" } else { "baz" });
```

See the [Advanced usage] section of the book for more information.

[Advanced usage]: https://mondeja.github.io/leptos-fluent/advanced-usage.html

### Bug fixes

- Forbid to pass an expression as `$i18n` parameter of `tr!` macros, it must
  be an identifier. The rationale is to avoid using `tr!(expect_i18n(), ...)`
  because that doesn't prevents to get out from the reactive graph anyway.

## 2025-03-10 - [0.2.7]

### New features

- Forbid a few expressions in files inside `check_translations` parameter glob.
  These break translations checker's requirement of one `tr!` macros source of
  thruth. The forbidden expressions are:
  - `use tr;`
  - `use move_tr;`
  - `use whatever::tr;` (except `leptos-fluent`)
  - `use whatever::move_tr;` (except `leptos-fluent`)
  - `use leptos_fluent::tr as whatever;`
  - `use leptos_fluent::move_tr as whatever;`
  - `use leptos_fluent as whatever;`
- Add next new parameters to `leptos_fluent!` macro related to browser's
  session storage:
  - `initial_language_from_url_param_to_sessionstorage` to set the initial
    language from URL parameter to session storage.
  - `initial_language_from_cookie_to_sessionstorage` to set the initial language
    from a cookie to session storage.
  - `initial_language_from_url_path_to_sessionstorage` to set the initial
    language from URL path to session storage.

### Enhancements

- Allow to pass expressions as `leptos_fluent!` macro parameters related to server
  functions.
- Add `i18n.tr` and `i18n.tr_with_args` methods.
- Deprecate `tr_impl(i18n, id)` and `tr_with_args_impl(i18n, id, args)` functions.
  Use `i18n.tr(id)` and `i18n.tr_with_args(id, args)` methods instead.

### Bug fixes

- Fix bugs raising some errors when parsing `leptos_fluent!` macro parameters.
- Fix error shown when passing dynamic values as `tr!` and `move_tr!` macros ids.

## 2025-03-07 - [0.2.6]

### New features

- Add a new parameter `fill_translations` to `leptos_fluent!` macro to add
  messages found in `tr!` and `move_tr!` macros to Fluent translations files.
- Add next new parameters to `leptos_fluent!` macro to use browser's session
  storage:
  - `sessionstorage_key` to set the key to store the current language.
  - `initial_language_from_sessionstorage` to set the initial language from
    session storage.
  - `set_language_to_sessionstorage` to set the current language to session
    storage.
  - `initial_language_from_sessionstorage_to_cookie` to set the initial
    language from session storage to a cookie.
  - `initial_language_from_sessionstorage_to_localstorage` to set the initial
    language from session storage to local storage.
  - `initial_language_from_sessionstorage_to_server_function` to set the
    initial language from session storage to a server function.
  - `initial_language_from_localstorage_to_sessionstorage` to set the initial
    language from local storage to session storage.
  - `initial_language_from_navigator_to_sessionstorage` to set the initial
    language from the browser `navigator.languages` to session storage.

## 2025-03-06 - [0.2.5]

### Bug fixes

- Fix strange error messages from translations checker when trying to pass a
  Rust literal that is not a string to `tr!` and `move_tr!` macros.
- Fix empty placeables detected by translations checker when passing invalid
  argument types to `tr!` and `move_tr!` macros.
- Fix Fluent message references not recognized by translations checker.

## 2025-03-02 - [0.2.4]

### Enhancements

- Update Russian language name. From `Русский язык` ("Russian language") to
  `Русский` ("Russian").
- Document `tr_impl` and `tr_with_args_impl` functions. Recommend their usage for
  dynamic translations.

## 2025-01-23 - [0.2.3]

### Bug fixes

- Fixed type in hashmap key propagating arguments to `fluent-templates`
  with `tr!(i18n, id, args)` and `move_tr!(i18n, id, args)` macros.

### Enhancements

- Add MSRVs to `Cargo.toml`s.

## 2025-01-13 - [0.2.2]

- Use `std::sync::LazyLock` instead of `once_cell::sync::Lazy`.
  Fixes compatibility with fluent-templates v0.13 (requires fluent-templates
  v0.13.0 or higher).

## 2024-12-23 - [0.2.1]

- Fixes compatibility with fluent-templates v0.12 (requires fluent-templates
  v0.12.0 or higher).

## 2024-12-17 - [0.2.0]

### Breaking changes

#### Require Leptos v0.7

Support for Leptos v0.6 has been dropped. Now leptos-fluent requires
Leptos v0.7.

#### Declarative API

Previously, the `leptos_fluent!` macro was providing the I18n context
using `provide_context`, so the app was configured as:

```rust
#[component]
fn App() -> impl IntoView {
    leptos_fluent! {
        // ...
    }

    view! {
        // ...
    }
}
```

Now a `children` option has been added to the macro and need to be used
to declare a I18n component:

```rust
#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        // ...
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <I18n>
            // ...
        </I18n>
    }
}
```

By this way `leptos_fluent!` does not return the I18n context anymore.

#### No more double curly braces

Support for deprecated double curly braces syntax for the `leptos_fluent!`
macro has been removed. Use single curly braces instead:

```rust
leptos_fluent! {
    // ...
}
```

#### No more default features

The feature `json` is not enabled by default anymore. Now leptos-fluent
does not includes features by default, so you don't need to use
`default-features = false` in your `Cargo.toml` file.

#### `SsrHtmlTag` component removed

The deprecated `SsrHtmlTag` component has been removed.

## 2024-11-15 - [0.1.26]

### Enhancements

- Let rustc infer literal expressions booleans in `leptos_fluent!` macro
  parameters, which optimizes compilation times.

## 2024-10-26 - [0.1.25]

### Bug fixes

- Fix unhygienic use of `leptos::window` in `leptos_fluent!` macro expansion.

## 2024-10-11 - [0.1.24]

### Enhancements

- Add `translations` field to `I18n`'s `Debug` implementation.
- Allow to pass empty translations array to `leptos_fluent!` with
  `translations: []`.

## 2024-09-25 - [0.1.23]

### Enhancements

- Allow single braces syntax for `leptos_fluent!` macro. The current
  syntax `leptos_fluent! {{ ... }}` is still supported but now triggers
  a deprecation warning. It will not be supported from `v0.2`.
  Use `leptos_fluent! { ... }` instead.

## 2024-09-24 - [0.1.22]

### Bug fixes

- Fix translations checker not detecting translation macros inside macro
  calls (regression from v0.1.21).

## 2024-08-27 - [0.1.21]

### Bug fixes

- Fix translation macros not extracted from some locations when a variable
  binding is not used in translations checker.

## 2024-08-26 - [0.1.20]

### Bug fixes

- Fix variables in Fluent selectors not being extracted as placeables when
  checking translations.

## 2024-08-19 - [0.1.19]

### Bug fixes

- Allow to pass `i18n` as first argument to `tr!` and `move_tr!` macros.
  This is an alternative to panicking when using the macros in event handlers.

## 2024-08-18 - [0.1.18]

### Bug fixes

- Relax `fluent-templates` dependency.

## 2024-08-17 - [0.1.17]

### New features

- Add `set_language_from_navigator` parameter to `leptos_fluent!` macro to set
  the language at runtime from the browser language.

## 2024-08-16 - [0.1.16]

### Enhancements

- Add `debug` feature to print debug information when using `leptos_fluent!`
  macro.
- Some performance improvements.

## 2024-08-15 - [0.1.15]

### Bug fixes

- Fix syntax error produced at `leptos_fluent!` macro expansion when using
  `sync_html_tag_dir` but not `sync_html_tag_lang` or vice versa on SSR
  (regression from v0.1.14).

## 2024-08-14 - [0.1.14]

### Enhancements

- Deprecate `SsrHtmlTag` component. The `sync_html_tag_lang` and
  `sync_html_tag_dir` parameters of `leptos_fluent!` macro are enough to sync
  the `<html>` tag attributes with the current language and direction on SSR.
  Will be removed on v0.2.

## 2024-08-11 - [0.1.13]

### Bug fixes

- Fix bug discovering initial language from `navigator.languages`
  (regression from v0.1.11).

## 2024-08-05 - [0.1.12]

### New features

- Add `initial_language_from_url_path` parameter to `leptos_fluent!` macro to
  set the initial language from URL path.
- Add `initial_language_from_url_path_to_cookie` to `leptos_fluent!` macro to
  set the initial language from URL path to a cookie.
- Add `initial_language_from_url_path_to_localstorage` to `leptos_fluent!` macro
  to set the initial language from URL path to local storage.
- Add `initial_language_from_url_path_to_server_function` to `leptos_fluent!`
  macro to set the initial language from URL path to a server function.

## 2024-08-04 - [0.1.11]

### Bug fixes

- Fix translations checker not extracting Fluent function positional arguments
  as placeables.

### Enhancements

- Accept multiple configuration conditional checks for the same parameters in
  `leptos_fluent!` macro:

  ```rust
  leptos_fluent! {
      // ...
      #[cfg(debug_assertions)]
      set_language_to_url_param: true,
      #[cfg(not(debug_assertions))]
      set_language_to_url_param: false,
  }
  ```

## 2024-08-03 - [0.1.10]

### Enhancements

- Accept almost all possible constant compile-time config expression paths
  for some `leptos_fluent!` parameters like `languages`.

  For example, only use a languages file when compiling on Unix systems:

  ```rust
  leptos_fluent! {
      // ...
      #[cfg(target_family = "unix")]
      languages: "./locales/languages.json",
  }
  ```

## 2024-08-02 - [0.1.9]

### Enhancements

- Allow [struct field init shorthand] for `leptos_fluent!` expressions macro
  parameters.

[struct field init shorthand]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand

## 2024-07-01 - [0.1.8]

### New features

- Add `tracing` feature to enable building with [`tracing`] support.

### Enhancements

- Stop depending on `wasm-bindgen` from `leptos-fluent` crate.
- Better error messages parsing parameters with `leptos_fluent!` macro.

[`tracing`]: https://docs.rs/tracing/latest/tracing/

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
  leptos_fluent! {
      // ...
      #[cfg(debug_assertions)]
      initial_language_from_url_param: true,
      #[cfg(debug_assertions)]
      set_language_to_url_param: true,
  }
  ```

[configuration conditional checks]: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html

## 2024-06-26 - [0.1.5]

### New features

- Add `leptos_fluent::SsrHtmlTag` component to render it on SSR to sync
  global attributes of `<html>` tag with the current language.
- Add new feature `system` to enable functionalities that require system
  information. Useful on non wasm targets like desktop applications.
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

### Versioning

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

[0.2.13]: https://github.com/mondeja/leptos-fluent/compare/v0.2.12...master
[0.2.12]: https://github.com/mondeja/leptos-fluent/compare/v0.2.11...v0.2.12
[0.2.11]: https://github.com/mondeja/leptos-fluent/compare/v0.2.10...v0.2.11
[0.2.10]: https://github.com/mondeja/leptos-fluent/compare/v0.2.9...v0.2.10
[0.2.9]: https://github.com/mondeja/leptos-fluent/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/mondeja/leptos-fluent/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/mondeja/leptos-fluent/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/mondeja/leptos-fluent/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/mondeja/leptos-fluent/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/mondeja/leptos-fluent/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/mondeja/leptos-fluent/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/mondeja/leptos-fluent/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/mondeja/leptos-fluent/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/mondeja/leptos-fluent/compare/v0.1.26...v0.2.0
[0.1.26]: https://github.com/mondeja/leptos-fluent/compare/v0.1.25...v0.1.26
[0.1.25]: https://github.com/mondeja/leptos-fluent/compare/v0.1.24...v0.1.25
[0.1.24]: https://github.com/mondeja/leptos-fluent/compare/v0.1.23...v0.1.24
[0.1.23]: https://github.com/mondeja/leptos-fluent/compare/v0.1.22...v0.1.23
[0.1.22]: https://github.com/mondeja/leptos-fluent/compare/v0.1.21...v0.1.22
[0.1.21]: https://github.com/mondeja/leptos-fluent/compare/v0.1.20...v0.1.21
[0.1.20]: https://github.com/mondeja/leptos-fluent/compare/v0.1.19...v0.1.20
[0.1.19]: https://github.com/mondeja/leptos-fluent/compare/v0.1.18...v0.1.19
[0.1.18]: https://github.com/mondeja/leptos-fluent/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/mondeja/leptos-fluent/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/mondeja/leptos-fluent/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/mondeja/leptos-fluent/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/mondeja/leptos-fluent/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/mondeja/leptos-fluent/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/mondeja/leptos-fluent/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/mondeja/leptos-fluent/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/mondeja/leptos-fluent/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/mondeja/leptos-fluent/compare/v0.1.8...v0.1.9
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
