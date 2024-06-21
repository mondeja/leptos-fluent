use crate::FluentFilePaths;

pub(crate) fn build_files_tracker_quote(
    fluent_resources: &FluentFilePaths,
    languages_path: &Option<String>,
    core_locales_path: &Option<String>,
) -> proc_macro2::TokenStream {
    let mut files_tracker_str = "{".to_string();
    for (lang, paths) in fluent_resources {
        for (i, path) in paths.iter().enumerate() {
            files_tracker_str.push_str(&format!(
                "let s{}{} = include_bytes!(\"{}\");",
                lang.replace('-', "_"),
                i,
                &escape_string(path)
            ));
        }
    }
    if let Some(languages_file_path) = &languages_path {
        files_tracker_str.push_str(&format!(
            "let languages = include_bytes!(\"{}\");",
            &escape_string(languages_file_path)
        ));
    }
    if let Some(core_locales_file_path) = &core_locales_path {
        files_tracker_str.push_str(&format!(
            "let core_locales = include_bytes!(\"{}\");",
            &escape_string(core_locales_file_path)
        ));
    }
    files_tracker_str.push_str("};");
    files_tracker_str
        .parse::<proc_macro2::TokenStream>()
        .unwrap()
}

fn escape_string(s: &str) -> String {
    s.replace('"', "\\\"")
        // Windows path separator
        .replace('\\', "\\\\")
}
