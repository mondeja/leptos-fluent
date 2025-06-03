use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, localstorage, sessionstorage};
use leptos_fluent_csr_minimal_example::LanguageSelector;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const LOCALSTORAGE_KEY: &str = "my-weird-localstorage-key";
const SESSIONSTORAGE_KEY: &str = "my-weird-sessionstorage-key";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_sessionstorage: true,
        sessionstorage_key: SESSIONSTORAGE_KEY,
        initial_language_from_sessionstorage_to_localstorage: true,
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
pub async fn test_sessionstorage_to_localstorage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_sessionstorage_to_localstorage
    localstorage::delete(LOCALSTORAGE_KEY);
    sessionstorage::delete(SESSIONSTORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }

    sessionstorage::set(SESSIONSTORAGE_KEY, "es");
    localstorage::delete(LOCALSTORAGE_KEY);
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
        assert_eq!(localstorage::get(LOCALSTORAGE_KEY), Some("es".to_string()));
    }

    sessionstorage::set(SESSIONSTORAGE_KEY, "en");
    localstorage::delete(LOCALSTORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
        assert_eq!(localstorage::get(LOCALSTORAGE_KEY), Some("en".to_string()));
    }
}
