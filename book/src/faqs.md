<!-- markdownlint-disable MD001 -->

# FAQs

<!-- toc -->

### How to get [`LanguageIdentifier`]?

```rust
use fluent_templates::LanguageIdentifier;
```

```admonish tip
`fluent-templates` also depends externally on
[`fluent-bundle`](https://docs.rs/fluent-bundle/latest/fluent_bundle/)
whichs provides utilities for parsing the Fluent syntax.
```

[`LanguageIdentifier`]: https://docs.rs/unic-langid/latest/unic_langid/struct.LanguageIdentifier.html

### How to get the [i18n context] at initialization?

```rust
use leptos_fluent::leptos_fluent;

let i18n = leptos_fluent! {
    // ...
};

leptos::logging::log!("i18n context: {i18n:#?}");
```

[i18n context]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html

### Custom [cookie attributes] are invalid

Use an expression to set the cookie attributes and will not be validated.

```rust
let attrs = "SameSite=Strict; MyCustomAttr=MyCustomValue;";
leptos_fluent! {
    cookie_attrs: attrs,
    // ...
};
```

[cookie attributes]: https://developer.mozilla.org/docs/Web/API/Document/cookie#write_a_new_cookie

### How to get the fallback language

From fluent-templates `v0.10` onwards can be obtained from your translations.

```rust
let fallback_language = expect_i18n().translations.get()[0].fallback();
```

### `tr!` and `move_tr!` outside reactive graph

Outside the reactive ownership tree, mainly known as the _reactive graph_,
we can't obtain the context of `I18n` using `expect_context::<leptos_fluent::I18n>()`,
which is what `tr!` and `move_tr!` do internally. Instead, we can pass the context
as first parameter to the macros:

```rust
let i18n = leptos_fluent! {
    // ...
};

let translated_signal = move_tr!(i18n, "my-translation");
```

And some shortcuts cannot be used. Rewrite all the code that calls `expect_context`
internally:

- Use `i18n.language.set(lang)` instead of `lang.activate()`.
- Use `lang == i18n.language.get()` instead of `lang.is_active()`.

#### On events, panics

For example, the next code panics when the `<div>` container is clicked:

```rust
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
    leptos_fluent! {
        // ...
    };
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

To avoid that, pass the i18n context as first parameter to `tr!` or `move_tr!`:

```rust
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
    let i18n = leptos_fluent! {
        // ...
    };
    view! {
        <div on:click=|_| {
            tr!(i18n, "my-translation");
        }>"CLICK ME!"</div>
    }
}
```

#### Confused about what context is used?

Take into account that the reactive ownership graph is not the same as the component
tree in Leptos. For example, the next code:

```rust
#[component]
fn Foo() -> impl IntoView {
    provide_context::<usize>(0);

    view! {
        <h1>"Foo"</h1>
        {
            let value = expect_context::<usize>();
            view! {
                <p>"Context value before Bar: "{value}</p>
            }
        }
        <Bar/>
        {
            let value = expect_context::<usize>();
            view! {
                <p>"Context value after Bar -> Baz: "{value}</p>
            }
        }
    }
}

#[component]
fn Bar() -> impl IntoView {
    provide_context::<usize>(1);
    view! {
        <h1>"Bar"</h1>
        {
            let value = expect_context::<usize>();
            view! {
                <p>"Context value before Baz: "{value}</p>
            }
        }
        <Baz/>
    }
}

