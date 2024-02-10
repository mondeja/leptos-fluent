use leptos::{wasm_bindgen::JsCast, *};
use leptos_fluent_complete_example::App as CompleteExampleApp;
use leptos_fluent_minimal_example::App as MinimalExampleApp;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

macro_rules! mount {
    ($app:ident) => {{
        mount_to_body(move || view! { <div id="wrapper"><$app/></div> });
    }};
}

macro_rules! unmount {
    () => {{
        document()
            .body()
            .unwrap()
            .remove_child(
                document()
                    .get_element_by_id("wrapper")
                    .unwrap()
                    .unchecked_ref(),
            )
            .unwrap();
    }};
}

fn p_text() -> String {
    document()
        .query_selector("p")
        .unwrap()
        .unwrap()
        .text_content()
        .unwrap()
}

fn input_by_id(id: &str) -> web_sys::HtmlInputElement {
    document()
        .get_element_by_id(id)
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
}

#[wasm_bindgen_test]
fn minimal_example() {
    mount!(MinimalExampleApp);

    assert_eq!(p_text(), "Select a language:");
    input_by_id("es").click();
    assert!(input_by_id("es").checked());
    assert!(!input_by_id("en").checked());
    assert_eq!(p_text(), "Selecciona un idioma:");

    unmount!();
}

#[wasm_bindgen_test]
fn complete_example() {
    mount!(CompleteExampleApp);

    assert_eq!(p_text(), "Select a language:");
    input_by_id("es").click();
    assert!(input_by_id("es").checked());
    assert!(!input_by_id("en").checked());
    assert_eq!(p_text(), "Selecciona un idioma:");

    unmount!();
}
