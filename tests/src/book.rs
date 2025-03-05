use super::{
    get_leptos_fluent_cargo_toml_version,
    get_leptos_fluent_macros_loader_fields, major_and_minor_version,
};
use walkdir::WalkDir;

static LEPTOS_FLUENT_REFERENCE_MD_CONTENT: &str =
    include_str!("../../book/src/leptos_fluent.md");

#[derive(Debug)]
struct BookSearchMatch {
    lineno: usize,
    line: String,
    file_path: String,
    value: String,
}

fn search_in_book(
    callback: impl Fn(usize, &str, &str, &mut Vec<BookSearchMatch>),
) -> Vec<BookSearchMatch> {
    let mut matches: Vec<BookSearchMatch> = Vec::new();

    for maybe_entry in WalkDir::new("../book/src") {
        let entry = maybe_entry.unwrap();
        let path = entry.path();
        if path.is_dir() || path.extension().unwrap() != "md" {
            continue;
        }

        let content = std::fs::read_to_string(path).unwrap();
        for (i, line) in content.lines().enumerate() {
            (callback)(i + 1, line, &path.display().to_string(), &mut matches);
        }
    }

    matches
}

fn search_all_leptos_fluent_versions_in_book() -> Vec<BookSearchMatch> {
    search_in_book(|lineno, line, file_path, matches| {
        if line.contains("leptos-fluent = ") {
            let value = line.split('"').nth(1).unwrap().to_string();
            let new_match = BookSearchMatch {
                line: line.into(),
                lineno,
                file_path: file_path.into(),
                value,
            };
            matches.push(new_match);
        }
    })
}

#[test]
fn leptos_fluent_versions_are_updated() {
    let leptos_fluent_version = get_leptos_fluent_cargo_toml_version();
    let leptos_fluent_major_and_minor_version =
        major_and_minor_version(&leptos_fluent_version);

    let leptos_fluent_versions = search_all_leptos_fluent_versions_in_book();
    for match_ in &leptos_fluent_versions {
        assert_eq!(
            match_.value,
            leptos_fluent_major_and_minor_version,
            concat!(
                "The version of leptos-fluent in the book is not updated",
                " at file {} line {}:\n{}\nExpected \"{}\" but found \"{}\""
            ),
            match_.file_path,
            match_.lineno,
            match_.line,
            leptos_fluent_major_and_minor_version,
            match_.value
        );
    }
}

#[test]
fn leptos_fluent_macro_parameters_sync() {
    let mut parameter_names: Vec<String> = Vec::new();

    let mut inside_parameters = false;
    for line in LEPTOS_FLUENT_REFERENCE_MD_CONTENT.lines() {
        if !inside_parameters {
            if line == "## Parameters" {
                inside_parameters = true;
            }
            continue;
        }

        if line.starts_with("### ") {
            let mut parameter_name = line.split('`').nth(1).unwrap();
            if parameter_name.contains(':') {
                parameter_name = parameter_name.split(':').next().unwrap();
            }
            parameter_names.push(parameter_name.into());
        }
    }

    let loader_fields = get_leptos_fluent_macros_loader_fields();

    for field in &loader_fields {
        assert!(
            parameter_names.contains(&field),
            concat!(
                "The parameter `{}` in `leptos_fluent!` macro is not",
                " documented in the book."
            ),
            field
        );
    }

    for parameter_name in parameter_names {
        assert!(
            loader_fields.contains(&parameter_name),
            concat!(
                "The parameter `{}` in the book is not used in",
                " the `leptos_fluent!` macro."
            ),
            parameter_name
        );
    }
}
