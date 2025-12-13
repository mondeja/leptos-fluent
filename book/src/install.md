<!-- markdownlint-disable MD033 -->

# Installation

<!-- toc -->

## CSR

For client side rendering apps just install **leptos-fluent**:

```toml
[dependencies]
leptos-fluent = "0.2"
```

## SSR

For server side rendering apps install **leptos-fluent** and enable
the `hydrate`, `ssr` and `actix`/`axum` features in their respective
features set.

```toml
[dependencies]
leptos-fluent = "0.2"
axum = { version = "0.8", optional = true }

[features]
hydrate = [
  "leptos-fluent/hydrate",
]
ssr = [
  "leptos-fluent/ssr",
  "leptos-fluent/axum",  # actix and axum are supported
  "dep:axum",
]

# Using cargo-leptos
[package.metadata.leptos]
watch-additional-files = ["locales"]
```

## Desktop applications

**leptos-fluent** can be installed on non-wasm targets, like desktop
applications. You need to install **leptos-fluent**, and enable the
`system` feature:

```toml
[dependencies]
leptos-fluent = { version = "0.2", features = ["system"] }
```

## Features

- **Server Side Rendering**: `ssr`
- **Hydration**: `hydrate`
- **Actix Web integration**: `actix`
- **Axum integration**: `axum`
- **Nightly toolchain**: `nightly`
- **Desktop applications**: `system`
- **JSON languages file**: `json`
- **YAML languages file**: `yaml`
- **JSON5 languages file**: `json5`
- **Tracing support**: `tracing`
- **Debugging**: `debug`

## Nightly toolchain

**leptos-fluent** builds nightly functionalities by enabling the `nightly`
feature:

```toml
[dependencies]
leptos-fluent = { version = "0.2", features = ["nightly"] }
```

## Language files

By default, **leptos-fluent** supports JSON languages files. To use other
formats to load custom languages, the `json5` or `yaml` features can be
enabled:

<!-- markdownlint-disable MD013 -->

```toml
[dependencies]
leptos-fluent = { version = "0.2", features = ["json5"] }
```

<!-- markdownlint-enable MD013 -->

```admonish tip
See [**4. Languages**](https://mondeja.github.io/leptos-fluent/latest/languages.html).
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
leptos-fluent = { version = "0.2", features = ["tracing"] }
fluent-templates = "0.13"
```

[`cargo leptos`]: https://github.com/leptos-rs/cargo-leptos
[`tracing`]: https://docs.rs/tracing/latest/tracing

## Compatibility

| **leptos-fluent** | **leptos** |
| ----------------- | ---------- |
| 0.2               | 0.7 / 0.8  |
| 0.1               | 0.6        |
