#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn get(key: &str) -> Option<String> {
    #[cfg(feature = "tracing")]
    tracing::trace!("Getting session storage key \"{}\"", key);

    #[cfg(not(feature = "ssr"))]
    {
        let storage = match leptos::prelude::window().session_storage() {
            Ok(Some(storage)) => storage,
            Ok(None) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Session storage unavailable in browser when getting key \"{}\"",
                    key
                );
                return None;
            }
            Err(_error) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to access session storage when getting key \"{}\": {:?}",
                    key,
                    _error
                );
                return None;
            }
        };

        let result = storage.get_item(key).ok().flatten();

        #[cfg(feature = "tracing")]
        if let Some(ref result) = result {
            tracing::trace!(
                "Got session storage key \"{}\" from browser: {:?}",
                key,
                result
            );
        } else {
            tracing::trace!(
                "Got no session storage key \"{}\" from browser",
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
    if let Ok(Some(storage)) = ::leptos::prelude::window().session_storage() {
        _ = storage.set_item(key, value);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Set session storage key \"{}\" in browser with value {:?}",
            key,
            value
        );
    } else {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Session storage unavailable in browser when setting key \"{}\"",
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
        if let Ok(Some(storage)) = ::leptos::prelude::window().session_storage()
        {
            _ = storage.remove_item(key);
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Deleted session storage key \"{}\" in browser",
                key
            );
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Session storage unavailable in browser when deleting key \"{}\"",
                key
            );
        }
    }

    #[cfg(feature = "ssr")]
    {
        _ = key;
    }
}
