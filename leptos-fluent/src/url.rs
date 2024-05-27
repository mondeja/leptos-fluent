use cfg_if::cfg_if;

pub fn get(#[allow(unused_variables)] k: &str) -> Option<String> {
    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        if let Ok(search) = leptos::window().location().search() {
            if let Ok(search_params) = web_sys::UrlSearchParams::new_with_str(&search) {
                return search_params.get(k);
            }
        }
    }};
    None
}

pub(crate) fn set(
    #[allow(unused_variables)] k: &str,
    #[allow(unused_variables)] v: &str,
) {
    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let url = web_sys::Url::new(
            &leptos::window()
                .location()
                .href()
                .expect("Failed to get location.href from the browser"),
        )
        .expect("Failed to parse location.href from the browser");
        url.search_params().set(k, v);
        leptos::window()
            .history()
            .expect("Failed to get the history from the browser")
            .replace_state_with_url(&leptos::wasm_bindgen::JsValue::NULL, "", Some(&url.href()))
            .expect("Failed to replace the history state");
    }};
}
