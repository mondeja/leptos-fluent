use crate::fluent_entries::FluentEntries;
use crate::tr_macros::gather_tr_macro_defs_from_globstr;
use crate::{FluentFilePaths, FluentResources};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub(crate) fn run(
    globstr: &str,
    manifest_path: impl AsRef<Path>,
    fluent_entries: &FluentEntries,
    fluent_file_paths: &FluentFilePaths,
    fluent_resources: &FluentResources,
    errors: &mut Vec<String>,
) -> Vec<(String, Vec<String>)> {
    let ws_path = manifest_path.as_ref();
    let maybe_tr_macros = gather_tr_macro_defs_from_globstr(
        ws_path.join(globstr),
        errors,
        #[cfg(not(test))]
        ws_path,
    );
    if maybe_tr_macros.is_err() {
        // If we can't gather the macros, we just return an empty vector.
        // The Rust compiler will raise an error if the file has a syntax error.
        return Vec::new();
    }
    let tr_macros = maybe_tr_macros.unwrap();

    let mut missing_message_names_by_lang: HashMap<Rc<String>, Vec<String>> =
        HashMap::new();
    let tr_macros_message_names = tr_macros
        .iter()
        .map(|tr_macro| tr_macro.message_name.clone())
        .collect::<Vec<String>>();

    for (lang, entries) in fluent_entries {
        for tr_macro_message_name in &tr_macros_message_names {
            if !entries
                .iter()
                .any(|entry| entry.message_name == *tr_macro_message_name)
            {
                let missing_message_names = missing_message_names_by_lang
                    .entry(lang.clone())
                    .or_default();
                missing_message_names.push(tr_macro_message_name.clone());
            }
        }
    }

    let mut result = Vec::with_capacity(fluent_resources.len());
    for (lang, resource_strs) in fluent_resources {
        let language = Rc::try_unwrap(lang.into()).unwrap();
        // Save them in first resource file
        let resource_str: &mut String =
            &mut resource_strs.first().unwrap().clone();
        let file_path =
            fluent_file_paths.get(language).unwrap().first().unwrap();
        let rel_file_path = pathdiff::diff_paths(file_path, ws_path)
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_string();
        if !resource_str.ends_with('\n') && !resource_str.is_empty() {
            resource_str.push('\n');
        }
        if let Some(missing_message_names) =
            missing_message_names_by_lang.get(lang)
        {
            for missing_message_name in missing_message_names {
                resource_str.push_str(&format!(
                    "{missing_message_name} = Unknown localization {missing_message_name}\n",
                ));
            }
            std::fs::write(file_path, resource_str).unwrap();
            result.push((
                rel_file_path.to_string(),
                missing_message_names.clone(),
            ));
        }
    }

    result
}
