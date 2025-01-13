# SSR example with Axum using islands for leptos-fluent

When using the islands feature, the app is rendered as static HTML by default.
To enable interactivity, the `#[island]` macro must be applied selectively.
Typically, with an internationalization framework like `leptos-fluent`, nearly
every part of the website is translated reactively, which would require using
the `#[island]` macro extensively.

However, this approach conflicts with one of the core principles of islands
architecture: islands should be as minimal and targeted as possible.

To benefit from smaller WebAssembly (wasm) file sizes, this example opts to keep
all translations server-side, instead of using the dynamic translation updates
provided by `move_tr!`. The page is reloaded after a language change, ensuring
the translations are updated without excessive use of islands.

To run:

```sh
cargo leptos watch
```
