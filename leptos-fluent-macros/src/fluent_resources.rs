use std::collections::HashMap;
use std::path::Path;

pub(crate) type FluentResources = HashMap<String, Vec<String>>;
pub(crate) type FluentFilePaths = HashMap<String, Vec<String>>;

pub(crate) fn build_fluent_resources_and_file_paths(
    dir: impl AsRef<std::path::Path>,
) -> (FluentResources, FluentFilePaths) {
    let mut resources = HashMap::new();
    let mut paths = HashMap::new();

    for entry in std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|rs| rs.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
    {
        if let Some(lang) = entry.file_name().into_string().ok().filter(|l| {
            l.parse::<fluent_templates::LanguageIdentifier>().is_ok()
        }) {
            let (file_paths, file_contents) = read_from_dir(entry.path());
            resources.insert(lang.clone(), file_contents);
            paths.insert(lang, file_paths);
        }
        // TODO: Handle error
    }
    (resources, paths)
}

fn read_from_dir<P: AsRef<Path>>(path: P) -> (Vec<String>, Vec<String>) {
    let mut paths = vec![];
    let mut contents = vec![];

    walkdir::WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |e| e == "ftl"))
        .for_each(|e| {
            let p = e.path().to_owned().as_path().to_str().unwrap().to_string();
            if let Ok(string) = std::fs::read_to_string(&p) {
                paths.push(p);
                contents.push(normalize_newlines(&string));
            }
            // TODO: Handle error
        });

    (paths, contents)
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
