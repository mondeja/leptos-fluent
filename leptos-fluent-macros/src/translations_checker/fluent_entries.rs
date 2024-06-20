use crate::{FluentFilePaths, FluentResources};
use std::collections::HashMap;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct FluentEntry {
    pub(crate) message_name: String,
    pub(crate) placeables: Vec<String>,
}

fn get_fluent_entries_from_resource(
    resource: &fluent_templates::fluent_bundle::FluentResource,
) -> Vec<FluentEntry> {
    let mut entries = Vec::new();
    for entry in resource.entries() {
        if let fluent_syntax::ast::Entry::Message(msg) = entry {
            if let Some(value) = &msg.value {
                let mut placeables = Vec::new();
                for element in &value.elements {
                    if let fluent_syntax::ast::PatternElement::Placeable {
                        expression: fluent_syntax::ast::Expression::Inline(
                            fluent_syntax::ast::InlineExpression::VariableReference {
                                id
                            }
                        )
                    } = element {
                        placeables.push(id.name.to_string());
                    }
                }
                entries.push(FluentEntry {
                    message_name: msg.id.name.to_string(),
                    placeables,
                });
            }
        }
    }
    entries
}

pub(crate) fn build_fluent_entries(
    fluent_resources: &FluentResources,
    fluent_file_paths: &FluentFilePaths,
    workspace_path: &str,
    core_locales_path: &Option<String>,
    core_locales_content: &Option<String>,
) -> (HashMap<String, Vec<FluentEntry>>, Vec<String>) {
    let mut fluent_entries: HashMap<String, Vec<FluentEntry>> = HashMap::new();
    let mut errors: Vec<String> = Vec::new();

    for (lang, resources) in fluent_resources {
        fluent_entries.insert(lang.to_owned(), vec![]);
        for (r, resource_str) in resources.iter().enumerate() {
            match fluent_templates::fluent_bundle::FluentResource::try_new(
                resource_str.to_owned(),
            ) {
                Ok(resource) => {
                    fluent_entries
                        .get_mut(lang)
                        .unwrap()
                        .extend(get_fluent_entries_from_resource(&resource));
                }
                Err((resource, errs)) => {
                    let file_path = fluent_file_paths
                        .get(lang)
                        .and_then(|paths| paths.get(r))
                        .unwrap();
                    let rel_file_path =
                        pathdiff::diff_paths(file_path, workspace_path)
                            .unwrap()
                            .as_path()
                            .to_str()
                            .unwrap()
                            .to_string();
                    errors.push(format!(
                        "Error{} parsing fluent resource in file {} for locale \"{}\":\n  + {}",
                        if errs.len() > 1 { "s" } else { "" },
                        rel_file_path,
                        lang,
                        errs
                            .iter()
                            .map(|e| {
                                let (line, col) = line_col_from_index_content(resource_str, e.pos.start);
                                format!("{e} (at line {line}, col {col})")
                            })
                            .collect::<Vec<String>>()
                            .join("\n   +")
                    ));
                    fluent_entries
                        .get_mut(lang)
                        .unwrap()
                        .extend(get_fluent_entries_from_resource(&resource));
                }
            }
        }
    }

    if let Some(resource_str) = &core_locales_content {
        match fluent_templates::fluent_bundle::FluentResource::try_new(
            resource_str.to_owned(),
        ) {
            Ok(resource) => {
                let langs =
                    fluent_entries.keys().cloned().collect::<Vec<String>>();
                for lang in langs {
                    fluent_entries
                        .get_mut(&lang)
                        .unwrap()
                        .extend(get_fluent_entries_from_resource(&resource));
                }
            }
            Err((resource, errs)) => {
                let rel_file_path = pathdiff::diff_paths(
                    core_locales_path.as_ref().unwrap(),
                    workspace_path,
                )
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .to_string();
                errors.push(format!(
                    "Error{} parsing core fluent resource in file {}:\n  + {}",
                    if errs.len() > 1 { "s" } else { "" },
                    rel_file_path,
                    errs.iter()
                        .map(|e| {
                            let (line, col) = line_col_from_index_content(
                                resource_str,
                                e.pos.start,
                            );
                            format!("{e} (at line {line}, col {col})")
                        })
                        .collect::<Vec<String>>()
                        .join("\n   +")
                ));
                let langs =
                    fluent_entries.keys().cloned().collect::<Vec<String>>();
                for lang in langs {
                    fluent_entries
                        .get_mut(&lang)
                        .unwrap()
                        .extend(get_fluent_entries_from_resource(&resource));
                }
            }
        }
    }

    (fluent_entries, errors)
}

