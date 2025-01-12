use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

pub(crate) type FluentResources = HashMap<Rc<String>, Vec<String>>;
pub(crate) type FluentFilePaths = HashMap<Rc<String>, Vec<String>>;

pub(crate) fn build_fluent_resources_and_file_paths(
    dir: impl AsRef<Path>,
) -> ((FluentResources, FluentFilePaths), Vec<String>) {
    let mut resources = HashMap::new();
    let mut paths = HashMap::new();
    let mut errors = Vec::new();

    for entry in std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|rs| rs.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
    {
        if let Some(lang) = entry.file_name().into_string().ok().filter(|l| {
            l.parse::<fluent_templates::LanguageIdentifier>().is_ok()
        }) {
            let l = Rc::new(lang);
            let ((file_paths, file_contents), read_errors) =
                read_from_dir(entry.path());
            resources.insert(Rc::clone(&l), file_contents);
            paths.insert(l, file_paths);
            errors.extend(read_errors);
        } else {
            errors.push(format!(
                "Invalid language directory name: {}",
                entry.file_name().to_string_lossy()
            ));
        }
    }
    ((resources, paths), errors)
}

fn read_from_dir(
    path: impl AsRef<Path>,
) -> ((Vec<String>, Vec<String>), Vec<String>) {
    let mut paths = Vec::new();
    let mut contents = Vec::new();
    let mut errors = Vec::new();

    walkdir::WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_none_or(|e| e == "ftl"))
        .for_each(|e| {
            let p = e.path().to_owned().as_path().to_str().unwrap().to_string();
            match std::fs::read_to_string(&p) {
                Ok(string) => {
                    paths.push(p);
                    contents.push(normalize_newlines(&string));
                }
                Err(e) => {
                    errors.push(format!("Failed to read file {p}: {e}"));
                }
            }
        });

    ((paths, contents), errors)
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
