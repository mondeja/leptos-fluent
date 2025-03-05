use crate::{FluentFilePaths, FluentResources};
use fluent_syntax::ast::{
    CallArguments, Expression, InlineExpression, PatternElement,
};
use std::collections::HashMap;
use std::rc::Rc;

pub(in crate::translations_checker) type FluentEntries =
    HashMap<Rc<String>, Vec<FluentEntry>>;

#[cfg_attr(any(debug_assertions, feature = "tracing"), derive(Debug))]
#[derive(Clone, PartialEq)]
enum Placeable {
    String(String),
    MessageReference(String),
}

impl core::fmt::Display for Placeable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Placeable::String(s) => write!(f, "{s}"),
            Placeable::MessageReference(s) => write!(f, "{s}"),
        }
    }
}

impl From<String> for Placeable {
    fn from(s: String) -> Self {
        Placeable::String(s)
    }
}

#[cfg_attr(any(debug_assertions, feature = "tracing"), derive(Debug))]
#[derive(Clone, PartialEq)]
struct MaybeReferencedFluentEntry {
    message_name: String,
    placeables: Vec<Placeable>,
}

#[cfg_attr(any(debug_assertions, feature = "tracing"), derive(Debug))]
#[derive(Clone, PartialEq)]
pub(in crate::translations_checker) struct FluentEntry {
    pub(crate) message_name: String,
    pub(crate) placeables: Vec<String>,
}

fn get_fluent_entries_from_resource(
    resource: &fluent_templates::fluent_bundle::FluentResource,
) -> (Vec<FluentEntry>, Vec<String>) {
    let mut entries = Vec::new();
    let mut errors = Vec::new();

    for entry in resource.entries() {
        if let fluent_syntax::ast::Entry::Message(msg) = entry {
            if let Some(value) = &msg.value {
                let mut placeables = Vec::new();
                for element in &value.elements {
                    if let PatternElement::Placeable { expression } = element {
                        if let Expression::Inline(
                            InlineExpression::VariableReference { id },
                        ) = expression
                        {
                            placeables.push(id.name.to_string().into());
                        } else if let Expression::Inline(
                            InlineExpression::FunctionReference {
                                arguments: CallArguments { positional, .. },
                                ..
                            },
                        ) = expression
                        {
                            for arg in positional {
                                if let InlineExpression::VariableReference {
                                    id,
                                } = arg
                                {
                                    placeables.push(id.name.to_string().into());
                                }
                            }
                        } else if let Expression::Select {
                            selector: InlineExpression::VariableReference { id },
                            ..
                        } = expression
                        {
                            placeables.push(id.name.to_string().into());
                        } else if let Expression::Select {
                            selector:
                                InlineExpression::FunctionReference {
                                    arguments: CallArguments { positional, .. },
                                    ..
                                },
                            ..
                        } = expression
                        {
                            for arg in positional {
                                if let fluent_syntax::ast::InlineExpression::VariableReference {
                                    id
                                } = arg {
                                    placeables.push(id.name.to_string().into());
                                }
                            }
                        } else if let Expression::Inline(
                            InlineExpression::MessageReference { id, .. },
                        ) = expression
                        {
                            placeables.push(Placeable::MessageReference(
                                id.name.to_string(),
                            ));
                        }
                    }
                }
                entries.push(MaybeReferencedFluentEntry {
                    message_name: msg.id.name.to_string(),
                    placeables,
                });
            }
        }
    }

    let entries_clone = entries.clone();

    let mut non_referenced_entries = Vec::with_capacity(entries.len());
    for mut entry in entries {
        for placeable in entry.placeables.clone() {
            if let Placeable::MessageReference(id) = placeable {
                if let Some(entry_) =
                    entries_clone.iter().find(|e| e.message_name == *id)
                {
                    entry.placeables.extend(entry_.placeables.clone());
                } else {
                    errors.push(format!(
                        "Message reference \"{}\" of entry \"{}\" is not found",
                        id, entry.message_name
                    ));
                }
            }
        }
        entry
            .placeables
            .retain(|p| !matches!(p, Placeable::MessageReference(_)));
        non_referenced_entries.push(FluentEntry {
            message_name: entry.message_name.clone(),
            placeables: entry
                .placeables
                .iter()
                .map(|p| p.to_string())
                .collect(),
        });
    }

    (non_referenced_entries, errors)
}

