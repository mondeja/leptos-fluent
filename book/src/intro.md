# Introduction

**leptos-fluent** is a framework for internationalizing [Leptos] applications
using [Fluent]. It has all the batteries included to handle language switching,
translations management, strategies for activating translations at
initialization, different modes of persistent storage, and more.

[![Crates.io](https://img.shields.io/crates/v/leptos-fluent?logo=rust)](https://crates.io/crates/leptos-fluent)
[![License](https://img.shields.io/crates/l/leptos-fluent?logo=mit)](https://github.com/mondeja/leptos-fluent/blob/master/LICENSE.md)
[![docs.rs](https://img.shields.io/docsrs/leptos-fluent?logo=docs.rs)](https://docs.rs/leptos-fluent)
[![Crates.io downloads](https://img.shields.io/crates/d/leptos-fluent)](https://crates.io/crates/leptos-fluent)

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

[![Discord channel][discord-badge]][`#leptos-fluent` channel]

<!-- markdownlint-enable MD013 -->

You can ask for help and support in the [`#leptos-fluent` channel] of
[Leptos Discord server], [open a discussion] in the GitHub repository.
Bugs can be reported [opening an issue].

## Contributing

[![Help wanted issues](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fapi.github.com%2Fsearch%2Fissues%3Fq%3Drepo%3Amondeja%2Fleptos-fluent%2520label%3A%2522help%2520wanted%2522%2520is%3Aopen%2520-linked%3Apr&query=%24.total_count&suffix=%20open&logo=github&label=help%20wanted%20issues&color=228f6c&labelColor=228f6c&logoColor=white&style=flat-square)](https://github.com/mondeja/leptos-fluent/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)

See [_CONTRIBUTING.md_] file for more information about how to setup the
development environment and contribute to the project.

[discord-badge]: https://img.shields.io/badge/Discord-grey?logo=discord&logoColor=white
[Leptos]: https://leptos.dev
[Fluent]: https://projectfluent.org
[Leptos i18n]: https://github.com/Baptistemontan/leptos_i18n
[`#leptos-fluent` channel]: https://discord.com/channels/1031524867910148188/1251579884371705927
[Leptos Discord server]: https://discord.com/channels/1031524867910148188
[open a discussion]: https://github.com/mondeja/leptos-fluent/discussions/new
[opening an issue]: https://github.com/mondeja/leptos-fluent/issues/new
[_CONTRIBUTING.md_]: https://github.com/mondeja/leptos-fluent/blob/master/CONTRIBUTING.md
