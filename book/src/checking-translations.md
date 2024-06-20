# Checking translations

To check that the translations of your application are correct at compile time,
you can set the `check_translations` parameter in the [`leptos_fluent!`] macro to
a glob pattern that matches the Rust files that you want to check.

The pattern must be relative to the location of the _Cargo.toml_ file.

For single crate projects, it would be something like:

```rust
leptos_fluent! {{
    check_translations: "./src/**/*.rs",
    // ...
}}
```

For workspace projects, it could be something like:

```rust
leptos_fluent! {{
    check_translations: "../{app,components}/src/**/*.rs",
}}
```

## Why glob patterns to Rust files?

**leptos-fluent** provides a [`leptos_fluent::I18n`] context to Leptos when
the macro [`leptos_fluent!`] is called. So multiple instances of a context
with different localization files and strategies can be initialized in
different component trees. This is useful, for example, in a multi page app.

The mechanism of translations checking needs to know where reside the calls to
[`tr!`] and [`move_tr!`] macros to extract the messages that need to be checked.
This is performed by parsing the source code looking for these macros
invocations.

This is the main reason why **leptos-fluent** doesn't provide
ways to translate directly using methods of the [`leptos_fluent::I18n`]
context, as it would be impossible to extract the translations at compile time.

The only limitation for checking translations with glob patterns is that the
[`tr!`] and [`move_tr!`] macros that consume each context must be in
different file trees, but this enforces anyway a good practice of file-level
separation of contexts in the codebase.

[`tr!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.tr.html
[`move_tr!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.move_tr.html
[`leptos_fluent::I18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html
[`leptos_fluent!`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/macro.leptos_fluent.html
