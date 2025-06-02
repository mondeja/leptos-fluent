use core::fmt::{self, Debug, Formatter};
use quote::ToTokens;
use std::path::Path;
use syn::visit::Visit;

pub(crate) fn gather_tr_macro_defs_from_rs_files(
    globstr: impl AsRef<Path>,
    #[cfg(not(test))] workspace_path: impl AsRef<Path>,
) -> (Vec<TranslationMacro>, Vec<String>) {
    let mut errors = Vec::new();
    let mut tr_macros = Vec::new();

    let glob_pattern = globstr.as_ref().to_string_lossy();

    match globwalk::glob(&glob_pattern) {
        Ok(paths) => {
            for walker in paths {
                match walker {
                    Ok(entry) => {
                        let path = entry.path();
                        tr_macros_from_file_path(
                            &mut tr_macros,
                            &mut errors,
                            &path.to_string_lossy(),
                            #[cfg(not(test))]
                            &workspace_path.as_ref().to_string_lossy(),
                        );
                    }
                    Err(error) => {
                        errors.push(format!("Error reading file: {error}"));
                    }
                }
            }
        }
        Err(error) => {
            errors.push(format!(
                "Error reading glob pattern \"{glob_pattern}\": {error}"
            ));
        }
    }

    (tr_macros, errors)
}

fn tr_macros_from_file_path(
    tr_macros: &mut Vec<TranslationMacro>,
    errors: &mut Vec<String>,
    file_path: &str,
    #[cfg(not(test))] workspace_path: &str,
) {
    if let Ok(file_content) = std::fs::read_to_string(file_path) {
        match syn::parse_file(&file_content) {
            Ok(ast) => {
                let mut visitor = TranslationsMacrosVisitor::new(
                    tr_macros,
                    errors,
                    #[cfg(not(test))]
                    file_path,
                    #[cfg(not(test))]
                    workspace_path,
                );
                visitor.visit_file(&ast);
            }
            Err(error) => {
                errors
                    .push(format!("Error parsing file {file_path}\n  {error}"));
            }
        }
    } else {
        errors.push(format!("Error reading file: {file_path}"));
    }
}

#[cfg_attr(test, derive(Clone))]
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

impl Debug for TranslationMacro {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TranslationMacro {{ name: {name:?}, message_name: {message_name:?}, placeables: {placeables:?} }}",
            name = self.name,
            message_name = self.message_name,
            placeables = self.placeables,
        )
    }
}

/// Current messages parsed by `TranslationsMacrosVisitor`.
#[derive(Debug)]
enum CurrentMessages {
    // Single message with its possible placeables.
    //
    // ```ignore
    // tr!("text-id", { "placeable1" => "value1", "placeable2" => "value2" })
    // ```
    Single(String, Vec<String>),
    // Conditional message with its possible placeables.
    //
    // ```ignore
    // tr!(
    //     if foo { "text-id-foo" } else { "text-id-bar" },
    //     {
    //         "placeable1" => "value1",
    //         "placeable2" => "value2",
    //     }
    // )
    // ```
    Conditional(Vec<(String, (String, Vec<String>))>),
}

pub(crate) struct TranslationsMacrosVisitor<'a> {
    pub(crate) tr_macros: &'a mut Vec<TranslationMacro>,
    pub(crate) errors: &'a mut Vec<String>,

    // State to gather translation macros
    current_tr_macro: Option<String>,
    current_tr_macro_punct: Option<char>,
    current_messages: Option<CurrentMessages>,
    #[cfg(feature = "nightly")]
    current_tr_macro_start: Option<proc_macro2::LineColumn>,

    // State to check validity of `use` statements
    current_use_path_is_leptos_fluent: bool,

    #[cfg(not(test))]
    file_path: std::rc::Rc<String>,
}

