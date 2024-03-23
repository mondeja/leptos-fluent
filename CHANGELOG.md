# CHANGELOG

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

[0.0.21]: https://github.com/mondeja/leptos-fluent/compare/v0.0.20...v0.0.21
[0.0.20]: https://github.com/mondeja/leptos-fluent/compare/v0.0.17...v0.0.20
[0.0.17]: https://github.com/mondeja/leptos-fluent/compare/v0.0.15...v0.0.17
[0.0.15]: https://github.com/mondeja/leptos-fluent/compare/v0.0.1...v0.0.15
