use end2end_helpers::{input_by_id, mount, unmount};
/// See:
/// - https://github.com/leptos-rs/leptos/issues/2852
/// - https://github.com/mondeja/leptos-fluent/issues/231
use leptos::{control_flow::Show, prelude::*};
use leptos_fluent::{leptos_fluent, use_i18n};
use leptos_fluent_csr_minimal_example::TRANSLATIONS;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[component]
fn App() -> impl IntoView {
    view! {
        <Show when=|| true>
            <Child />
        </Show>
    }
}

#[component]
fn Child() -> impl IntoView {
    use wasm_bindgen::JsCast;
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
    };
    view! {
        <div
            id="fails"
            on:click=|ev| {
                if use_i18n().is_some() {
                    ev.target()
                        .unwrap()
                        .unchecked_into::<web_sys::HtmlElement>()
                        .set_inner_text("CLICKED!");
                }
            }
        >

            "CLICK ME!"
        </div>
        <div
            id="success"
            on:click=|ev| {
                ev.target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlElement>()
                    .set_inner_text("CLICKED!");
            }
        >

            "CLICK ME!"
        </div>
    }
}

#[wasm_bindgen_test]
async fn context_outside_reactive_ownership_tree() {
    let fails_div = move || input_by_id("fails");
    let success_div = move || input_by_id("success");

    mount!(App);
    assert_eq!(fails_div().inner_text(), "CLICK ME!");
    fails_div().click();
    assert_eq!(fails_div().inner_text(), "CLICK ME!");
    unmount!();

    mount!(App);
    assert_eq!(success_div().inner_text(), "CLICK ME!");
    success_div().click();
    assert_eq!(success_div().inner_text(), "CLICKED!");
    unmount!();
}
