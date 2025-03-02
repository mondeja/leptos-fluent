use quote::ToTokens;
use std::path::Path;
use syn::visit::Visit;

pub(crate) fn gather_tr_macro_defs_from_rs_files(
    check_translations_globstr: &Path,
    #[cfg(not(test))] workspace_path: &Path,
) -> (Vec<TranslationMacro>, Vec<String>) {
    let mut errors = Vec::new();
    let glob_pattern = check_translations_globstr.to_string_lossy();

    match globwalk::glob(&glob_pattern) {
        Ok(paths) => {
            let mut tr_macros = Vec::new();
            #[cfg(not(test))]
            let workspace_path_str = workspace_path.to_string_lossy();
            for walker in paths {
                match walker {
                    Ok(entry) => {
                        let path = entry.path();
                        match tr_macros_from_file_path(
                            &path.to_string_lossy(),
                            #[cfg(not(test))]
                            &workspace_path_str,
                        ) {
                            Ok(new_tr_macros) => {
                                tr_macros.extend(new_tr_macros);
                            }
                            Err(message) => errors.push(message),
                        }
                    }
                    Err(error) => {
                        errors.push(format!("Error reading file: {error}"));
                    }
                }
            }

            (tr_macros, errors)
        }
        Err(error) => (
            Vec::new(),
            vec![format!(
                r#"Error parsing glob pattern "{}": {}"#,
                glob_pattern, error,
            )],
        ),
    }
}

fn tr_macros_from_file_path(
    file_path: &str,
    #[cfg(not(test))] workspace_path: &str,
) -> Result<Vec<TranslationMacro>, String> {
    if let Ok(file_content) = std::fs::read_to_string(file_path) {
        match syn::parse_file(&file_content) {
            Ok(ast) => {
                let mut visitor = TranslationsMacrosVisitor::new(
                    #[cfg(not(test))]
                    file_path,
                    #[cfg(not(test))]
                    workspace_path,
                );
                visitor.visit_file(&ast);
                if visitor.errors.is_empty() {
                    Ok(visitor.tr_macros)
                } else {
                    let error = visitor.errors.join("\n");
                    Err(format!("Error parsing file {file_path}\n{error}"))
                }
            }
            Err(error) => {
                Err(format!("Error parsing file {file_path}\n{error}"))
            }
        }
    } else {
        Err(format!("Error reading file: {file_path}"))
    }
}

#[cfg_attr(any(debug_assertions, feature = "tracing"), derive(Debug))]
pub(crate) struct TranslationMacro {
    pub(crate) name: String,
    pub(crate) message_name: String,
    pub(crate) placeables: Vec<String>,
    #[cfg(feature = "nightly")]
    pub(crate) start: proc_macro2::LineColumn,

    // On tests is easier to not use file paths
    #[cfg(not(test))]
    pub(crate) file_path: std::rc::Rc<String>,
}

impl PartialEq for TranslationMacro {
    fn eq(&self, other: &Self) -> bool {
        let equal = self.name == other.name
            && self.message_name == other.message_name
            && self.placeables == other.placeables;
        #[cfg(all(not(test), not(feature = "nightly")))]
        return equal && self.file_path == other.file_path;
        #[cfg(all(not(test), feature = "nightly"))]
        return equal
            && self.file_path == other.file_path
            && self.start == other.start;
        #[cfg(test)]
        return equal;
    }
}

pub(crate) struct TranslationsMacrosVisitor {
    pub(crate) tr_macros: Vec<TranslationMacro>,
    pub(crate) errors: Vec<String>,

    current_tr_macro: Option<String>,
    current_tr_macro_punct: Option<char>,
    current_message_name: Option<String>,
    current_placeables: Vec<String>,
    #[cfg(feature = "nightly")]
    current_tr_macro_start: Option<proc_macro2::LineColumn>,

    #[cfg(not(test))]
    file_path: std::rc::Rc<String>,
}