#[component]
fn Baz() -> impl IntoView {
    provide_context::<usize>(2);
    view! {
        <h1>"Baz"</h1>
    }
}
```

Renders:

```html
<h1>Foo</h1>
<p>Context value before Bar: 0</p>
<h1>Bar</h1>
<p>Context value before Baz: 1</p>
<h1>Baz</h1>
<p>Context value after Bar -&gt; Baz: 2</p>
```

Because `Baz` is a sibling of `Foo` children in the reactive graph. But maybe
you think that is just a children of `Bar` in the component tree and that is
outside the scope of `Foo` children. That doesn't matter for Leptos.

In those cases where you're using two or more contexts, pass the context as the
first argument to the `tr!` and `move_tr!` macros to avoid confusion.

```rust
#[component]
fn Foo() -> impl IntoView {
    let i18n = leptos_fluent! {
        translations: [TRANSLATION_WITH_ONLY_FOO],
        // ...
    };
    <p>{move_tr!("my-translation-from-foo")}</p>
    <Bar/>
    // The next message will not be translated because after `<Bar>`
    // now the i18n context accessed by `move_tr!` is the one from `Bar`
    <p>{move_tr!("my-translation-from-foo")}</p>
    // instead, use:
    <p>{move_tr!(i18n, "my-translation-from-foo")}</p>
}

#[component]
fn Bar() -> impl IntoView {
    let i18n = leptos_fluent! {
        translations: [TRANSLATION_WITH_ONLY_BAR],
        // ...
    };
    <p>{move_tr!("my-translation-from-bar")}</p>
}
```

### Why examples don't use [`<For/>`] component?

```admonish bug
There are some cases in which the [`<For/>`] component is not reproducible between
SSR and hydrate modes leading to different renders, so decided to use a
simple vector to not bring confusion to main examples.
```

In any case, the [`<For/>`] component is safe on CSR contexts and
[`leptos_fluent::Language`] implement `Hash` and `Eq` traits to be
able to be passed directly to `key`s properties trigerring reactivity
depending on the current active language.

```rust
use leptos_fluent::{i18n, Language};

leptos::logging::warn!("[WARNING]: Not secure on SSR");
view! {
    <p>{move_tr!("select-a-language")}</p>
    <For
        each=move || i18n().languages
        key=|lang| *lang
        children=move |lang| render_language(lang)
    />
}

fn render_language(lang: &'static Language) -> impl IntoView { ... }
```

### How to manage translations on server actions

The translations reside on the client side, so the [`I18n`] can not be
accessed as context on server actions. Pass the translations as values
if the bandwidth is not a problem or use your own statics on server side.

```rust
use leptos::*;
use leptos_fluent::{tr, Language};

/// Server action showing client-side translated message on console
#[server(ShowHelloWorld, "/api")]
pub async fn show_hello_world(
    translated_hello_world: String,
    language: String,
) -> Result<(), ServerFnError> {
    println!("{translated_hello_world} ({language})");
    Ok(())
}

fn render_language(lang: &'static Language) -> impl IntoView {
    // Call on click to server action with a client-side translated
    // "hello-world" message
    let on_click = move |_| {
        lang.activate();
        spawn_local(async {
            _ = show_hello_world(
                tr!("hello-world"),
                lang.name.to_string(),
            ).await;
        });
    };

    view! {
        <div>
            <label for=lang>{lang.name}</label>
            <input
                id=lang
                name="language"
                value=lang
                checked=lang.is_active()
                on:click=on_click
                type="radio"
            />
        </div>
    }
}
```

### How to get values of `leptos_fluent!` macro at runtime?

Use `provide_meta_context` at the macro initialization and get them
with the method `I18n::meta`:

```rust
let i18n = leptos_fluent! {
    // ...
    provide_meta_context: true,
};

println!("Macro parameters: {:?}", i18n.meta().unwrap());
```

### [Configuration conditional checks]

```rust
leptos_fluent! {
    // ...
    #[cfg(debug_assertions)]
    set_language_to_url_param: true,
    #[cfg(not(debug_assertions))]
    set_language_to_url_param: false,
}
```

[configuration conditional checks]: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
[`<For/>`]: https://docs.rs/leptos/latest/leptos/control_flow/fn.For.html
[`leptos_fluent::Language`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.Language.html
[`I18n`]: https://docs.rs/leptos-fluent/latest/leptos_fluent/struct.I18n.html
