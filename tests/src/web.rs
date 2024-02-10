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

async fn sleep(delay: i32) {
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

fn html() -> web_sys::HtmlHtmlElement {
    document()
        .document_element()
        .unwrap()
        .unchecked_into::<web_sys::HtmlHtmlElement>()
}

fn html_lang() -> String {
    html().lang()
}

#[wasm_bindgen_test]
async fn minimal_example() {
    mount!(MinimalExampleApp);
    let es = input_by_id("es");
    let en = input_by_id("en");

    // translations working
    assert_eq!(p_text(), "Select a language:");
    es.click();
    assert!(es.checked());
    assert!(!en.checked());
    assert_eq!(p_text(), "Selecciona un idioma:");

    // language change not reflected in html tag
    sleep(30).await;
    html().remove_attribute("lang").unwrap();
    assert_eq!(html_lang(), "".to_string());
    es.click();
    assert!(es.checked());
    assert_eq!(html_lang(), "".to_string());
    en.click();
    assert!(en.checked());
    assert_eq!(html_lang(), "".to_string());

    unmount!();
}

#[wasm_bindgen_test]
async fn complete_example() {
    mount!(CompleteExampleApp);
    let es = input_by_id("es");
    let en = input_by_id("en");

    // translations working
    assert_eq!(p_text(), "Select a language:");
    es.click();
    assert!(es.checked());
    assert!(!en.checked());
    assert_eq!(p_text(), "Selecciona un idioma:");
    en.click();
    assert!(en.checked());
    assert!(!es.checked());
    assert_eq!(p_text(), "Select a language:");

    // language change reflected in html tag
    sleep(30).await;
    assert_eq!(html_lang(), "en".to_string());
    es.click();
    assert!(es.checked());
    assert_eq!(html_lang(), "es".to_string());
    en.click();
    assert_eq!(html_lang(), "en".to_string());

    unmount!();
}