fn line_col_from_index_content(content: &str, index: usize) -> (usize, usize) {
    let line = content[..index].chars().filter(|c| *c == '\n').count() + 1;
    let col = content[..index]
        .chars()
        .rev()
        .take_while(|c| *c != '\n')
        .count()
        + 1;
    (line, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let fluent_resources = HashMap::from([
            (
                "en-US".to_string(),
                vec!["foo = Bar\nhello = Hello { $name }\n".to_string()],
            ),
            (
                "es-ES".to_string(),
                vec!["foo = Bar\nhello = Hola { $name }\n".to_string()],
            ),
        ]);
        let fluent_file_paths = HashMap::from([
            (
                "en-US".to_string(),
                vec!["./locales/en-US/foo.ftl".to_string()],
            ),
            (
                "es-ES".to_string(),
                vec!["./locales/es-ES/foo.ftl".to_string()],
            ),
        ]);
        let workspace_path = "./";
        let (entries, errors) = build_fluent_entries(
            &fluent_resources,
            &fluent_file_paths,
            workspace_path,
            &None,
            &None,
        );
        assert!(errors.is_empty());
        assert_eq!(
            entries,
            HashMap::from([
                (
                    "en-US".to_string(),
                    vec![
                        FluentEntry {
                            message_name: "foo".to_string(),
                            placeables: vec![]
                        },
                        FluentEntry {
                            message_name: "hello".to_string(),
                            placeables: vec!["name".to_string()]
                        }
                    ]
                ),
                (
                    "es-ES".to_string(),
                    vec![
                        FluentEntry {
                            message_name: "foo".to_string(),
                            placeables: vec![]
                        },
                        FluentEntry {
                            message_name: "hello".to_string(),
                            placeables: vec!["name".to_string()]
                        }
                    ]
                )
            ])
        );
    }

    #[test]
    fn test_empty_resource() {
        let fluent_resources =
            HashMap::from([("en-US".to_string(), vec!["".to_string()])]);
        let fluent_file_paths = HashMap::from([(
            "en-US".to_string(),
            vec!["./locales/en-US/foo.ftl".to_string()],
        )]);
        let workspace_path = "./";
        let (entries, errors) = build_fluent_entries(
            &fluent_resources,
            &fluent_file_paths,
            workspace_path,
            &None,
            &None,
        );
        assert!(errors.is_empty());
        assert_eq!(entries, HashMap::from([("en-US".to_string(), vec![])]));
    }

    #[test]
    fn test_empty_message_name() {
        let fluent_resources = HashMap::from([(
            "en-US".to_string(),
            vec!["foo =\nbar = Baz".to_string()],
        )]);
        let fluent_file_paths = HashMap::from([(
            "en-US".to_string(),
            vec!["./locales/en-US/foo.ftl".to_string()],
        )]);
        let workspace_path = "./";
        let (entries, errors) = build_fluent_entries(
            &fluent_resources,
            &fluent_file_paths,
            workspace_path,
            &None,
            &None,
        );
        assert_eq!(
            errors,
            vec![concat!(
                "Error parsing fluent resource in file locales/en-US/foo.ftl",
                " for locale \"en-US\":\n",
                "  + Expected a message field for \"foo\" (at line 1, col 1)"
            )]
        );
        assert_eq!(
            entries,
            HashMap::from([(
                "en-US".to_string(),
                vec![FluentEntry {
                    message_name: "bar".to_string(),
                    placeables: vec![]
                }]
            )])
        );
    }

    #[test]
    fn test_empty_variable_name() {
        let fluent_resources = HashMap::from([(
            "en-US".to_string(),
            vec!["foo = Bar\nhello = Hello { $ }\n".to_string()],
        )]);
        let fluent_file_paths = HashMap::from([(
            "en-US".to_string(),
            vec!["./locales/en-US/foo.ftl".to_string()],
        )]);
        let workspace_path = "./";
        let (entries, errors) = build_fluent_entries(
            &fluent_resources,
            &fluent_file_paths,
            workspace_path,
            &None,
            &None,
        );
        assert_eq!(
            errors,
            vec![concat!(
                "Error parsing fluent resource in file locales/en-US/foo.ftl",
                " for locale \"en-US\":\n",
                "  + Expected one of \"a-zA-Z\" (at line 2, col 18)"
            )]
        );
        assert_eq!(
            entries,
            HashMap::from([(
                "en-US".to_string(),
                vec![FluentEntry {
                    message_name: "foo".to_string(),
                    placeables: vec![]
                }]
            )])
        );
    }
}
