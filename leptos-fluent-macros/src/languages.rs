use std::fs;
use std::path::PathBuf;

pub(crate) fn read_languages_file(path: &PathBuf) -> Vec<(String, String)> {
    let file_extension = path.extension().unwrap_or_default();
    if file_extension == "json" {
        serde_json::from_str::<Vec<Vec<String>>>(
            fs::read_to_string(path)
                .expect("Couldn't read languages file")
                .as_str(),
        )
        .expect("Invalid JSON")
        .iter()
        .map(|lang| (lang[0].clone(), lang[1].clone()))
        .collect::<Vec<(String, String)>>()
    } else {
        panic!("The languages file should be a JSON file. Found file extension {:?}", file_extension);
    }
}
