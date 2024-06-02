use pathdiff::diff_paths;
use quote::ToTokens;
use std::path::PathBuf;
use syn::{
    visit::{self, Visit},
    Macro,
};

pub(crate) fn gather_tr_macro_defs_from_rs_files(
    check_translations_globstr: &PathBuf,
    workspace_path: &PathBuf,
) -> Result<Vec<TranslationMacro>, syn::Error> {
    // TODO: handle errors
    let glob_pattern =
        glob::glob(check_translations_globstr.as_path().to_str().unwrap())
            .unwrap();

    let mut tr_macros = Vec::new();
    for path in glob_pattern.flatten() {
        tr_macros.extend(tr_macros_from_file_path(
            &path,
            &workspace_path.to_str().unwrap(),
        ));
    }

    Ok(tr_macros)
}

#[derive(Debug)]
pub(crate) struct TranslationMacro {
    pub(crate) name: String,
    pub(crate) message_name: String,
    pub(crate) placeables: Vec<String>,

    // On tests is easier to not use file paths
    #[cfg(not(test))]
    pub(crate) file_path: String,
}

impl PartialEq for TranslationMacro {
    fn eq(&self, other: &Self) -> bool {
        let equal = self.name == other.name
            && self.message_name == other.message_name
            && self.placeables == other.placeables;
        #[cfg(not(test))]
        return equal && self.file_path == other.file_path;
        #[cfg(test)]
        return equal;
    }
}

pub(crate) struct TranslationsMacrosVisitor {
    pub(crate) tr_macros: Vec<TranslationMacro>,
    current_tr_macro: Option<String>,
    current_message_name: Option<String>,
    current_placeables: Vec<String>,

    #[cfg(not(test))]
    file_path: PathBuf,
    #[cfg(not(test))]
    workspace_path: String,
}

impl TranslationsMacrosVisitor {
    fn new(
        #[cfg(not(test))] file_path: PathBuf,
        #[cfg(not(test))] workspace_path: &str,
    ) -> Self {
        Self {
            tr_macros: Vec::new(),
            current_tr_macro: None,
            current_message_name: None,
            current_placeables: Vec::new(),
            #[cfg(not(test))]
            file_path,
            #[cfg(not(test))]
            workspace_path: workspace_path.to_string(),
        }
    }
}

fn tr_macros_from_file_path(
    file_path: &PathBuf,
    workspace_path: &str,
) -> Vec<TranslationMacro> {
    let file_content = std::fs::read_to_string(file_path).unwrap();
    let ast = syn::parse_file(&file_content).unwrap();
    let mut visitor = TranslationsMacrosVisitor::new(
        #[cfg(not(test))]
        file_path.clone(),
        #[cfg(not(test))]
        workspace_path,
    );
    visitor.visit_file(&ast);
    visitor.tr_macros
}

impl<'ast> TranslationsMacrosVisitor {
    fn visit_maybe_macro_tokens_stream(
        &mut self,
        tokens: &'ast proc_macro2::TokenStream,
    ) {
        // Inside a macro group like `view!`
        for token in tokens.clone().into_iter() {
            if let proc_macro2::TokenTree::Ident(ident) = token {
                let ident_str = ident.to_string();
                if ident_str == "move_tr" || ident_str == "tr" {
                    self.current_tr_macro = Some(ident.to_string());
                }
            } else if let proc_macro2::TokenTree::Group(group) = token {
                if let Some(ref tr_macro) = &self.current_tr_macro {
                    for tr_token in group.stream() {
                        if let proc_macro2::TokenTree::Literal(literal) =
                            tr_token
                        {
                            self.current_message_name = Some(
                                literal
                                    .to_string()
                                    .strip_prefix('"')
                                    .unwrap()
                                    .strip_suffix('"')
                                    .unwrap()
                                    .to_string(),
                            );
                        } else if let proc_macro2::TokenTree::Group(
                            placeables_group,
                        ) = tr_token
                        {
                            let mut after_comma_punct = true;
                            for arg_token in placeables_group.stream() {
                                if let proc_macro2::TokenTree::Literal(
                                    arg_literal,
                                ) = arg_token
                                {
                                    if after_comma_punct {
                                        self.current_placeables.push(
                                            arg_literal
                                                .to_string()
                                                .strip_prefix('"')
                                                .unwrap()
                                                .strip_suffix('"')
                                                .unwrap()
                                                .to_string(),
                                        );
                                        after_comma_punct = false;
                                    }
                                } else if let proc_macro2::TokenTree::Punct(
                                    punct,
                                ) = arg_token
                                {
                                    if punct.as_char() == ',' {
                                        after_comma_punct = true;
                                    }
                                }
                            }
                        }
                    }

                    let new_tr_macro = TranslationMacro {
                        name: tr_macro.clone(),
                        message_name: self
                            .current_message_name
                            .clone()
                            .unwrap(),
                        placeables: self.current_placeables.clone(),
                        #[cfg(not(test))]
                        file_path: diff_paths(
                            self.file_path
                                .as_path()
                                .to_str()
                                .unwrap()
                                .to_string(),
                            &self.workspace_path,
                        )
                        .unwrap()
                        .as_path()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    };
                    // TODO: this is expensive because we're executing
                    // it recursively for each group
                    if !self.tr_macros.contains(&new_tr_macro) {
                        self.tr_macros.push(new_tr_macro);
                    }
                    self.current_tr_macro = None;
                    self.current_message_name = None;
                    self.current_placeables = Vec::new();
                    break;
                } else {
                    self.visit_maybe_macro_tokens_stream(&group.stream());
                }
            }
        }
    }
}

