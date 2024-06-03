pub fn get(name: &str) -> Option<String> {
    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;
        let mut cookies = leptos::document()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
            .cookie()
            .unwrap();
        cookies.insert_str(0, "; ");
        return cookies
            .split(&format!("; {}=", name).as_str())
            .nth(1)
            .and_then(|cookie| cookie.split(';').next().map(String::from));
    }

    #[cfg(feature = "ssr")]
    {
        _ = name;
        None
    }
}

pub fn set(name: &str, value: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;
        leptos::document()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
            .set_cookie(&format!("{}={}", name, value))
            .unwrap();
    }

    #[cfg(feature = "ssr")]
    {
        _ = name;
        _ = value;
    }
}
