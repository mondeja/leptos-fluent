use std::fs::{self, File};
use std::io::{Read, Write};

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn set(data_file_key: &str, language: &str) {
    if let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    {
        let data_dir = proj_dirs.data_dir();
        if !data_dir.exists() {
            _ = fs::create_dir_all(data_dir);
        }
        let data_file = data_dir.join(format!("data_file-{data_file_key}"));
        let mut file = match File::create(&data_file) {
            Ok(file) => file,
            Err(_error) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to create data file \"{}\" for key \"{}\": {:?}",
                    data_file.display(),
                    data_file_key,
                    _error
                );
                return;
            }
        };

        if let Err(_error) = file.write_all(language.as_bytes()) {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Failed to write language \"{}\" to data file \"{}\": {:?}",
                language,
                data_file.display(),
                _error
            );
        }

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Set language \"{}\" to data file \"{}\"",
            language,
            &data_file_key
        );
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn get(data_file_key: &str) -> Option<String> {
    #[cfg(feature = "tracing")]
    tracing::trace!("Getting language from data file \"{}\"", &data_file_key);

    if let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    {
        let data_dir = proj_dirs.data_dir();
        let data_file = data_dir.join(format!("data_file-{data_file_key}"));
        if !data_dir.exists() {
            _ = fs::create_dir_all(data_dir);
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Data directory \"{}\" does not exist, created. Language not found",
                &data_dir.display()
            );
            return None;
        }
        if !data_file.exists() {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Data file \"{}\" does not exist. Language not found",
                &data_file.display()
            );
            return None;
        }
        let mut file = match File::open(&data_file) {
            Ok(file) => file,
            Err(_error) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to open data file \"{}\" for key \"{}\": {:?}",
                    data_file.display(),
                    data_file_key,
                    _error
                );
                return None;
            }
        };

        let mut contents = String::new();
        if let Err(_error) = file.read_to_string(&mut contents) {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Failed to read data file \"{}\" for key \"{}\": {:?}",
                data_file.display(),
                data_file_key,
                _error
            );
            return None;
        }
        if contents.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Data file \"{:?}\" is empty. Language not found",
                &data_file
            );
            return None;
        }
        Some(contents)
    } else {
        None
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn delete(data_file_key: &str) {
    if let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    {
        let data_dir = proj_dirs.data_dir();
        let data_file = data_dir.join(format!("data_file-{data_file_key}"));
        match fs::remove_file(&data_file) {
            Ok(()) => {
                #[cfg(feature = "tracing")]
                tracing::trace!("Deleted data file \"{:?}\"", &data_file);
            }
            Err(_error) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Failed to delete data file \"{}\": {:?}",
                    data_file.display(),
                    _error
                );
            }
        }
    }
}
