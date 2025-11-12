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
            let window = leptos::prelude::window();
            let href = match window.location().href() {
                Ok(href) => href,
                Err(error) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(
                        "Failed to get location.href from the browser when setting URL parameter \"{}\": {:?}",
                        k,
                        error
                    );
                    return;
                }
            };

            let url = match web_sys::Url::new(&href) {
                Ok(url) => url,
                Err(error) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(
                        "Failed to parse location.href when setting URL parameter \"{}\": {:?}",
                        k,
                        error
                    );
                    return;
                }
            };
            url.search_params().set(k, v);

            if let Ok(history) = window.history() {
                if let Err(error) = history.replace_state_with_url(
                    &wasm_bindgen::JsValue::NULL,
                    "",
                    Some(&url.href()),
                ) {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(
                        "Failed to replace the history state when setting URL parameter \"{}\": {:?}",
                        k,
                        error
                    );
                }
            } else {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to get the history from the browser when setting URL parameter \"{}\"",
                    k
                );
            }

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
            let window = leptos::prelude::window();
            let href = match window.location().href() {
                Ok(href) => href,
                Err(error) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(
                        "Failed to get location.href from the browser when deleting URL parameter \"{}\": {:?}",
                        k,
                        error
                    );
                    return;
                }
            };

            let url = match web_sys::Url::new(&href) {
                Ok(url) => url,
                Err(error) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(
                        "Failed to parse location.href when deleting URL parameter \"{}\": {:?}",
                        k,
                        error
                    );
                    return;
                }
            };
            url.search_params().delete(k);

            if let Ok(history) = window.history() {
                if let Err(error) = history.replace_state_with_url(
                    &wasm_bindgen::JsValue::NULL,
                    "",
                    Some(&url.href()),
                ) {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(
                        "Failed to replace the history state when deleting URL parameter \"{}\": {:?}",
                        k,
                        error
                    );
                }
            } else {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to get the history from the browser when deleting URL parameter \"{}\"",
                    k
                );
            }

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
