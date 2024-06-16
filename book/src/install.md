# Installation

## CSR

For client side rendering apps you only need to install **leptos-fluent** and
[`fluent-templates`]:

```toml
[dependencies]
leptos-fluent = "0.1"
fluent-templates = "0.9"
```

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
```

[`fluent-templates`]: https://github.com/XAMPPRocky/fluent-templates