pub(crate) fn build_fluent_entries(
    fluent_resources: &FluentResources,
    fluent_file_paths: &FluentFilePaths,
    workspace_path: &str,
    core_locales_path: &Option<String>,
    core_locales_content: &Option<String>,
) -> (FluentEntries, Vec<String>) {
    let mut fluent_entries: FluentEntries = HashMap::new();
    let mut errors: Vec<String> = Vec::new();

    for (lang, resources) in fluent_resources {
        fluent_entries.insert(Rc::clone(lang), vec![]);
        for resource_str in resources {
            match fluent_templates::fluent_bundle::FluentResource::try_new(
                resource_str.to_owned(),
            ) {
                Ok(resource) => {
                    let (entries, errs) =
                        get_fluent_entries_from_resource(&resource);
                    if !errs.is_empty() {
                        let index = resources
                            .iter()
                            .position(|r| r == resource_str)
                            .unwrap();
                        let file_path = fluent_file_paths
                            .get(lang)
                            .and_then(|paths| paths.get(index))
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
                            errs.join("\n   +")
                        ));
                    }
                    fluent_entries.get_mut(lang).unwrap().extend(entries);
                }
                Err((resource, errs)) => {
                    let index = resources
                        .iter()
                        .position(|r| r == resource_str)
                        .unwrap();
                    let file_path = fluent_file_paths
                        .get(lang)
                        .and_then(|paths| paths.get(index))
                        .unwrap();
                    let rel_file_path =
                        pathdiff::diff_paths(file_path, workspace_path)
                            .unwrap()
                            .as_path()
                            .to_str()
                            .unwrap()
                            .to_string();

                    let (entries, more_errors) =
                        get_fluent_entries_from_resource(&resource);
                    errors.extend(more_errors);
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
                    fluent_entries.get_mut(lang).unwrap().extend(entries);
                }
            }
        }
    }

    if let Some(resource_str) = &core_locales_content {
        match fluent_templates::fluent_bundle::FluentResource::try_new(
            resource_str.to_owned(),
        ) {
            Ok(resource) => {
                for resources in fluent_entries.values_mut() {
                    let (entries, errs) =
                        get_fluent_entries_from_resource(&resource);
                    if !errs.is_empty() {
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
                            errs.join("\n   +")
                        ));
                    }
                    resources.extend(entries);
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
                for resources in fluent_entries.values_mut() {
                    let (entries, errs) =
                        get_fluent_entries_from_resource(&resource);
                    resources.extend(entries);
                    errors.extend(errs);
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
    use std::borrow::Cow;

    fn cross_platform_path_repr(path: &str) -> Cow<'_, str> {
        #[cfg(target_os = "windows")]
        {
            path.replace('/', "\\").into()
        }
        #[cfg(not(target_os = "windows"))]
        {
            path.into()
        }
    }

    #[test]
    fn valid() {
        let fluent_resources = HashMap::from([
            (
                Rc::new("en-US".to_string()),
                vec!["foo = Bar\nhello = Hello { $name }\n".to_string()],
            ),
            (
                Rc::new("es-ES".to_string()),
                vec!["foo = Bar\nhello = Hola { $name }\n".to_string()],
            ),
        ]);
        let fluent_file_paths = HashMap::from([
            (
                Rc::new("en-US".to_string()),
                vec!["./locales/en-US/foo.ftl".to_string()],
            ),
            (
                Rc::new("es-ES".to_string()),
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
                    Rc::new("en-US".to_string()),
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
                    Rc::new("es-ES".to_string()),
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
    fn empty_resource() {
        let fluent_resources = HashMap::from([(
            Rc::new("en-US".to_string()),
            vec!["".to_string()],
        )]);
        let fluent_file_paths = HashMap::from([(
            Rc::new("en-US".to_string()),
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
        assert_eq!(
            entries,
            HashMap::from([(Rc::new("en-US".to_string()), vec![])])
        );
    }

    #[test]
    fn empty_message_name() {
        let fluent_resources = HashMap::from([(
            Rc::new("en-US".to_string()),
            vec!["foo =\nbar = Baz".to_string()],
        )]);
        let fluent_file_paths = HashMap::from([(
            Rc::new("en-US".to_string()),
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
            vec![
                format!(
                    concat!(
                        "Error parsing fluent resource in file {}",
                        " for locale \"en-US\":\n",
                        "  + Expected a message field for \"foo\" (at line 1, col 1)"
                    ),
                    cross_platform_path_repr("locales/en-US/foo.ftl"),
                )
            ]
        );
        assert_eq!(
            entries,
            HashMap::from([(
                Rc::new("en-US".to_string()),
                vec![FluentEntry {
                    message_name: "bar".to_string(),
                    placeables: vec![]
                }]
            )])
        );
    }

    #[test]
    fn empty_variable_name() {
        let fluent_resources = HashMap::from([(
            Rc::new("en-US".to_string()),
            vec!["foo = Bar\nhello = Hello { $ }\n".to_string()],
        )]);
        let fluent_file_paths = HashMap::from([(
            Rc::new("en-US".to_string()),
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
            vec![format!(
                concat!(
                    "Error parsing fluent resource in file {}",
                    " for locale \"en-US\":\n",
                    "  + Expected one of \"a-zA-Z\" (at line 2, col 18)"
                ),
                cross_platform_path_repr("locales/en-US/foo.ftl"),
            )]
        );
        assert_eq!(
            entries,
            HashMap::from([(
                Rc::new("en-US".to_string()),
                vec![FluentEntry {
                    message_name: "foo".to_string(),
                    placeables: vec![]
                }]
            )])
        );
    }

    #[test]
    fn fluent_functions() {
        let fluent_resources = HashMap::from([(
            Rc::new("en-US".to_string()),
            vec![
                r#"locale-date-format = { DATETIME($date, month: "long", year: "numeric", day: "numeric") }
log-time2 = Entry time: { DATETIME($date) }
emails2 = Number of unread emails { NUMBER($unreadEmails) }
"#.to_string()
            ],
        )]);
        let fluent_file_paths = HashMap::from([(
            Rc::new("en-US".to_string()),
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
        assert_eq!(
            entries,
            HashMap::from([(
                Rc::new("en-US".to_string()),
                vec![
                    FluentEntry {
                        message_name: "locale-date-format".to_string(),
                        placeables: vec!["date".to_string()]
                    },
                    FluentEntry {
                        message_name: "log-time2".to_string(),
                        placeables: vec!["date".to_string()]
                    },
                    FluentEntry {
                        message_name: "emails2".to_string(),
                        placeables: vec!["unreadEmails".to_string()]
                    }
                ]
            )])
        );
    }

    #[test]
    fn fluent_selectors() {
        let fluent_resources = HashMap::from([(
            Rc::new("en-US".to_string()),
            vec![r#"emails =
    { $unreadEmails ->
        [one] You have one unread email.
       *[other] You have { $unreadEmails } unread emails.
    }
your-score =
    { NUMBER($score, minimumFractionDigits: 1) ->
        [0.0]   You scored zero points. What happened?
       *[other] You scored { NUMBER($score, minimumFractionDigits: 1) } points.
    }
your-rank = { NUMBER($pos, type: "ordinal") ->
   [1] You finished first!
   [one] You finished {$pos}st
   [two] You finished {$pos}nd
   [few] You finished {$pos}rd
  *[other] You finished {$pos}th
}
"#
            .to_string()],
        )]);
        let fluent_file_paths = HashMap::from([(
            Rc::new("en-US".to_string()),
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
        assert_eq!(
            entries,
            HashMap::from([(
                Rc::new("en-US".to_string()),
                vec![
                    FluentEntry {
                        message_name: "emails".to_string(),
                        placeables: vec!["unreadEmails".to_string()]
                    },
                    FluentEntry {
                        message_name: "your-score".to_string(),
                        placeables: vec!["score".to_string()]
                    },
                    FluentEntry {
                        message_name: "your-rank".to_string(),
                        placeables: vec!["pos".to_string()]
                    }
                ]
            ),])
        );
    }

    #[test]
    fn fluent_message_references() {
        let fluent_resources = HashMap::from([(
            Rc::new("en-US".to_string()),
            vec![
                r#"units-unit-conversion = {$unit_value} = {$base_unit_value}
units-unit-conversion-continuation = {units-unit-conversion}, where
units-unit-conversion-continuation-double = {units-unit-conversion}, where {units-unit-conversion}
"#
                .to_string(),
            ],
        )]);

        let fluent_file_paths = HashMap::from([(
            Rc::new("en-US".to_string()),
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

        assert_eq!(
            entries,
            HashMap::from([(
                Rc::new("en-US".to_string()),
                vec![
                    FluentEntry {
                        message_name: "units-unit-conversion".to_string(),
                        placeables: vec![
                            "unit_value".to_string(),
                            "base_unit_value".to_string()
                        ]
                    },
                    FluentEntry {
                        message_name: "units-unit-conversion-continuation"
                            .to_string(),
                        placeables: vec![
                            "unit_value".to_string(),
                            "base_unit_value".to_string()
                        ]
                    },
                    FluentEntry {
                        message_name:
                            "units-unit-conversion-continuation-double"
                                .to_string(),
                        placeables: vec![
                            "unit_value".to_string(),
                            "base_unit_value".to_string(),
                            "unit_value".to_string(),
                            "base_unit_value".to_string(),
                        ]
                    }
                ]
            ),])
        );
    }

    #[test]
    fn fluent_message_reference_not_found() {
        let fluent_resources = HashMap::from([(
            Rc::new("en-US".to_string()),
            vec![r#"foo = {$bar}
bar = My {not-found} message reference
"#
            .to_string()],
        )]);

        let fluent_file_paths = HashMap::from([(
            Rc::new("en-US".to_string()),
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
        assert!(!errors.is_empty());

        assert_eq!(
            errors,
            vec![
                cross_platform_path_repr(concat!(
                    "Error parsing fluent resource in file",
                    " locales/en-US/foo.ftl for locale \"en-US\":",
                    "\n  + Message reference \"not-found\" of entry \"bar\" is not found",
                ).into())
            ]
        );

        assert_eq!(
            entries,
            HashMap::from([(
                Rc::new("en-US".to_string()),
                vec![
                    FluentEntry {
                        message_name: "foo".to_string(),
                        placeables: vec!["bar".to_string(),]
                    },
                    FluentEntry {
                        message_name: "bar".to_string(),
                        placeables: vec![]
                    },
                ]
            )])
        );
    }
}
