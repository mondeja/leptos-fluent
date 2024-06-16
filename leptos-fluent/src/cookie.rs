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

#[cfg(not(feature = "ssr"))]
fn set_cookie(new_value: &str) {
    use wasm_bindgen::JsCast;
    leptos::document()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap()
        .set_cookie(new_value)
        .unwrap();
}

pub fn set(name: &str, value: &str, attrs: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        let mut new_value = format!("{}={}", name, value);
        if !attrs.is_empty() {
            new_value.push_str("; ");
            new_value.push_str(attrs);
        }
        set_cookie(&new_value);
    }

    #[cfg(feature = "ssr")]
    {
        _ = name;
        _ = value;
        _ = attrs;
    }
}

pub fn delete(name: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        let new_value =
            format!("{}=; expires=Thu, 01 Jan 1970 00:00:00 GMT", name);
        set_cookie(&new_value);
    }

    #[cfg(feature = "ssr")]
    {
        _ = name;
    }
}
