use std::fs::{self, File};
use std::io::{Read, Write};

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
    }
}

pub fn get(data_file_key: &str) -> Option<String> {
    if let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    {
        let data_dir = proj_dirs.data_dir();
        let data_file = data_dir.join(format!("data_file-{data_file_key}"));
        if !data_dir.exists() {
            _ = fs::create_dir_all(data_dir);
            return None;
        }
        if !data_file.exists() {
            return None;
        }
        let mut file = File::open(data_file).unwrap();
        let mut contents = String::new();
        _ = file.read_to_string(&mut contents);
        Some(contents)
    } else {
        None
    }
}

pub fn delete(data_file_key: &str) {
    if let Some(proj_dirs) =
        directories::ProjectDirs::from("rs", "leptos-fluent", data_file_key)
    {
        let data_dir = proj_dirs.data_dir();
        let data_file = data_dir.join(format!("data_file-{data_file_key}"));
        _ = fs::remove_file(data_file);
    }
}
