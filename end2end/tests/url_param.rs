use leptos::*;
use leptos_fluent::{leptos_fluent, url};
use leptos_fluent_csr_minimal_example::{ChildComponent, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, sleep, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const URL_PARAM: &str = "my-weird-url-param";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "../examples/csr-minimal/locales",
        initial_language_from_url_param: true,
        url_param: URL_PARAM,
        set_language_to_url_param: true,
    }};

    view! { <ChildComponent/> }
}

#[wasm_bindgen_test]
async fn test_url_param() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // set_language_to_url_param
    mount!(App);
    assert_eq!(leptos::window().location().search().unwrap(), "");
    es().click();
    sleep(30).await;
    assert_eq!(
        leptos::window().location().search().unwrap(),
        format!("?{}=es", URL_PARAM)
    );
    en().click();
    sleep(30).await;
    assert_eq!(
        leptos::window().location().search().unwrap(),
        format!("?{}=en", URL_PARAM)
    );
    unmount!();

    // initial_language_from_url_param
    url::delete(URL_PARAM);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();

    url::set(URL_PARAM, "es");
    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    unmount!();

    url::set(URL_PARAM, "en");
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();
}
