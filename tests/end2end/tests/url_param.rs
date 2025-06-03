use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, url};
use leptos_fluent_csr_minimal_example::LanguageSelector;
use wasm_bindgen_test::*;
use web_sys_ec::{Ec, Wait};

wasm_bindgen_test_configure!(run_in_browser);

const URL_PARAM: &str = "my-weird-url-param";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_url_param: true,
        url_param: URL_PARAM,
        set_language_to_url_param: true,
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <I18n>
            <LanguageSelector />
        </I18n>
    }
}

#[wasm_bindgen_test]
pub async fn test_url_param() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // set_language_to_url_param
    {
        mount!(App);
        assert_eq!(leptos::prelude::window().location().search().unwrap(), "");
        es().click();
        Wait(1)
            .until(Ec::LocationSearchIs(format!("?{URL_PARAM}=es")))
            .await;
        en().click();
        Wait(1)
            .until(Ec::LocationSearchIs(format!("?{URL_PARAM}=en")))
            .await;
    }

    // initial_language_from_url_param
    url::param::delete(URL_PARAM);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }

    url::param::set(URL_PARAM, "es");
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
    }

    url::param::set(URL_PARAM, "en");
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }
}
