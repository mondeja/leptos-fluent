use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, local_storage, url};
use leptos_fluent_csr_minimal_example::LanguageSelector;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const URL_PARAM: &str = "my-weird-url-param";
const LOCAL_STORAGE_KEY: &str = "my-weird-local-storage-key";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_url_param: true,
        url_param: URL_PARAM,
        initial_language_from_url_param_to_local_storage: true,
        local_storage_key: LOCAL_STORAGE_KEY,
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
pub async fn test_url_param_to_local_storage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_url_param_to_local_storage
    url::param::delete(URL_PARAM);
    local_storage::delete(LOCAL_STORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }

    url::param::set(URL_PARAM, "es");
    local_storage::delete(LOCAL_STORAGE_KEY);
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
        assert_eq!(
            local_storage::get(LOCAL_STORAGE_KEY),
            Some("es".to_string())
        );
    }

    url::param::set(URL_PARAM, "en");
    local_storage::delete(LOCAL_STORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
        assert_eq!(
            local_storage::get(LOCAL_STORAGE_KEY),
            Some("en".to_string())
        );
    }
}
