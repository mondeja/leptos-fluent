mod fluent_entries;
mod tr_macros;

use fluent_entries::{gather_fluent_entries_from_locales_path, FluentEntry};
use std::collections::HashMap;
use std::path::Path;
use tr_macros::{gather_tr_macro_defs_from_rs_files, TranslationMacro};

pub(crate) fn run(
    check_translations_globstr: &str,
    locales_path: &str,
    workspace_path: &Path,
) -> Result<Vec<String>, syn::Error> {
    let tr_macros: Vec<TranslationMacro> = gather_tr_macro_defs_from_rs_files(
        &workspace_path.join(check_translations_globstr),
        workspace_path,
    )?;

    let fluent_entries: HashMap<String, Vec<FluentEntry>> =
        gather_fluent_entries_from_locales_path(workspace_path, locales_path);

    let mut error_messages =
        check_tr_macros_against_fluent_entries(&tr_macros, &fluent_entries);
    error_messages.extend(check_fluent_entries_against_tr_macros(
        &tr_macros,
        &fluent_entries,
    ));
    // TODO: Include the core.ftl file in the check
    // TODO: Currently, the fluent-syntax parser does not offer a CST
    //       parser so we don't know the spans of the entries.
    //       See https://github.com/projectfluent/fluent-rs/issues/270
    Ok(error_messages)
}

fn format_macro_call(
    macro_name: &str,
    message_name: &str,
    has_placeables: bool,
) -> String {
    if has_placeables {
        return format!(r#"`{}!("{}", {{ ... }})`"#, macro_name, message_name);
    }
    format!(r#"`{}!("{}")`"#, macro_name, message_name)
}

fn check_tr_macros_against_fluent_entries(
    tr_macros: &Vec<TranslationMacro>,
    fluent_entries: &HashMap<String, Vec<FluentEntry>>,
) -> Vec<String> {
    let mut error_messages: Vec<String> = Vec::new();

    for tr_macro in tr_macros {
        for (lang, entries) in fluent_entries.iter() {
            // tr macro message must be defined for each language
            let mut message_name_found = false;
            for entry in entries {
                if tr_macro.message_name == entry.message_name {
                    message_name_found = true;

                    // Check if all variables in the tr macro are present in the fluent entry
                    for placeable in &tr_macro.placeables {
                        if !entry.placeables.contains(placeable) {
                            let file_path = {
                                #[cfg(not(test))]
                                {
                                    tr_macro.file_path.clone()
                                }

                                #[cfg(test)]
                                {
                                    "[test content]".to_string()
                                }
                            };

                            error_messages.push(format!(
                                concat!(
                                    r#"Variable "{}" defined at {} macro"#,
                                    r#" call in {} not found in message"#,
                                    r#" "{}" of locale "{}"."#,
                                ),
                                placeable,
                                format_macro_call(
                                    &tr_macro.name,
                                    &tr_macro.message_name,
                                    !tr_macro.placeables.is_empty(),
                                ),
                                file_path,
                                tr_macro.message_name,
                                lang,
                            ));
                        }
                    }

                    break;
                }
            }
            if !message_name_found {
                let file_path = {
                    #[cfg(not(test))]
                    {
                        tr_macro.file_path.clone()
                    }

                    #[cfg(test)]
                    {
                        "[test content]".to_string()
                    }
                };

                error_messages.push(format!(
                    concat!(
                        r#"Message "{}" defined at {} macro call in {}"#,
                        r#" not found in files for locale "{}"."#,
                    ),
                    tr_macro.message_name,
                    format_macro_call(
                        &tr_macro.name,
                        &tr_macro.message_name,
                        !tr_macro.placeables.is_empty(),
                    ),
                    file_path,
                    lang,
                ));
            }
        }
    }
    error_messages
}

fn check_fluent_entries_against_tr_macros(
    tr_macros: &Vec<TranslationMacro>,
    fluent_entries: &HashMap<String, Vec<FluentEntry>>,
) -> Vec<String> {
    let mut error_messages: Vec<String> = Vec::new();

    for (lang, entries) in fluent_entries.iter() {
        for entry in entries {
            // fluent entry message must be defined for each language
            let mut message_name_found = false;
            for tr_macro in tr_macros {
                if tr_macro.message_name == entry.message_name {
                    message_name_found = true;

                    // Check if all variables in the entry are present in the tr macro
                    for placeable in &entry.placeables {
                        if !tr_macro.placeables.contains(placeable) {
                            let file_path = {
                                #[cfg(not(test))]
                                {
                                    tr_macro.file_path.clone()
                                }

                                #[cfg(test)]
                                {
                                    "[test content]".to_string()
                                }
                            };

                            error_messages.push(
                                format!(
                                    concat!(
                                        r#"Variable "{}" defined in message "{}" of"#,
                                        r#" locale "{}" not found in arguments of"#,
                                        r#" {} macro call at file {}."#,
                                    ),
                                    placeable,
                                    entry.message_name,
                                    lang,
                                    format_macro_call(
                                        &tr_macro.name,
                                        &tr_macro.message_name,
                                        !tr_macro.placeables.is_empty(),
                                    ),
                                    file_path,
                                )
                            );
                        }
                    }

                    break;
                }
            }
            if !message_name_found {
                error_messages.push(format!(
                    concat!(
                        r#"Message "{}" of locale "{}" not found in any"#,
                        r#" `tr!` or `move_tr!` macro calls."#,
                    ),
                    entry.message_name, lang,
                ));
            }
        }
    }
    error_messages
}
