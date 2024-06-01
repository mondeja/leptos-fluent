use quote::ToTokens;
use std::path::PathBuf;
use syn::{
    visit::{self, Visit},
    Macro,
};

pub(crate) fn run(
    check_translations_globstr: &str,
    locales_path: &str,
    workspace_path: &PathBuf,
) -> Result<(), syn::Error> {
    let tr_macros = gather_tr_macro_defs_from_rs_files(
        &workspace_path.join(check_translations_globstr),
    )?;

    // TODO: include the core.ftl file in the check
    Ok(())
}

fn gather_tr_macro_defs_from_rs_files(
    check_translations_globstr: &PathBuf,
) -> Result<Vec<TranslationMacro>, syn::Error> {
    // TODO: handle errors
    let glob_pattern =
        glob::glob(check_translations_globstr.as_path().to_str().unwrap())
            .unwrap();

    let mut tr_macros = Vec::new();
    for path in glob_pattern.flatten() {
        tr_macros.extend(tr_macros_from_file_path(&path));
    }

    Ok(tr_macros)
}

#[derive(Debug)]
struct TranslationMacro {
    name: String,
    fluent_ident: String,
    args: Vec<String>,
    span: (proc_macro2::LineColumn, proc_macro2::LineColumn),

    file_path: Option<PathBuf>,
}

impl PartialEq for TranslationMacro {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.fluent_ident == other.fluent_ident
            && self.args == other.args
            && self.file_path == other.file_path
    }
}

struct TranslationsMacrosVisitor {
    pub(crate) tr_macros: Vec<TranslationMacro>,
    current_tr_macro: Option<String>,
    current_fluent_ident: Option<String>,
    current_args: Vec<String>,
    current_span_start: Option<proc_macro2::LineColumn>,
    current_span_end: Option<proc_macro2::LineColumn>,

    file_path: Option<PathBuf>,
}

impl TranslationsMacrosVisitor {
    fn new(file_path: Option<PathBuf>) -> Self {
        Self {
            tr_macros: Vec::new(),
            current_tr_macro: None,
            current_fluent_ident: None,
            current_args: Vec::new(),
            current_span_start: None,
            current_span_end: None,
            file_path,
        }
    }
}

fn tr_macros_from_file_path(file_path: &PathBuf) -> Vec<TranslationMacro> {
    let file_content = std::fs::read_to_string(file_path).unwrap();
    let ast = syn::parse_file(&file_content).unwrap();
    let mut visitor = TranslationsMacrosVisitor::new(Some(file_path.clone()));
    visitor.visit_file(&ast);
    visitor.tr_macros
}

fn tr_macros_from_file_content(file_content: &str) -> Vec<TranslationMacro> {
    let ast = syn::parse_file(file_content).unwrap();
    let mut visitor = TranslationsMacrosVisitor::new(None);
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
                    self.current_span_start = Some(ident.span().start());
                }
            } else if let proc_macro2::TokenTree::Group(group) = token {
                if let Some(ref tr_macro) = &self.current_tr_macro {
                    for tr_token in group.stream() {
                        if let proc_macro2::TokenTree::Literal(literal) =
                            tr_token
                        {
                            self.current_fluent_ident = Some(
                                literal
                                    .to_string()
                                    .strip_prefix('"')
                                    .unwrap()
                                    .strip_suffix('"')
                                    .unwrap()
                                    .to_string(),
                            );
                            self.current_span_end = Some(literal.span().end());
                        } else if let proc_macro2::TokenTree::Group(
                            args_group,
                        ) = tr_token
                        {
                            let mut after_comma_punct = true;
                            for arg_token in args_group.stream() {
                                if let proc_macro2::TokenTree::Literal(
                                    arg_literal,
                                ) = arg_token
                                {
                                    if after_comma_punct {
                                        self.current_args.push(
                                            arg_literal
                                                .to_string()
                                                .strip_prefix('"')
                                                .unwrap()
                                                .strip_suffix('"')
                                                .unwrap()
                                                .to_string(),
                                        );
                                        self.current_span_end =
                                            Some(arg_literal.span().end());
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
                    self.tr_macros.push(TranslationMacro {
                        name: tr_macro.clone(),
                        fluent_ident: self
                            .current_fluent_ident
                            .clone()
                            .unwrap(),
                        args: self.current_args.clone(),
                        file_path: self.file_path.clone(),
                        span: (
                            self.current_span_start.unwrap(),
                            self.current_span_end.unwrap(),
                        ),
                    });
                    self.current_tr_macro = None;
                    self.current_fluent_ident = None;
                    self.current_args = Vec::new();
                    self.current_span_start = None;
                    self.current_span_end = None;
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
    use super::*;
    use quote::quote;

    macro_rules! tr_macro {
        ($name:literal, $fluent_ident:literal, $args:expr) => {
            TranslationMacro {
                name: $name.to_string(),
                fluent_ident: $fluent_ident.to_string(),
                args: $args,
                span: (
                    // not important for tests, not implemented on `PartialEq`
                    proc_macro2::LineColumn { line: 0, column: 0 },
                    proc_macro2::LineColumn { line: 0, column: 0 },
                ),
                file_path: None,
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
}
