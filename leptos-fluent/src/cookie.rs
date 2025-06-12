#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn get(name: &str) -> Option<String> {
    #[cfg(not(feature = "ssr"))]
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("Getting cookie \"{name}\" from browser");

        use wasm_bindgen::JsCast;
        let mut cookies = leptos::prelude::document()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
            .cookie()
            .unwrap_or("".to_string());
        if cookies.is_empty() {
            return None;
        }
        cookies.insert_str(0, "; ");
        let result = cookies
            .split(format!("; {name}=").as_str())
            .nth(1)
            .and_then(|cookie| cookie.split(';').next().map(String::from));

        #[cfg(feature = "tracing")]
        if let Some(ref result) = result {
            tracing::trace!(
                "Got cookie \"{}\" from browser: {:?}",
                name,
                result
            );
        } else {
            tracing::trace!("Got no cookie \"{}\" from browser", name);
        }

        result
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
    _ = leptos::prelude::document()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap()
        .set_cookie(new_value);
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn set(name: &str, value: &str, attrs: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        let mut new_value = format!("{name}={value}");
        if !attrs.is_empty() {
            new_value.push_str("; ");
            new_value.push_str(attrs);
        }
        set_cookie(&new_value);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Set cookie \"{}\" in browser {:?} with attributes {:?}",
            name,
            new_value,
            attrs
        );
    }

    #[cfg(feature = "ssr")]
    {
        _ = name;
        _ = value;
        _ = attrs;
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn delete(name: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        let new_value =
            format!("{name}=; expires=Thu, 01 Jan 1970 00:00:00 GMT");
        set_cookie(&new_value);

        #[cfg(feature = "tracing")]
        tracing::trace!("Deleted cookie \"{}\" in browser", name);
    }

    #[cfg(feature = "ssr")]
    {
        _ = name;
    }
}
