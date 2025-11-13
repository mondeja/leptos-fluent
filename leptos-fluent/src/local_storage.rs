#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn get(key: &str) -> Option<String> {
    #[cfg(feature = "tracing")]
    tracing::trace!("Getting local storage key \"{}\"", key);

    #[cfg(not(feature = "ssr"))]
    {
        let storage = match leptos::prelude::window().local_storage() {
            Ok(Some(storage)) => storage,
            Ok(None) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Local storage unavailable in browser when getting key \"{}\"",
                    key
                );
                return None;
            }
            Err(_error) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to access local storage when getting key \"{}\": {:?}",
                    key,
                    _error
                );
                return None;
            }
        };

        match storage.get_item(key) {
            Ok(Some(result)) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Got local storage key \"{}\" from browser: {:?}",
                    key,
                    result
                );

                Some(result)
            }
            Ok(None) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Got no local storage key \"{}\" from browser",
                    key
                );

                None
            }
            Err(_error) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to get local storage key \"{}\" from browser: {:?}",
                    key,
                    _error
                );

                None
            }
        }
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
    if let Ok(Some(storage)) = ::leptos::prelude::window().local_storage() {
        _ = storage.set_item(key, value);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Set local storage key \"{}\" in browser with value {:?}",
            key,
            value
        );
    } else {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Local storage unavailable in browser when setting key \"{}\"",
            key
        );
    }

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
        if let Ok(Some(storage)) = ::leptos::prelude::window().local_storage() {
            _ = storage.remove_item(key);
            #[cfg(feature = "tracing")]
            tracing::trace!("Deleted local storage key \"{}\" in browser", key);
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Local storage unavailable in browser when deleting key \"{}\"",
                key
            );
        }
    }

    #[cfg(feature = "ssr")]
    {
        _ = key;
    }
}
