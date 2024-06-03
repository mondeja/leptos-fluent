use std::collections::HashMap;
use std::path::Path;

pub(crate) type FluentResources = HashMap<String, (Vec<String>, Vec<String>)>;

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
                contents.push(string);
            }
            // TODO: Handle error
        });

    (paths, contents)
}

pub(crate) fn build_fluent_resources(
    dir: impl AsRef<std::path::Path>,
) -> FluentResources {
    let mut all_resources = HashMap::new();
    for entry in std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|rs| rs.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
    {
        if let Some(lang) = entry.file_name().into_string().ok().filter(|l| {
            l.parse::<fluent_templates::LanguageIdentifier>().is_ok()
        }) {
            let (file_paths, resources) = read_from_dir(entry.path());
            all_resources.insert(lang, (file_paths, resources));
        }
    }
    all_resources
}