impl TranslationsMacrosVisitor {
    fn new(
        #[cfg(not(test))] file_path: &str,
        #[cfg(not(test))] workspace_path: &str,
    ) -> Self {
        #[cfg(not(test))]
        let rel_path = pathdiff::diff_paths(file_path, workspace_path)
            .unwrap()
            .as_path()
            .to_string_lossy()
            .to_string();
        Self {
            tr_macros: Vec::new(),
            current_tr_macro: None,
            current_tr_macro_punct: None,
            current_message_name: None,
            current_placeables: Vec::new(),
            errors: Vec::new(),
            #[cfg(not(test))]
            file_path: std::rc::Rc::new(rel_path),
            #[cfg(feature = "nightly")]
            current_tr_macro_start: None,
        }
    }
}

/// Convert a literal to a string, removing the quotes and the string type characters
fn value_from_literal(
    literal: &proc_macro2::Literal,
    location_macro_name: &str,
) -> Result<String, String> {
    let literal_str = literal.to_string();
    if literal_str.starts_with("r#") {
        Ok(literal_str
            .strip_prefix("r#\"")
            .expect("Raw string literal that does not starts with 'r#\"'")
            .strip_suffix("\"#")
            .expect("Raw string literal that does not ends with '\"#'")
            .into())
    } else if literal_str.starts_with("c\"") {
        Ok(literal_str
            .strip_prefix("c\"")
            .expect("C string literal that does not starts with 'c\"'")
            .strip_suffix('"')
            .expect("C string literal that does not ends with '\"'")
            .into())
    } else if literal_str.starts_with("cr#") {
        Ok(literal_str
            .strip_prefix("cr#\"")
            .expect("C raw string literal that does not starts with 'cr#\"'")
            .strip_suffix("\"#")
            .expect("C raw string literal that does not ends with '\"#'")
            .into())
    } else if literal_str.starts_with('"') {
        Ok(literal_str
            .strip_prefix('"')
            .expect("Literal that does not starts with '\"'")
            .strip_suffix('"')
            .expect("Literal that does not ends with '\"'")
            .into())
    } else {
        // TODO: Indicate the source file, line and column on nightly
        // https://doc.rust-lang.org/beta/proc_macro/struct.Span.html#method.source_file
        Err(format!(
            "Literal `{literal_str}` at `{location_macro_name}!` macro must be a string literal"
        ))
    }
}

