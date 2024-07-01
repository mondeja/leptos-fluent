#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn get(key: &str) -> Option<String> {
    #[cfg(feature = "tracing")]
    tracing::trace!("Getting local storage key \"{}\"", key);

    #[cfg(not(feature = "ssr"))]
    {
        let result = leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(key)
            .unwrap_or(None);

        #[cfg(feature = "tracing")]
        if let Some(ref result) = result {
            tracing::trace!(
                "Got local storage key \"{}\" from browser: {:?}",
                key,
                result
            );
        } else {
            tracing::trace!(
                "Got no local storage key \"{}\" from browser",
                key
            );
        }

        result
    }

    #[cfg(feature = "ssr")]
    {
        _ = key;
        None
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn set(key: &str, value: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        _ = ::leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(key, value);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Set local storage key \"{}\" in browser with value {:?}",
            key,
            value
        );
    };

    #[cfg(feature = "ssr")]
    {
        _ = key;
        _ = value;
    };
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn delete(key: &str) {
    #[cfg(not(feature = "ssr"))]
    {
        _ = ::leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .remove_item(key);
        #[cfg(feature = "tracing")]
        tracing::trace!("Deleted local storage key \"{}\" in browser", key);
    }

    #[cfg(feature = "ssr")]
    {
        _ = key;
    }
}
