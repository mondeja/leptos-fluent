use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct FluentEntry {
    pub(crate) message_name: String,
    pub(crate) placeables: Vec<String>,
}

pub(crate) fn gather_fluent_entries_from_locales_path(
    workspace_path: &PathBuf,
    locales_path: &str,
) -> HashMap<String, Vec<FluentEntry>> {
    let mut fluent_entries: HashMap<String, Vec<FluentEntry>> = HashMap::new();

    let fluent_resources = build_resources(workspace_path.join(locales_path));
    for (lang, resources_paths) in fluent_resources.iter() {
        fluent_entries.insert(lang.to_owned(), vec![]);
        for path in resources_paths {
            let resource_str = std::fs::read_to_string(path).unwrap();
            let resource =
                fluent_templates::fluent_bundle::FluentResource::try_new(
                    resource_str.to_owned(),
                )
                .unwrap();
            for entry in resource.entries() {
                if let fluent_syntax::ast::Entry::Message(msg) = entry {
                    if let Some(value) = &msg.value {
                        let mut placeables = Vec::new();
                        for element in &value.elements {
                            if let fluent_syntax::ast::PatternElement::Placeable {
                                expression
                            } = element {
                                if let fluent_syntax::ast::Expression::Inline(inline_expr) = expression {
                                    if let fluent_syntax::ast::InlineExpression::VariableReference { id } = inline_expr {
                                        placeables.push(id.name.to_string());
                                    }
                                }
                            }
                        }
                        fluent_entries.get_mut(lang).unwrap().push(
                            FluentEntry {
                                message_name: msg.id.name.to_string(),
                                placeables,
                            },
                        );
                    }
                }
            }
        }
    }
    fluent_entries
}

/// Copied from `fluent_templates/macros` to ensure that the same implementation
/// is followed.
fn build_resources(
    dir: impl AsRef<std::path::Path>,
) -> HashMap<String, Vec<String>> {
    let mut all_resources = HashMap::new();
    for entry in std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|rs| rs.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
    {
        if let Some(lang) = entry.file_name().into_string().ok().filter(|l| {
            l.parse::<fluent_templates::LanguageIdentifier>().is_ok()
        }) {
            let resources = read_from_dir(entry.path());
            all_resources.insert(lang, resources);
        }
    }
    all_resources
}

/// Copied from `fluent_templates/macros` to ensure that the same implementation
/// is followed.
pub(crate) fn read_from_dir<P: AsRef<Path>>(path: P) -> Vec<String> {
    let (tx, rx) = flume::unbounded();

    ignore::WalkBuilder::new(path)
        .follow_links(true)
        .build_parallel()
        .run(|| {
            let tx = tx.clone();
            Box::new(move |result| {
                if let Ok(entry) = result {
                    if entry.file_type().as_ref().map_or(false, |e| e.is_file())
                        && entry
                            .path()
                            .extension()
                            .map_or(false, |e| e == "ftl")
                    {
                        tx.send(entry.path().display().to_string()).unwrap();
                    }
                }

                ignore::WalkState::Continue
            })
        });

    rx.drain().collect::<Vec<_>>()
}
