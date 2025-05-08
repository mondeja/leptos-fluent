use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, localstorage, url};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const URL_PARAM: &str = "my-weird-url-param";
const LOCALSTORAGE_KEY: &str = "my-weird-localstorage-key";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_url_param: true,
        url_param: URL_PARAM,
        initial_language_from_url_param_to_localstorage: true,
        localstorage_key: LOCALSTORAGE_KEY,
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
pub async fn test_url_param_to_localstorage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_url_param_to_localstorage
    url::param::delete(URL_PARAM);
    localstorage::delete(LOCALSTORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }

    url::param::set(URL_PARAM, "es");
    localstorage::delete(LOCALSTORAGE_KEY);
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
        assert_eq!(localstorage::get(LOCALSTORAGE_KEY), Some("es".to_string()));
    }

    url::param::set(URL_PARAM, "en");
    localstorage::delete(LOCALSTORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
        assert_eq!(localstorage::get(LOCALSTORAGE_KEY), Some("en".to_string()));
    }
}
