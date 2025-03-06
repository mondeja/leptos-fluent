use crate::fluent_entries::build_fluent_entries;
use crate::tr_macros::gather_tr_macro_defs_from_rs_files;
use crate::{FluentFilePaths, FluentResources};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub(crate) fn run(
    globstr: &str,
    workspace_path: &Path,
    fluent_resources: &FluentResources,
    fluent_file_paths: &FluentFilePaths,
    core_locales_path: &Option<String>,
    core_locales_content: &Option<String>,
) -> (Vec<(String, Vec<String>)>, Vec<String>) {
    let mut errors = Vec::new();

    let (tr_macros, tr_macros_errors) = gather_tr_macro_defs_from_rs_files(
        &workspace_path.join(globstr),
        #[cfg(not(test))]
        workspace_path,
    );

    #[cfg(feature = "tracing")]
    if !tr_macros_errors.is_empty() {
        tracing::warn!(
            "Errors while gathering tr macros: {:#?}",
            tr_macros_errors
        );
    } else {
        tracing::trace!("Gathered tr macros: {:#?}", tr_macros);
    }

    errors.extend(tr_macros_errors);

    let (fluent_entries, fluent_syntax_errors) = build_fluent_entries(
        fluent_resources,
        fluent_file_paths,
        workspace_path.to_str().unwrap(),
        core_locales_path,
        core_locales_content,
    );

    #[cfg(feature = "tracing")]
    if !&fluent_syntax_errors.is_empty() {
        tracing::warn!(
            "Errors while building fluent entries: {:#?}",
            &fluent_syntax_errors
        );
    } else {
        tracing::trace!("Built fluent entries: {:#?}", fluent_entries);
    }

    errors.extend(fluent_syntax_errors);

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
        let rel_file_path = pathdiff::diff_paths(file_path, workspace_path)
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_string();
        if !resource_str.ends_with('\n') {
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

    (result, errors)
}