impl TranslationsMacrosVisitor {
    fn visit_maybe_macro_tokens_stream(
        &mut self,
        tokens: &proc_macro2::TokenStream,
    ) {
        // Inside a macro group like `view!`
        for token in tokens.clone() {
            if let proc_macro2::TokenTree::Ident(ident) = token {
                let ident_str = ident.to_string();
                if ident_str == "move_tr" || ident_str == "tr" {
                    self.current_tr_macro = Some(ident.to_string());
                    #[cfg(feature = "nightly")]
                    {
                        self.current_tr_macro_start =
                            Some(ident.span().start());
                    }
                }
            } else if let proc_macro2::TokenTree::Punct(punct) = token {
                if self.current_tr_macro.is_some()
                    && self.current_tr_macro_punct.is_none()
                {
                    self.current_tr_macro_punct = Some(punct.as_char());
                }
            } else if let proc_macro2::TokenTree::Group(group) = token {
                if self.current_tr_macro.is_some() {
                    if let Some(ref tr_macro_punct) =
                        &self.current_tr_macro_punct
                    {
                        if *tr_macro_punct != '!' {
                            self.current_tr_macro = None;
                            self.current_tr_macro_punct = None;

                            #[cfg(feature = "nightly")]
                            {
                                self.current_tr_macro_start = None;
                            }
                            continue;
                        }
                    } else {
                        self.current_tr_macro = None;

                        #[cfg(feature = "nightly")]
                        {
                            self.current_tr_macro_start = None;
                        }
                        continue;
                    }

                    for tr_token in group.stream() {
                        if let proc_macro2::TokenTree::Literal(literal) =
                            tr_token
                        {
                            match value_from_literal(
                                &literal,
                                self.current_tr_macro.as_ref().unwrap(),
                            ) {
                                Ok(value) => {
                                    self.current_message_name = Some(value);
                                }
                                Err(error) => {
                                    if !self.errors.contains(&error) {
                                        self.errors.push(error);
                                    }
                                }
                            }
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
                                        match value_from_literal(
                                            &arg_literal,
                                            self.current_tr_macro
                                                .as_ref()
                                                .unwrap(),
                                        ) {
                                            Ok(value) => {
                                                self.current_placeables
                                                    .push(value);
                                            }
                                            Err(error) => {
                                                if !self.errors.contains(&error)
                                                {
                                                    self.errors.push(error);
                                                }

                                                // invalid placeable found, so
                                                // skip the current macro
                                                self.current_tr_macro = None;
                                                self.current_tr_macro_punct =
                                                    None;
                                                self.current_message_name =
                                                    None;
                                                self.current_placeables =
                                                    Vec::new();
                                                break;
                                            }
                                        }
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

                    if let Some(current_message_name) =
                        &self.current_message_name
                    {
                        let new_tr_macro = TranslationMacro {
                            name: self
                                .current_tr_macro
                                .as_ref()
                                .unwrap()
                                .to_owned(),
                            message_name: current_message_name.to_owned(),
                            placeables: self.current_placeables.to_owned(),
                            #[cfg(not(test))]
                            file_path: std::rc::Rc::clone(&self.file_path),
                            #[cfg(feature = "nightly")]
                            start: self.current_tr_macro_start.unwrap(),
                        };
                        if !self.tr_macros.contains(&new_tr_macro) {
                            self.tr_macros.push(new_tr_macro);
                        }
                        self.current_tr_macro = None;
                        self.current_tr_macro_punct = None;
                        self.current_message_name = None;
                        self.current_placeables = Vec::new();

                        #[cfg(feature = "nightly")]
                        {
                            self.current_tr_macro_start = None;
                        }
                    } else {
                        // if `current_message_name.is_none()` we are parsing
                        // `<tr>` tag from html, so we should ignore it
                        self.current_tr_macro = None;
                        self.current_tr_macro_punct = None;

                        #[cfg(feature = "nightly")]
                        {
                            self.current_tr_macro_start = None;
                        }
                    }
                } else {
                    self.visit_maybe_macro_tokens_stream(&group.stream());
                }
            }
        }
    }
}

impl<'ast> Visit<'ast> for TranslationsMacrosVisitor {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.visit_maybe_macro_tokens_stream(&node.to_token_stream());
        for token in node.tokens.clone() {
            if let proc_macro2::TokenTree::Group(group) = token {
                self.visit_maybe_macro_tokens_stream(&group.stream());
            }
        }
        syn::visit::visit_macro(self, node);
    }

    fn visit_stmt_macro(&mut self, node: &'ast syn::StmtMacro) {
        self.visit_maybe_macro_tokens_stream(&node.to_token_stream());
        syn::visit::visit_stmt_macro(self, node);
    }

    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        let stream = node
            .to_token_stream()
            .into_iter()
            .skip(2)
            .collect::<proc_macro2::TokenStream>();
        self.visit_maybe_macro_tokens_stream(&stream);

        syn::visit::visit_stmt(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::{TranslationMacro, TranslationsMacrosVisitor};
    use quote::quote;
    use syn::visit::Visit;

    fn visitor_from_file_content(
        file_content: &str,
    ) -> TranslationsMacrosVisitor {
        let ast = syn::parse_file(file_content).unwrap();
        let mut visitor = TranslationsMacrosVisitor::new();
        visitor.visit_file(&ast);
        visitor
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
    fn view() {
        let content = quote! {
            fn App() -> impl IntoView {
                view! {
                    <p>{move_tr!("select-a-language")}</p>
                    <p>{move_tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" })}</p>
                }
            }
        };
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("move_tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn closure() {
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
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
                tr_macro!("tr", "select-another-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "other-html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn signal_derive() {
        let content = quote! {
            fn App() -> impl IntoView {
                let description = Signal::derive(move || {
                    tr!("site-description", {
                        "n-icons" => get_number_of_icons!(),
                        "svg" => tr!("svg"),
                    })
                });
            }
        };
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!(
                    "tr",
                    "site-description",
                    vec!["n-icons".to_string(), "svg".to_string()]
                ),
                tr_macro!("tr", "svg", Vec::new()),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn stmt_macros() {
        let content = quote! {
            fn App() -> impl IntoView {
                // for completeness, this is not idiomatic
                tr!("select-a-language");
                tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
            }
        };
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn stmt() {
        let content = quote! {
            fn App() -> impl IntoView {
                let a = tr!("select-a-language");
                let b = tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
            }
        };
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn if_inside_view_macro() {
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
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("move_tr", "some-errors-happened", Vec::new()),
                tr_macro!("move_tr", "an-error-happened", Vec::new()),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn component_argument() {
        let content = quote! {
            fn App() -> impl IntoView {
                view! {
                    <ControlButtonIcon
                        title=move_tr!("light-color-scheme")
                        icon=ChSun
                        active=Signal::derive(move || color_scheme() == ColorMode::Light)
                        on:click=move |_| set_color_scheme(ColorMode::Light)
                    />
                    <ControlButtonIcon
                        title=move_tr!("dark-color-scheme")
                        icon=ChMoon
                        active=Signal::derive(move || color_scheme() == ColorMode::Dark)
                        on:click=move |_| set_color_scheme(ColorMode::Dark)
                    />
                }
            }
        };

        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("move_tr", "light-color-scheme", Vec::new()),
                tr_macro!("move_tr", "dark-color-scheme", Vec::new())
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn tr_html_tag() {
        let content = quote! {
            #[component]
            fn ThirdPartyExtensionsTableRow(
                extension: &'static ThirdPartyExtension,
            ) -> impl IntoView {
                view! {
                    <tr>
                        <td>
                            <a href=extension.url target="_blank">
                                {extension.name}
                            </a>
                        </td>
                    </tr>
                }
            }
        };

        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(visitor.tr_macros, Vec::new());
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn tr_non_string_literals_as_text_id() {
        let content = quote! {
            #[component]
            fn Foo() -> impl IntoView {
                view! {
                    <p>{tr!(56)}</p>
                    <p>{move_tr!("foo", {b'7' => "bar"})}</p>
                }
            }
        };

        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(visitor.tr_macros, Vec::new());
        assert_eq!(
            visitor.errors,
            vec![
                "Literal `56` at `tr!` macro must be a string literal",
                "Literal `b'7'` at `move_tr!` macro must be a string literal",
            ]
        );
    }

    #[test]
    fn raw_string_literals() {
        let content = quote! {
            fn App() -> impl IntoView {
                view! {
                    <p>{tr!(r#"select-a-language"#)}</p>
                    <p>{move_tr!(r#"html-tag-lang-is"#, { "foo" => r#"value1"#, "bar" => r#"value2"# })}</p>
                }
            }
        };
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn context_as_first_macro_parameters() {
        let content = quote! {
            fn App() -> impl IntoView {
                tr!(i18n, "select-a-language");
                tr!(i18n, "html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
            }
        };
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn at_return() {
        let content = quote! {
            pub fn foo() -> String {
                tr!("now")
            }

            pub fn bar() -> String {
                return move_tr!("after");
            }

            fn baz() -> String {
                (tr!("before"), move_tr!("tomorrow"))
            }
        };

        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!("tr", "now", Vec::new()),
                tr_macro!("move_tr", "after", Vec::new()),
                tr_macro!("tr", "before", Vec::new()),
                tr_macro!("move_tr", "tomorrow", Vec::new()),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }

    #[test]
    fn tr_in_move_tr_params() {
        let content = quote! {
            #[component]
            fn App() -> impl IntoView {
                let download_svg_msg =
                    move_tr!("download-filetype", {"filetype" => tr!("svg")});
                let download_colored_svg_msg =
                    move_tr!("download-filetype", {"filetype" => tr!("colored-svg")});
                let download_pdf_msg =
                    move_tr!("download-filetype", {"filetype" => tr!("pdf")});
                let download_jpg_msg =
                    move_tr!("download-filetype", {"filetype" => tr!("jpg")});
                let download_png_msg =
                    move_tr!("download-filetype", {"filetype" => tr!("png")});
            }
        };
        let visitor = visitor_from_file_content(&content.to_string());

        assert_eq!(
            visitor.tr_macros,
            vec![
                tr_macro!(
                    "move_tr",
                    "download-filetype",
                    vec!["filetype".to_string()]
                ),
                tr_macro!("tr", "svg", Vec::new()),
                tr_macro!("tr", "colored-svg", Vec::new()),
                tr_macro!("tr", "pdf", Vec::new()),
                tr_macro!("tr", "jpg", Vec::new()),
                tr_macro!("tr", "png", Vec::new()),
            ]
        );
        assert_eq!(visitor.errors, Vec::<String>::new());
    }
}
