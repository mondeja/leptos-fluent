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
                    .query_selector("#wrapper")
                    .unwrap()
                    .unwrap()
                    .unchecked_ref(),
            )
            .unwrap();
    }};
}

async fn _sleep(delay: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, delay,
            )
            .unwrap();
    };

    let p = js_sys::Promise::new(&mut cb);
    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

#[wasm_bindgen_test]
fn minimal_example() {
    mount!(MinimalExampleApp);

    let p_text = move || {
        document()
            .query_selector("p")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap()
    };
    let get_input = move |lang: &str| {
        document()
            .query_selector(&format!("#{}", lang))
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>()
    };

    assert_eq!(p_text(), "Select a language:");
    get_input("es").click();
    assert!(get_input("es").checked());
    assert!(!get_input("en").checked());
    assert_eq!(p_text(), "Selecciona un idioma:");

    unmount!();
}

#[wasm_bindgen_test]
fn complete_example() {
    mount!(CompleteExampleApp);

    let p_text = move || {
        document()
            .query_selector("p")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap()
    };
    let get_input = move |lang: &str| {
        document()
            .query_selector(&format!("#{}", lang))
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>()
    };

    assert_eq!(p_text(), "Select a language:");
    get_input("es").click();
    assert!(get_input("es").checked());
    assert!(!get_input("en").checked());
    assert_eq!(p_text(), "Selecciona un idioma:");

    unmount!();
}
