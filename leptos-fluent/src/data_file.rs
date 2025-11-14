use std::fs::{self, File};
use std::io::{Read, Write};

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn set(data_file_key: &str, language: &str) {
    let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    else {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Project directories unavailable for data file key \"{}\"",
            data_file_key
        );
        return;
    };

    let data_dir = proj_dirs.data_dir();
    if !data_dir.exists() && fs::create_dir_all(data_dir).is_err() {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Failed to create data directory \"{}\" for key \"{}\"",
            data_dir.display(),
            data_file_key
        );
        return;
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
        return;
    }

    #[cfg(feature = "tracing")]
    tracing::trace!(
        "Set language \"{}\" to data file \"{}\"",
        language,
        &data_file_key
    );
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn get(data_file_key: &str) -> Option<String> {
    #[cfg(feature = "tracing")]
    tracing::trace!("Getting language from data file \"{}\"", &data_file_key);

    let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    else {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Project directories unavailable for data file key \"{}\"",
            data_file_key
        );
        return None;
    };

    let data_dir = proj_dirs.data_dir();
    let data_file = data_dir.join(format!("data_file-{data_file_key}"));
    if !data_dir.exists() {
        if fs::create_dir_all(data_dir).is_err() {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Failed to create data directory \"{}\" for key \"{}\"",
                data_dir.display(),
                data_file_key
            );
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Data directory \"{}\" did not exist, created. Language not found",
                data_dir.display()
            );
        }
        return None;
    }
    if !data_file.exists() {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Data file \"{}\" does not exist. Language not found",
            data_file.display()
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
    if contents.trim().is_empty() {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Data file \"{}\" is empty. Language not found",
            data_file.display()
        );
        return None;
    }
    Some(contents)
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn delete(data_file_key: &str) {
    let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    else {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Project directories unavailable for deleting data file key \"{}\"",
            data_file_key
        );
        return;
    };

    let data_dir = proj_dirs.data_dir();
    let data_file = data_dir.join(format!("data_file-{data_file_key}"));
    match fs::remove_file(&data_file) {
        Ok(()) => {
            #[cfg(feature = "tracing")]
            tracing::trace!("Deleted data file \"{}\"", data_file.display());
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Data file \"{}\" not found when attempting to delete",
                data_file.display()
            );
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
