# Advanced usage

<!-- toc -->

## `tr!` and `move_tr!` outside reactive graph

Outside the reactive ownership tree, mainly known as the _reactive graph_,
we can't obtain the context of `I18n` using `expect_context::<leptos_fluent::I18n>()`,
which is what `tr!` and `move_tr!` do internally. Instead, pass the context
as first parameter to the macros:

```rust
use leptos::prelude::expect_context;
use leptos_fluent::{I18n, move_tr};

let i18n = expect_context::<I18n>();
let translated_signal = move_tr!(i18n, "my-translation");
```

And some shortcuts cannot be used. Rewrite all the code that calls `expect_context`
internally:

- Use `i18n.language.set(lang)` instead of `lang.activate()`.
- Use `lang == i18n.language.get()` instead of `lang.is_active()`.

### On events, panics

For example, the next code panics when the `<div>` container is clicked:

```rust
use leptos::prelude::*;
use leptos_fluent::tr;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Show when=|| true>
            <Child/>
        </Show>
    }
}

#[component]
pub fn Child() -> impl IntoView {
    view! {
        <div on:click=|_| {
            tr!("my-translation");
        }>"CLICK ME!"</div>
    }
}
```

With Leptos v0.7, whatever `tr!` macro used in the `on:` event will panic,
but with Leptos v0.6, this outsiding of the ownership tree has been ignored
from the majority of the cases as unintended behavior.

To avoid that, pass the i18n context as first parameter to `tr!` and `move_tr!`:

```rust
use leptos::prelude::*;
use leptos_fluent::{I18n, tr};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Show when=|| true>
            <Child/>
        </Show>
    }
}

#[component]
pub fn Child() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    view! {
        <div on:click=|_| {
            tr!(i18n, "my-translation");
        }>"CLICK ME!"</div>
    }
}
```

## Dynamic text identifiers for `tr!` and `move_tr!`

Dynamic translations keys can't be passed to `tr!` and `move_tr!` macros through
variables as is because they will not be checkable at compile time
by `check_translations` option of `leptos_fluent!` macro.

```rust
use leptos_fluent::tr;

// not allowed
let text_id = "my-translation";
tr!(text_id);
/*
error: argument must be a string literal
  --> src/lib.rs
   |
   |     tr!(text_id);
   |     ^^^^^^^^^^^^
*/
```

Instead, you can use the next `i18n.tr` and `i18n.tr_with_args` methods:

```rust
use leptos::prelude::expect_context;
use leptos_fluent::I18n;

let text_id = "my-translation";
let i18n = expect_context::<I18n>();
i18n.tr(text_id);
```

```rust
use std::collections::HashMap;
use leptos::prelude::expect_context;
use leptos_fluent::I18n;

let text_id = "hello-args";
let mut args = HashMap::new();
args.insert("name", "World");

let i18n = expect_context::<I18n>();
i18n.tr_with_args(text_id, args);
```

Note that `i18n.tr` and `i18n.tr_with_args` methods are not reactive,
so you need to enclose their calls in a reactive context like a function to
update the view on the fly when the language changes, and that the translations
checker will not be able to check passed translation data at compile time,
even if they're defined as literals.

### Dynamic compile time checking

If you want dynamic translations keys to be checked at compile time with the
`check_translations` option of `leptos_fluent!` macro, you must give some
information about the keys you're using leptos-fluent.

Currently, the next patterns are supported:

#### `if (else if)? else`

```rust
use leptos_fluent::{tr, move_tr};

let (foo, bar) = (false, true);

tr!(if foo {"foo"} else {"bar"});
move_tr!(if foo {"foo"} else if bar {"bar"} else {"baz"});
```

Of course, other associated variants are supported:

```rust
// stable
#[allow(unused_parens)]
_ = tr!(
    i18n,
    if (my_signal.get() && my_function()) {"foo"} else {"bar"},
    {
        "arg1" => "value1",
        "arg2" => "value2",
    }
);

// nightly
#![feature(stmt_expr_attributes)]

_ = move_tr!(
    i18n,
    #[allow(unused_parens)]
    if (my_signal.get() && my_function()) {"foo"} else {"bar"},
    #[allow(unused_braces)]
    {
        "arg1" => "value1",
        "arg2" => {"value2"},
    }
);
```

## Configuration conditional checks

You can pass [configuration conditional checks] to most parameters of the
`leptos_fluent!` macro. For example, you can set the `set_language_to_url_param`
parameter to `true` in debug mode and to `false` in release mode:

```rust
leptos_fluent! {
    // ...
    #[cfg(debug_assertions)]
    set_language_to_url_param: true,
    #[cfg(not(debug_assertions))]
    set_language_to_url_param: false,
}
```

Options that use compile-time execution, like `check_translations`,
only accept assertions that are known at compile time for leptos-fluent.

## Destructuring `leptos_fluent!` parameters

You can destructure the runtime parameters of the `leptos_fluent!` macro to make
the code more readable. Instead of:

```rust
let my_cookie_name = "language";
leptos_fluent! {
    // ...
    cookie_name: my_cookie_name,
}
```

You can destructure the parameter `cookie_name`:

```rust
let cookie_name = "language";
leptos_fluent! {
    // ...
    cookie_name,  // `cookie_name: cookie_name`
}
```

Ideal for passing props to the internationalization provider component:

```rust
#[component]
pub fn I18n(children: Children, cookie_name: &str) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        // ...
        cookie_name,
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n cookie_name="language">
            // ...
        </I18n>
    }
}
```

[configuration conditional checks]: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
