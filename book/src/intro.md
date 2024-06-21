# Introduction

**leptos-fluent** is a framework for internationalizing [Leptos] applications
using [Fluent]. It has all the batteries included to handle language switching,
translations management, strategies for activating translations at
initialization, different modes of persistent storage, and more.

## Alternatives

The unique alternative to **leptos-fluent** currently is [Leptos i18n], which
follows a different approach. The main differences are:

- `leptos-fluent` uses [Fluent] files and the Fluent syntax for translations,
  while `leptos_i18n` uses JSON files and a custom syntax.
- `leptos-fluent` defines all the configuration in a macro and an optional
  languages file, while `leptos_i18n` defines the configuration in a
  `[package.metadata.leptos-i18n]` section in _Cargo.toml_.
- `leptos-fluent` allows to instantiate multiple internationalization contexts,
  while `leptos_i18n` only allows one.
- `leptos-fluent` has a lot of strategies for activating the initial language
  of the user at initialization and updating it when the user changes the
  language, while `leptos_i18n` only follows the cookie strategy.
- `leptos-fluent` automatically builds language names and directions based on
  language codes, while `leptos_i18n` don't.
- `leptos-fluent` has multiple side effects for updating the language in the
  client, while `leptos_i18n` only has the `<html lang="...">` attribute
  and is not configurable.

## Project goals

The main goals of **leptos-fluent** are:

- To provide a simple and easy-to-use API for internationalizing Leptos
  applications.
- Be the most fully featured internationalization framework in any language.
- Be the most performant internationalization framework available.

Defining the internationalization strategy using a macro allows to generate
the less code possible for each possible configuration.

## Help and support

<!-- markdownlint-disable MD013 -->

[![Discord channel](https://img.shields.io/badge/Discord%20server-grey?logo=discord&logoColor=white)][`leptos-fluent` channel]

<!-- markdownlint-enable MD013 -->

You can ask for help and support in the [`leptos-fluent` channel] of
[Leptos Discord server]. Additionally, if you experience bugs or just don't
like chat servers you can [open an issue in the GitHub repository].

[Leptos]: https://leptos.dev
[Fluent]: https://projectfluent.org
[Leptos i18n]: https://github.com/Baptistemontan/leptos_i18n
[`leptos-fluent` channel]: https://discord.com/channels/1031524867910148188/1251579884371705927
[Leptos Discord server]: https://discord.com/channels/1031524867910148188
[open an issue in the GitHub repository]: https://github.com/mondeja/leptos-fluent/issues/new
