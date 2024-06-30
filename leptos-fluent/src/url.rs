pub fn get(k: &str) -> Option<String> {
    #[cfg(not(feature = "ssr"))]
    if let Ok(search) = leptos::window().location().search() {
        if let Ok(search_params) =
            web_sys::UrlSearchParams::new_with_str(&search)
        {
            return search_params.get(k);
        }
    }

    #[cfg(feature = "ssr")]
    {
        _ = k;
    }

    None
}

pub fn set(k: &str, v: &str) {
    #[cfg(not(feature = "ssr"))]
    {
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
            .replace_state_with_url(
                &crate::web_sys::wasm_bindgen::JsValue::NULL,
                "",
                Some(&url.href()),
            )
            .expect("Failed to replace the history state");
    };

    #[cfg(feature = "ssr")]
    {
        _ = k;
        _ = v;
    };
}

pub fn delete(k: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        let url = web_sys::Url::new(
            &leptos::window()
                .location()
                .href()
                .expect("Failed to get location.href from the browser"),
        )
        .expect("Failed to parse location.href from the browser");
        url.search_params().delete(k);
        leptos::window()
            .history()
            .expect("Failed to get the history from the browser")
            .replace_state_with_url(
                &crate::web_sys::wasm_bindgen::JsValue::NULL,
                "",
                Some(&url.href()),
            )
            .expect("Failed to replace the history state");
    };

    #[cfg(feature = "ssr")]
    {
        _ = k;
    };
}
