# Installation

<!-- toc -->

## CSR

For client side rendering apps you only need to install **leptos-fluent** and
[`fluent-templates`]:

```toml
[dependencies]
leptos-fluent = "0.1"
fluent-templates = "0.9"
```

<!-- markdownlint-disable MD033 -->

<details>
<summary>Minimal</summary>

Using `default-features = false` the `json` default feature of
**leptos-fluent** will not be enabled, so the `languages` parameter
of [`leptos_fluent!`] macro will not be available.

```toml
[dependencies]
leptos-fluent = { version = "0.1", default-features = false }
fluent-templates = "0.9"
```

</details>

<!-- markdownlint-enable MD033 -->

## SSR

For server side rendering apps you need to install **leptos-fluent**,
[`fluent-templates`] and activate the `hydrate`, `ssr` and `actix`/`axum`
features in their respective feature set of your app.

```toml
[dependencies]
leptos-fluent = "0.1"
fluent-templates = "0.9"

[features]
hydrate = [
  "leptos-fluent/hydrate"
]
ssr = [
  "leptos-fluent/ssr",
  "leptos-fluent/actix",  # actix and axum are supported
]

# Using cargo-leptos
[package.metadata.leptos]
watch-additional-files = ["locales"]
```

## Desktop applications

**leptos-fluent** can be installed on non-wasm targets, like desktop
applications. You need to install **leptos-fluent**, [`fluent-templates`]
and enable the `system` feature:

```toml
[dependencies]
leptos-fluent = { version = "0.1", features = ["system"] }
fluent-templates = "0.9"
```

```admonish example
See the [GTK example](https://github.com/mondeja/leptos-fluent/tree/master/examples/system-gtk).
```

## Nightly toolchain

**leptos-fluent** builds nightly functionalities by enabling the `nightly`
feature:

```toml
[dependencies]
leptos-fluent = { version = "0.1", features = ["nightly"] }
fluent-templates = "0.9"
```

## Language files

By default, **leptos-fluent** supports JSON languages files. If you want to
use other formats to load your custom languages, you can enable the
`json5` or `yaml` features:

```toml
[dependencies]
fluent-templates = "0.9"
leptos-fluent = {
  version = "0.1", features = ["json5"], default-features = false
}
```

```admonish tip
See [**4. Languages**](https://mondeja.github.io/leptos-fluent/languages.html).
```

## Features

- **Server Side Rendering**: `ssr`
- **Hydration**: `hydrate`
- **Actix Web integration**: `actix`
- **Axum integration**: `axum`
- **Nightly toolchain**: `nightly`
- **Desktop applications**: `system`
- **JSON languages file**: `json` (enabled by default)
- **YAML languages file**: `yaml`
- **JSON5 languages file**: `json5`

[`fluent-templates`]: https://github.com/XAMPPRocky/fluent-templates
[`leptos_fluent!`]: https://mondeja.github.io/leptos-fluent/leptos_fluent.html
