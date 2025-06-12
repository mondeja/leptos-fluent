pub mod param {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all)
    )]
    pub fn get(k: &str) -> Option<String> {
        #[cfg(not(feature = "ssr"))]
        if let Ok(search) = leptos::prelude::window().location().search() {
            if let Ok(search_params) =
                web_sys::UrlSearchParams::new_with_str(&search)
            {
                let result = search_params.get(k);

                #[cfg(feature = "tracing")]
                if let Some(ref result) = result {
                    tracing::trace!(
                        "Got URL search parameter \"{}\" from browser: {:?}",
                        k,
                        result
                    );
                } else {
                    tracing::trace!(
                        "Got no URL search parameter \"{}\" from browser",
                        k
                    );
                }

                return result;
            }
        }

        #[cfg(feature = "ssr")]
        {
            _ = k;
        }

        None
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all)
    )]
    pub fn set(k: &str, v: &str) {
        #[cfg(not(feature = "ssr"))]
        {
            let url = web_sys::Url::new(
                &leptos::prelude::window()
                    .location()
                    .href()
                    .expect("Failed to get location.href from the browser"),
            )
            .expect("Failed to parse location.href from the browser");
            url.search_params().set(k, v);
            leptos::prelude::window()
                .history()
                .expect("Failed to get the history from the browser")
                .replace_state_with_url(
                    &wasm_bindgen::JsValue::NULL,
                    "",
                    Some(&url.href()),
                )
                .expect("Failed to replace the history state");

            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Set URL search parameter \"{}\" in browser with value {:?}",
                k,
                v
            );
        };

        #[cfg(feature = "ssr")]
        {
            _ = k;
            _ = v;
        };
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all)
    )]
    pub fn delete(k: &str) {
        #[cfg(not(feature = "ssr"))]
        {
            let url = web_sys::Url::new(
                &leptos::prelude::window()
                    .location()
                    .href()
                    .expect("Failed to get location.href from the browser"),
            )
            .expect("Failed to parse location.href from the browser");
            url.search_params().delete(k);
            leptos::prelude::window()
                .history()
                .expect("Failed to get the history from the browser")
                .replace_state_with_url(
                    &wasm_bindgen::JsValue::NULL,
                    "",
                    Some(&url.href()),
                )
                .expect("Failed to replace the history state");

            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Deleted URL search parameter \"{}\" in browser",
                k
            );
        };

        #[cfg(feature = "ssr")]
        {
            _ = k;
        };
    }
}

pub mod path {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all)
    )]
    pub fn get() -> Option<String> {
        #[cfg(not(feature = "ssr"))]
        if let Ok(pathname) = leptos::prelude::window().location().pathname() {
            return Some(pathname);
        }

        None
    }
}
