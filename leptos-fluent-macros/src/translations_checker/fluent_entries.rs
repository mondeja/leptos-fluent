use crate::FluentResources;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct FluentEntry {
    pub(crate) message_name: String,
    pub(crate) placeables: Vec<String>,
}

pub(crate) fn build_fluent_entries(
    fluent_resources: &FluentResources,
) -> HashMap<String, Vec<FluentEntry>> {
    let mut fluent_entries: HashMap<String, Vec<FluentEntry>> = HashMap::new();

    for (lang, (_, resources)) in fluent_resources.iter() {
        fluent_entries.insert(lang.to_owned(), vec![]);
        for resource_str in resources {
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
                                expression: fluent_syntax::ast::Expression::Inline(
                                    fluent_syntax::ast::InlineExpression::VariableReference {
                                        id
                                    }
                                )
                            } = element {
                                placeables.push(id.name.to_string());
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