impl<'ast> Visit<'ast> for TranslationsMacrosVisitor {
    fn visit_macro(&mut self, node: &'ast Macro) {
        for token in node.tokens.clone() {
            if let proc_macro2::TokenTree::Group(group) = token {
                self.visit_maybe_macro_tokens_stream(&group.stream());
            }
        }

        visit::visit_macro(self, node);
    }

    fn visit_stmt_macro(&mut self, node: &'ast syn::StmtMacro) {
        let stream = node.to_token_stream();
        self.visit_maybe_macro_tokens_stream(&stream);

        visit::visit_stmt_macro(self, node);
    }

    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        let stream = node
            .to_token_stream()
            .into_iter()
            .skip(2)
            .collect::<proc_macro2::TokenStream>();
        self.visit_maybe_macro_tokens_stream(&stream);

        visit::visit_stmt(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::{TranslationMacro, TranslationsMacrosVisitor};
    use quote::quote;
    use syn::visit::{self, Visit};

    fn tr_macros_from_file_content(
        file_content: &str,
    ) -> Vec<TranslationMacro> {
        let ast = syn::parse_file(file_content).unwrap();
        let mut visitor = TranslationsMacrosVisitor::new();
        visitor.visit_file(&ast);
        visitor.tr_macros
    }

    macro_rules! tr_macro {
        ($name:literal, $message_name:literal, $placeables:expr) => {
            TranslationMacro {
                name: $name.to_string(),
                message_name: $message_name.to_string(),
                placeables: $placeables,
            }
        };
    }

    #[test]
    fn tr_macros_from_view() {
        let content = quote! {
            fn App() -> impl IntoView {
                view! {
                    <p>{move_tr!("select-a-language")}</p>
                    <p>{move_tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" })}</p>
                }
            }
        };
        let tr_macros = tr_macros_from_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string(),]
                ),
            ]
        );
    }

    #[test]
    fn tr_macros_from_closure() {
        let content = quote! {
            fn App() -> impl IntoView {
                let closure_a = move || tr!("select-a-language");
                let closure_b = move || {
                    tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
                };
                let closure_c = || tr!("select-another-language");
                let closure_d = || {
                    tr!("other-html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
                };
            }
        };
        let tr_macros = tr_macros_from_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string(),]
                ),
                tr_macro!("tr", "select-another-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "other-html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string(),]
                ),
            ]
        );
    }

    #[test]
    fn tr_macros_from_stmt_macros() {
        let content = quote! {
            fn App() -> impl IntoView {
                // for completeness, this is not idiomatic
                tr!("select-a-language");
                tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
            }
        };
        let tr_macros = tr_macros_from_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string(),]
                ),
            ]
        );
    }

    #[test]
    fn tr_macros_from_stmt() {
        let content = quote! {
            fn App() -> impl IntoView {
                let a = tr!("select-a-language");
                let b = tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
            }
        };
        let tr_macros = tr_macros_from_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string(),]
                ),
            ]
        );
    }

    #[test]
    fn tr_macros_from_if_inside_view_macro() {
        let content = quote! {
            fn App() -> impl IntoView {
                view! {
                    <h1>
                        {
                            if errors.len() > 1 {
                                move_tr!("some-errors-happened")
                            } else {
                                move_tr!("an-error-happened")
                            }
                        }
                    </h1>
                }
            }
        };
        let tr_macros = tr_macros_from_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "some-errors-happened", Vec::new()),
                tr_macro!("move_tr", "an-error-happened", Vec::new()),
            ]
        );
    }
}