fn parse_if_elseif_else(
    visitor: &mut TranslationsMacrosVisitor<'_>,
    group: &proc_macro2::Group,
    n_parsed_tokens: usize,
) -> usize {
    let stream = group
        .stream()
        .into_iter()
        .skip(n_parsed_tokens)
        .collect::<proc_macro2::TokenStream>();

    let mut n_parsed_tokens = 0;
    let mut condition = String::new();
    for token in stream {
        n_parsed_tokens += 1;
        if let proc_macro2::TokenTree::Ident(ident) = token {
            let ident_str = ident.to_string();
            if ident_str == "if" {
                continue;
            } else if ident_str == "else" {
                condition.push_str("else ");
            } else {
                condition.push_str(&ident.to_string());
            }
        } else if let proc_macro2::TokenTree::Group(group_token_tree) = token {
            let group_token_tree_stream = group_token_tree.stream();
            if group_token_tree_stream.into_iter().count() != 1 {
                continue;
            }
            if let proc_macro2::TokenTree::Literal(literal) =
                group_token_tree.stream().into_iter().next().unwrap()
            {
                match value_from_literal_str(
                    &literal.to_string(),
                    visitor.current_tr_macro.as_ref().unwrap(),
                ) {
                    Ok(value) => {
                        match visitor.current_messages.as_mut() {
                            Some(CurrentMessages::Conditional(
                                ref mut messages,
                            )) => {
                                let message = messages.iter_mut().find(
                                    |(message_condition, _)| {
                                        message_condition == &condition
                                    },
                                );
                                if let Some((_, (_, placeables))) = message {
                                    placeables.push(value.to_owned());
                                } else {
                                    messages.push((
                                        condition.clone(),
                                        (value.to_owned(), Vec::new()),
                                    ));
                                }
                            }
                            Some(CurrentMessages::Single(_, _)) => {}
                            None => {
                                visitor.current_messages =
                                    Some(CurrentMessages::Conditional({
                                        vec![(
                                            condition.clone(),
                                            (value.to_owned(), Vec::new()),
                                        )]
                                    }));
                            }
                        };
                        condition = String::new();
                    }
                    Err(error) => {
                        if !visitor.errors.contains(&error) {
                            visitor.errors.push(error);
                        }

                        // invalid placeable found, so
                        // skip the current macro
                        visitor.current_tr_macro = None;
                        visitor.current_tr_macro_punct = None;
                        visitor.current_messages = None;
                        break;
                    }
                }
            }
        } else if let proc_macro2::TokenTree::Punct(punct) = token {
            if punct.as_char() == ',' {
                break;
            } else if punct.as_char() == '#' {
                n_parsed_tokens += 1;
                break;
            }
        }
    }

    n_parsed_tokens
}

impl<'a> TranslationsMacrosVisitor<'a> {
    fn new(
        tr_macros: &'a mut Vec<TranslationMacro>,
        errors: &'a mut Vec<String>,
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
            tr_macros,
            errors,
            current_tr_macro: None,
            current_tr_macro_punct: None,
            current_messages: None,
            current_use_path_is_leptos_fluent: false,
            #[cfg(not(test))]
            file_path: std::rc::Rc::new(rel_path),
            #[cfg(feature = "nightly")]
            current_tr_macro_start: None,
        }
    }
}

