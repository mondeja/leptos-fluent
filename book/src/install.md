<!-- markdownlint-disable MD033 -->

# Installation

<!-- toc -->

## CSR

For client side rendering apps install **leptos-fluent** and [`fluent-templates`]:

```toml
[dependencies]
leptos-fluent = "0.1"
fluent-templates = "0.9"
```

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

## SSR

For server side rendering apps install **leptos-fluent**, [`fluent-templates`]
and activate the `hydrate`, `ssr` and `actix`/`axum` features in their
respective features set.

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
applications. You need to install `leptos-fluent`, [`fluent-templates`]
and enable the `system` feature:

```toml
[dependencies]
leptos-fluent = { version = "0.1", features = ["system"] }
fluent-templates = "0.9"
```

```admonish example
See the [GTK example](https://github.com/mondeja/leptos-fluent/tree/master/examples/system-gtk).
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
- **Tracing support**: `tracing`

## Nightly toolchain

**leptos-fluent** builds nightly functionalities by enabling the `nightly`
feature:

```toml
[dependencies]
leptos-fluent = { version = "0.1", features = ["nightly"] }
fluent-templates = "0.9"
```

## Language files

By default, **leptos-fluent** supports JSON languages files. To use other
formats to load custom languages, the `json5` or `yaml` features can be
enabled:

<!-- markdownlint-disable MD013 -->

```toml
[dependencies]
fluent-templates = "0.9"
leptos-fluent = { version = "0.1", features = ["json5"], default-features = false }
```

<!-- markdownlint-enable MD013 -->

```admonish tip
See [**4. Languages**](https://mondeja.github.io/leptos-fluent/languages.html).
```

## Tracking locales files with [`cargo leptos`]

Using [`cargo leptos`] watch of the _locales/_ folder for reloads:

```toml
# Relative to Cargo.toml file
[package.metadata.leptos]
watch-additional-files = ["locales"]
```

When inside a workspace, use the full path to the folder from the
workspace _Cargo.toml_ file:

```toml
 # Relative to workspace Cargo.toml file
[package.metadata.leptos]
watch-additional-files = ["examples/csr/locales"]
```

## Tracing

To enable [`tracing`] support, add the `tracing` feature to **leptos-fluent**:

```toml
[dependencies]
leptos-fluent = { version = "0.1", features = ["tracing"] }
fluent-templates = "0.9"
```

```admonish example
See the [GTK example](https://github.com/mondeja/leptos-fluent/tree/master/examples/system-gtk).
```

[`fluent-templates`]: https://github.com/XAMPPRocky/fluent-templates
[`leptos_fluent!`]: https://mondeja.github.io/leptos-fluent/leptos_fluent.html
[`cargo leptos`]: https://github.com/leptos-rs/cargo-leptos
[`tracing`]: https://docs.rs/tracing/latest/tracing
