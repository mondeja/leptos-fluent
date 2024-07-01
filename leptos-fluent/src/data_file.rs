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
        let mut file = File::create(data_file).unwrap();
        _ = file.write_all(language.as_bytes());

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
                data_dir.display()
            );
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
        let mut file = File::open(data_file.clone()).unwrap();
        let mut contents = String::new();
        _ = file.read_to_string(&mut contents);
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
        _ = fs::remove_file(&data_file);
        #[cfg(feature = "tracing")]
        tracing::trace!("Deleted data file \"{:?}\"", &data_file);
    }
}