impl TranslationsMacrosVisitor<'_> {
    fn visit_maybe_macro_tokens_stream(
        &mut self,
        tokens: &proc_macro2::TokenStream,
    ) {
        // println!("\ntokens: {:#?}\n----------------------", tokens);

        // Inside a macro group like `view!`
        for token in tokens.clone() {
            if let proc_macro2::TokenTree::Ident(ref ident) = token {
                let ident_str = ident.to_string();
                if ident_str == "move_tr" || ident_str == "tr" {
                    self.current_tr_macro = Some(ident.to_string());
                    #[cfg(feature = "nightly")]
                    {
                        self.current_tr_macro_start =
                            Some(ident.span().start());
                    }
                }
            } else if let proc_macro2::TokenTree::Punct(ref punct) = token {
                if self.current_tr_macro.is_some()
                    && self.current_tr_macro_punct.is_none()
                {
                    self.current_tr_macro_punct = Some(punct.as_char());
                }
            } else if let proc_macro2::TokenTree::Group(ref group) = token {
                if self.current_tr_macro.is_none() {
                    self.visit_maybe_macro_tokens_stream(&group.stream());
                    continue;
                }
                if let Some(ref tr_macro_punct) = &self.current_tr_macro_punct {
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

                let mut n_parsed_tokens = 0;

                let group_first_token =
                    group.stream().into_iter().next().unwrap();
                if let proc_macro2::TokenTree::Literal(literal) =
                    group_first_token
                {
                    // we are in text identifier
                    match value_from_literal_str(
                        &literal.to_string(),
                        self.current_tr_macro.as_ref().unwrap(),
                    ) {
                        Ok(value) => {
                            self.current_messages =
                                Some(CurrentMessages::Single(
                                    value.to_owned(),
                                    Vec::new(),
                                ));
                            n_parsed_tokens += 2;
                        }
                        Err(error) => {
                            if !self.errors.contains(&error) {
                                self.errors.push(error);
                            }
                        }
                    }
                } else if let proc_macro2::TokenTree::Ident(first_ident) =
                    group_first_token
                {
                    // probably is conditional message
                    if first_ident == "if" {
                        // is conditional message
                        n_parsed_tokens +=
                            parse_if_elseif_else(self, group, n_parsed_tokens);
                    } else {
                        n_parsed_tokens += 2;
                        if group.stream().into_iter().count() < 3 {
                            continue;
                        }

                        let group_second_token =
                            group.stream().into_iter().nth(2).unwrap();
                        if let proc_macro2::TokenTree::Ident(second_ident) =
                            group_second_token
                        {
                            // probably is conditional message
                            if second_ident == "if" {
                                // is conditional message
                                n_parsed_tokens += parse_if_elseif_else(
                                    self,
                                    group,
                                    n_parsed_tokens,
                                );
                            }
                        } else if let proc_macro2::TokenTree::Literal(literal) =
                            group_second_token
                        {
                            n_parsed_tokens += 2;
                            // we are in text identifier
                            match value_from_literal_str(
                                &literal.to_string(),
                                self.current_tr_macro.as_ref().unwrap(),
                            ) {
                                Ok(value) => {
                                    self.current_messages =
                                        Some(CurrentMessages::Single(
                                            value.to_owned(),
                                            Vec::new(),
                                        ));
                                }
                                Err(error) => {
                                    if !self.errors.contains(&error) {
                                        self.errors.push(error);
                                    }
                                }
                            }
                        } else if let proc_macro2::TokenTree::Punct(punct) =
                            group_second_token
                        {
                            if punct.as_char() == '#' {
                                // inside an attribute
                                n_parsed_tokens += 2;
                            }
                            let next_token = group
                                .stream()
                                .into_iter()
                                .nth(n_parsed_tokens)
                                .unwrap();
                            if let proc_macro2::TokenTree::Ident(ident) =
                                next_token
                            {
                                if ident == "if" {
                                    // is conditional message
                                    n_parsed_tokens += parse_if_elseif_else(
                                        self,
                                        group,
                                        n_parsed_tokens,
                                    );
                                }
                            }
                        } else {
                            n_parsed_tokens += 1;
                        }
                    }
                } else if let proc_macro2::TokenTree::Punct(punct) =
                    group_first_token
                {
                    if punct.as_char() == '#' {
                        // inside an attribute
                        n_parsed_tokens += 2;
                    }
                    let next_token = group
                        .stream()
                        .into_iter()
                        .nth(n_parsed_tokens)
                        .unwrap();
                    if let proc_macro2::TokenTree::Ident(ident) = next_token {
                        if ident == "if" {
                            // is conditional message
                            n_parsed_tokens += parse_if_elseif_else(
                                self,
                                group,
                                n_parsed_tokens,
                            );
                        }
                    }
                }

                if n_parsed_tokens == 0 {
                    self.current_tr_macro = None;
                    self.current_tr_macro_punct = None;

                    #[cfg(feature = "nightly")]
                    {
                        self.current_tr_macro_start = None;
                    }
                    self.current_messages = None;
                    continue;
                }

                let group_n_tokens = group.stream().into_iter().count();
                if n_parsed_tokens <= group_n_tokens {
                    let args_token =
                        group.stream().into_iter().nth(n_parsed_tokens);
                    if let Some(proc_macro2::TokenTree::Punct(punct)) =
                        args_token
                    {
                        if punct.as_char() == ',' {
                            n_parsed_tokens += 1;
                        } else if punct.as_char() == '#' {
                            n_parsed_tokens += 2;
                        }
                    }

                    let group_n_tokens = group.stream().into_iter().count();
                    if n_parsed_tokens <= group_n_tokens {
                        let args_token =
                            group.stream().into_iter().nth(n_parsed_tokens);
                        if let Some(proc_macro2::TokenTree::Group(args_group)) =
                            args_token
                        {
                            if args_group.stream().into_iter().count() >= 4 {
                                let mut after_comma = true;
                                for token in args_group.stream() {
                                    if let proc_macro2::TokenTree::Literal(
                                        literal,
                                    ) = token
                                    {
                                        if !after_comma {
                                            continue;
                                        }

                                        match value_from_literal_str(
                                            &literal.to_string(),
                                            self.current_tr_macro.as_ref().unwrap(),
                                        ) {
                                            Ok(value) => {
                                                match self.current_messages.as_mut() {
                                                Some(CurrentMessages::Single(
                                                    _,
                                                    ref mut placeables,
                                                )) => {
                                                    placeables.push(value.to_owned());
                                                }
                                                Some(
                                                    CurrentMessages::Conditional(
                                                        ref mut messages,
                                                    ),
                                                ) => {
                                                    for (
                                                        _condition,
                                                        (_text_id, placeables),
                                                    ) in messages.iter_mut()
                                                    {
                                                        placeables
                                                            .push(value.to_owned());
                                                    }
                                                }
                                                None => {}
                                            }
                                            }
                                            Err(error) => {
                                                if !self.errors.contains(&error) {
                                                    self.errors.push(error);
                                                }

                                                // invalid placeable found, so
                                                // skip the current macro
                                                self.current_tr_macro = None;
                                                self.current_tr_macro_punct = None;
                                                self.current_messages = None;
                                                break;
                                            }
                                        }
                                    } else if let proc_macro2::TokenTree::Punct(
                                        punct,
                                    ) = token
                                    {
                                        after_comma = punct.as_char() == ',';
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(current_messages) = &self.current_messages {
                    let new_tr_macros = match current_messages {
                        CurrentMessages::Single(message_name, placables) => {
                            vec![TranslationMacro {
                                name: self
                                    .current_tr_macro
                                    .as_ref()
                                    .unwrap()
                                    .to_owned(),
                                message_name: message_name.to_owned(),
                                placeables: placables.to_owned(),
                                #[cfg(not(test))]
                                file_path: std::rc::Rc::clone(&self.file_path),
                                #[cfg(feature = "nightly")]
                                start: self.current_tr_macro_start.unwrap(),
                            }]
                        }
                        CurrentMessages::Conditional(messages) => messages
                            .iter()
                            .map(|(_condition, (message_name, placeables))| {
                                TranslationMacro {
                                    name: self
                                        .current_tr_macro
                                        .as_ref()
                                        .unwrap()
                                        .to_owned(),
                                    message_name: message_name.to_owned(),
                                    placeables: placeables.to_owned(),
                                    #[cfg(not(test))]
                                    file_path: std::rc::Rc::clone(
                                        &self.file_path,
                                    ),
                                    #[cfg(feature = "nightly")]
                                    start: self.current_tr_macro_start.unwrap(),
                                }
                            })
                            .collect::<Vec<TranslationMacro>>(),
                    };
                    for new_tr_macro in new_tr_macros {
                        if !self.tr_macros.contains(&new_tr_macro) {
                            self.tr_macros.push(new_tr_macro);
                        }
                    }
                    self.current_tr_macro = None;
                    self.current_tr_macro_punct = None;
                    self.current_messages = None;

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
            }
        }
    }
}

impl<'ast> Visit<'ast> for TranslationsMacrosVisitor<'_> {
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

    fn visit_use_rename(&mut self, node: &'ast syn::UseRename) {
        if self.current_use_path_is_leptos_fluent {
            let ident = node.ident.to_string();
            if ident == "tr" || ident == "move_tr" {
                let rename_ident = node.rename.to_string();
                self.errors.push(format!(
                    "Importing `{ident}` as `{rename_ident}` is not allowed because breaks leptos-fluent's compile-time checking of translations."
                ));
            }
        } else {
            let ident = node.ident.to_string();
            let rename_ident = node.rename.to_string();
            if ident == "leptos_fluent" {
                self.errors.push(format!(
                    "Importing `leptos-fluent` as `{rename_ident}` is not allowed because breaks leptos-fluent's compile-time checking of translations."
                ));
            } else if rename_ident == "tr" || rename_ident == "move_tr" {
                self.errors.push(format!(
                    "Importing as `{rename_ident}` is not allowed because breaks leptos-fluent's compile-time checking of translations."
                ));
            }
        }

        syn::visit::visit_use_rename(self, node);
    }

    fn visit_use_path(&mut self, node: &'ast syn::UsePath) {
        let ident = node.ident.to_string();
        self.current_use_path_is_leptos_fluent = ident == "leptos_fluent";
        syn::visit::visit_use_path(self, node);
    }

    fn visit_use_name(&mut self, node: &'ast syn::UseName) {
        let ident = node.ident.to_string();
        if !self.current_use_path_is_leptos_fluent
            && (ident == "tr" || ident == "move_tr")
        {
            self.errors.push(format!(
                "Importing `{ident}` is not allowed because breaks leptos-fluent's compile-time checking of translations."
            ));
        }

        syn::visit::visit_use_name(self, node);
    }
}

/// Convert a literal to a string, removing the quotes and the string type characters
fn value_from_literal_str<'a>(
    literal_str: &'a str,
    location_macro_name: &str,
) -> Result<&'a str, String> {
    if literal_str.starts_with("r#\"") {
        Ok(literal_str[3..literal_str.len() - 2].into())
    } else if literal_str.starts_with("c\"") {
        Ok(literal_str[2..literal_str.len() - 1].into())
    } else if literal_str.starts_with("cr#\"") {
        Ok(literal_str[4..literal_str.len() - 2].into())
    } else if literal_str.starts_with('"') {
        Ok(literal_str[1..literal_str.len() - 1].into())
    } else {
        // TODO: Indicate the source file, line and column on nightly
        // https://doc.rust-lang.org/beta/proc_macro/struct.Span.html#method.source_file
        Err(format!(
            "Literal `{literal_str}` at `{location_macro_name}!` macro must be a string literal"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{TranslationMacro, TranslationsMacrosVisitor};
    use quote::quote;
    use syn::visit::Visit;

    fn parse_file_content(
        file_content: &str,
    ) -> (Vec<TranslationMacro>, Vec<String>) {
        let maybe_ast = syn::parse_file(file_content);
        if let Err(error) = maybe_ast {
            panic!("Error parsing `quote!` content: {}", error);
        }
        let ast = maybe_ast.unwrap();
        let mut tr_macros = Vec::new();
        let mut errors = Vec::new();
        let mut visitor =
            TranslationsMacrosVisitor::new(&mut tr_macros, &mut errors);
        visitor.visit_file(&ast);
        (visitor.tr_macros.clone(), visitor.errors.clone())
    }

    macro_rules! tr_macro {
        ($name:literal, $message_name:literal, $placeables:expr) => {
            TranslationMacro {
                name: $name.to_string(),
                message_name: $message_name.to_string(),
                placeables: $placeables,
                #[cfg(feature = "nightly")]
                start: proc_macro2::LineColumn { line: 0, column: 0 },
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
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
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
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
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
        assert_eq!(errors, Vec::<String>::new());
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
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!(
                    "tr",
                    "site-description",
                    vec!["n-icons".to_string(), "svg".to_string()]
                ),
                tr_macro!("tr", "svg", Vec::new()),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
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
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
    }

    #[test]
    fn stmt() {
        let content = quote! {
            fn App() -> impl IntoView {
                let a = tr!("select-a-language");
                let b = tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
            }
        };
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
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
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "some-errors-happened", Vec::new()),
                tr_macro!("move_tr", "an-error-happened", Vec::new()),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
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

        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "light-color-scheme", Vec::new()),
                tr_macro!("move_tr", "dark-color-scheme", Vec::new())
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
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

        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(tr_macros, Vec::new());
        assert_eq!(errors, Vec::<String>::new());
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

        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(tr_macros, Vec::new());
        assert_eq!(
            errors,
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
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
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

        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "now", Vec::new()),
                tr_macro!("move_tr", "after", Vec::new()),
                tr_macro!("tr", "before", Vec::new()),
                tr_macro!("move_tr", "tomorrow", Vec::new()),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
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
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
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
        assert_eq!(errors, Vec::<String>::new());
    }

    #[test]
    fn i18n_as_first_tr_macros_param() {
        let content = quote! {
            fn App() -> impl IntoView {
                tr!(i18n, "select-a-language");
                move_tr!(i18n, "html-tag-lang-is", { "foo" => "value1", "bar" => "value2" });
            }
        };
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(errors, Vec::<String>::new());
    }

    #[test]
    fn use_rename_from_tr_macros() {
        // Forbid importing `tr` and `move_tr` renaming with `use ... as`
        let content = quote! {
            use leptos_fluent::tr as tr_alias_1;
            use leptos_fluent::move_tr as move_tr_alias_1;
            use leptos_fluent::{tr as tr_alias_2, move_tr as move_tr_alias_2};

            fn App() -> impl IntoView {
                view! {
                    <p>{move_tr!("select-a-language")}</p>
                    <p>{move_tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" })}</p>
                }
            }
        };
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(
            errors,
            vec![
                concat!(
                    "Importing `tr` as `tr_alias_1` is not allowed because breaks",
                    " leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `move_tr` as `move_tr_alias_1` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `tr` as `tr_alias_2` is not allowed because breaks",
                    " leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `move_tr` as `move_tr_alias_2` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
            ]
        );
    }

    #[test]
    fn use_rename_to_tr_macros() {
        // Forbid importing renaming with `use ... as` `tr` and `move_tr`
        let content = quote! {
            use foobar::foo as move_tr;
            use barbaz as tr;
            use tr;
            use move_tr;
            use foo::{tr, move_tr};

            fn App() -> impl IntoView {
                view! {
                    <p>{move_tr!("select-a-language")}</p>
                    <p>{move_tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" })}</p>
                }
            }
        };
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(
            errors,
            vec![
                concat!(
                    "Importing as `move_tr` is not allowed because breaks",
                    " leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing as `tr` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `tr` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `move_tr` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `tr` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `move_tr` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
            ]
        );
    }

    #[test]
    fn use_rename_leptos_fluent() {
        // Forbid importing `leptos-fluent` renaming with `use ... as`
        let content = quote! {
            use leptos_fluent as lf;
            use whatever::leptos_fluent as lf2;

            fn App() -> impl IntoView {
                view! {
                    <p>{move_tr!("select-a-language")}</p>
                    <p>{move_tr!("html-tag-lang-is", { "foo" => "value1", "bar" => "value2" })}</p>
                }
            }
        };
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "select-a-language", Vec::new()),
                tr_macro!(
                    "move_tr",
                    "html-tag-lang-is",
                    vec!["foo".to_string(), "bar".to_string()]
                ),
            ]
        );
        assert_eq!(
            errors,
            vec![
                concat!(
                    "Importing `leptos-fluent` as `lf` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
                concat!(
                    "Importing `leptos-fluent` as `lf2` is not allowed because",
                    " breaks leptos-fluent's compile-time checking of translations.",
                ),
            ]
        );
    }

    #[test]
    fn tr_macros_pass() {
        let content = quote! {
            use leptos_fluent::{expect_i18n, move_tr, tr};

            fn App() -> impl IntoView {
                let i18n = expect_i18n();
                _ = move_tr!("foo");

                let (foo, bar) = (false, true);

                _ = tr!(if foo {"foo1"} else {"bar1"});
                _ = tr!(if foo {"foo1"} else {"bar1"});
                _ = move_tr!(if foo {"foo2"} else {"bar2"});
                // i18n
                _ = tr!(i18n, if foo {"foo3"} else {"bar3"});
                _ = move_tr!(i18n, if foo {"foo4"} else {"bar4"});
                // args
                _ = tr!(
                    if foo {
                        "foo5"
                    } else {
                        "bar5"
                    },
                    { "foo6" => "value1", "bar6" => "value2" }
                );
                _ = move_tr!(
                    if foo {
                        "foo7"
                    } else {
                        "bar7"
                    },
                    { "foo8" => "value2", "bar8" => "value3" }
                );
                // i18n + args
                _ = tr!(
                    i18n,
                    if foo {
                        "foo9"
                    } else {
                        "bar9"
                    },
                    { "foo10" => "value3", "bar10" => "value4" }
                );
                _ = move_tr!(
                    i18n,
                    if foo {
                        "foo11"
                    } else {
                        "bar11"
                    },
                    { "foo12" => "value4", "bar12" => "value5" }
                );

                // else if
                _ = tr!(if foo {"foo13"} else if bar {"bar13"} else {"baz13"});
                _ = move_tr!(if foo {"foo14"} else if bar {"bar14"} else {"baz14"});
                // else if + args
                _ = tr!(
                    if foo {
                        "foo15"
                    } else if bar {
                        "bar15"
                    } else {
                        "baz15"
                    },
                    { "foo16" => "value5", "bar16" => "value6" }
                );
                _ = move_tr!(
                    if foo {
                        "foo17"
                    } else if bar {
                        "bar17"
                    } else {
                        "baz17"
                    },
                    { "foo18" => "value6", "bar18" => "value7" }
                );

                // tt as condition
                _ = tr!(
                    if {signal.get() || fake_fn()} {"foo18"} else {"bar18"}
                );
                _ = move_tr!(
                    if {signal.get() || fake_fn()} {"foo19"} else {"bar19"}
                );
                _ = tr!(
                    if {other_signal.get() || other_fake_fn()} {"foo20"} else {"bar20"}
                );
                _ = move_tr!(
                    if {other_signal.get() || other_fake_fn()} {"foo21"} else {"bar21"}
                );
            }
        };
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("move_tr", "foo", Vec::new()),
                tr_macro!("tr", "foo1", Vec::new()),
                tr_macro!("tr", "bar1", Vec::new()),
                tr_macro!("move_tr", "foo2", Vec::new()),
                tr_macro!("move_tr", "bar2", Vec::new()),
                tr_macro!("tr", "foo3", Vec::new()),
                tr_macro!("tr", "bar3", Vec::new()),
                tr_macro!("move_tr", "foo4", Vec::new()),
                tr_macro!("move_tr", "bar4", Vec::new()),
                tr_macro!(
                    "tr",
                    "foo5",
                    vec!["foo6".to_string(), "bar6".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "bar5",
                    vec!["foo6".to_string(), "bar6".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "foo7",
                    vec!["foo8".to_string(), "bar8".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "bar7",
                    vec!["foo8".to_string(), "bar8".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "foo9",
                    vec!["foo10".to_string(), "bar10".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "bar9",
                    vec!["foo10".to_string(), "bar10".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "foo11",
                    vec!["foo12".to_string(), "bar12".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "bar11",
                    vec!["foo12".to_string(), "bar12".to_string()]
                ),
                tr_macro!("tr", "foo13", Vec::new()),
                tr_macro!("tr", "bar13", Vec::new()),
                tr_macro!("tr", "baz13", Vec::new()),
                tr_macro!("move_tr", "foo14", Vec::new()),
                tr_macro!("move_tr", "bar14", Vec::new()),
                tr_macro!("move_tr", "baz14", Vec::new()),
                tr_macro!(
                    "tr",
                    "foo15",
                    vec!["foo16".to_string(), "bar16".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "bar15",
                    vec!["foo16".to_string(), "bar16".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "baz15",
                    vec!["foo16".to_string(), "bar16".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "foo17",
                    vec!["foo18".to_string(), "bar18".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "bar17",
                    vec!["foo18".to_string(), "bar18".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "baz17",
                    vec!["foo18".to_string(), "bar18".to_string()]
                ),
                tr_macro!("tr", "foo18", Vec::new()),
                tr_macro!("tr", "bar18", Vec::new()),
                tr_macro!("move_tr", "foo19", Vec::new()),
                tr_macro!("move_tr", "bar19", Vec::new()),
                tr_macro!("tr", "foo20", Vec::new()),
                tr_macro!("tr", "bar20", Vec::new()),
                tr_macro!("move_tr", "foo21", Vec::new()),
                tr_macro!("move_tr", "bar21", Vec::new()),
            ]
        );
        assert!(errors.is_empty());
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn tr_macros_attributes() {
        let content = quote! {
            use leptos_fluent::{tr, move_tr, expect_i18n};

            #[component]
            fn App() -> impl IntoView {
                let i18n = expect_i18n();
                let (foo, bar) = (true, false);

                _ = tr!(
                    #[allow(unused_braces)]
                    if foo {"foo1"} else {"bar1"}
                );

                _ = move_tr!(
                    #[allow(unused_braces)]
                    if foo {"foo2"} else {"bar2"}
                );

                _ = tr!(
                    #[allow(unused_braces, unused_parens)]
                    if foo {
                        "foo3"
                    } else {
                        "bar3"
                    },
                    { "foo4" => "value1", "bar4" => "value2" }
                );
                _ = move_tr!(
                    if foo {"foo5"} else {"bar5"}
                    #[allow(unused_braces, unused_parens)]
                    { "foo6" => {"value1"}, "bar6" => ("value2") }
                );
                _ = tr!(
                    "foo7",
                    #[allow(unused_braces, unused_parens)]
                    { "foo8" => {"value1"}, "bar8" => ("value2") }
                );

                _ = move_tr!(
                    i18n,
                    "foo9",
                    #[allow(unused_braces, unused_parens)]
                    { "foo10" => {"value1"}, "bar10" => ("value2") }
                );

                _ = tr!(
                    i18n,
                    #[allow(unused_braces, unused_parens)]
                    if foo {"foo11"} else {"bar11"}
                    #[allow(unused_braces, unused_parens)]
                    { "foo12" => {"value1"}, "bar12" => ("value2") }
                );

                _ = move_tr!(
                    i18n,
                    #[allow(unused_braces, unused_parens)]
                    if foo {"foo13"} else if bar {"bar13"} else {"baz13"}
                    #[allow(unused_braces, unused_parens)]
                    { "foo14" => {"value1"}, "bar14" => ("value2") }
                );

                // is not valid to use attrs on literal and the macros
                // parser can't parse the next syntax:
                _ = tr!(
                    #[allow(unused_braces, unused_parens)]
                    "foo15",
                    #[allow(unused_braces, unused_parens)]
                    { "foo16" => {"value1"}, "bar16" => ("value2") }
                );

                view! {
                    <p></p>
                }
            }
        };
        let (tr_macros, errors) = parse_file_content(&content.to_string());

        assert_eq!(
            tr_macros,
            vec![
                tr_macro!("tr", "foo1", Vec::new()),
                tr_macro!("tr", "bar1", Vec::new()),
                tr_macro!("move_tr", "foo2", Vec::new()),
                tr_macro!("move_tr", "bar2", Vec::new()),
                tr_macro!(
                    "tr",
                    "foo3",
                    vec!["foo4".to_string(), "bar4".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "bar3",
                    vec!["foo4".to_string(), "bar4".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "foo5",
                    vec!["foo6".to_string(), "bar6".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "bar5",
                    vec!["foo6".to_string(), "bar6".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "foo7",
                    vec!["foo8".to_string(), "bar8".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "foo9",
                    vec!["foo10".to_string(), "bar10".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "foo11",
                    vec!["foo12".to_string(), "bar12".to_string()]
                ),
                tr_macro!(
                    "tr",
                    "bar11",
                    vec!["foo12".to_string(), "bar12".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "foo13",
                    vec!["foo14".to_string(), "bar14".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "bar13",
                    vec!["foo14".to_string(), "bar14".to_string()]
                ),
                tr_macro!(
                    "move_tr",
                    "baz13",
                    vec!["foo14".to_string(), "bar14".to_string()]
                ),
                /* This is not catched: tr_macro!(
                    "tr",
                    "foo15",
                    vec!["foo16".to_string(), "bar16".to_string()]
                ),*/
            ],
        );
        assert!(errors.is_empty());
    }
}
