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
        let mut new_cookies: Vec<String> = vec![];
        let cookies = leptos::document()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
            .cookie()
            .unwrap();
        for cookie in cookies.split(';') {
            if !cookie.starts_with(&format!("{}=", name)) {
                new_cookies.push(cookie.to_string());
            } else {
                new_cookies.push(format!("{}={}", name, value));
            }
        }
        leptos::document()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
            .set_cookie(&new_cookies.join("; "))
            .unwrap();
    }

    #[cfg(feature = "ssr")]
    {
        _ = name;
        _ = value;
    }
}
