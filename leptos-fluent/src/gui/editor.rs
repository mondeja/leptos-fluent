use leptos::*;
use std::collections::HashMap;
use crate::gui::{Project, file_to_relative_path};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub(crate) struct OpenedFile {
    pub path: String,
}

impl OpenedFile {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[component]
pub(crate) fn Editor(
    project: Project,
    opened_files: RwSignal<Vec<OpenedFile>>,
    active_file: RwSignal<Option<OpenedFile>>,
) -> impl IntoView {
    let editor_files = create_memo(move |_| {
        project.languages.iter().flat_map(|(_, files)| files.clone()).collect::<Vec<_>>()
    });

    let editor_files_contents: RwSignal<HashMap<String, String>> = create_rw_signal(HashMap::new());

    create_effect(move |_| {
        spawn_local(async move {
            let mut content = HashMap::new();
            for file in editor_files.get() {
                let file_content = read_file(file.clone()).await.unwrap();
                content.insert(file.clone(), file_content);
            }
            editor_files_contents.set(content);
        });
    });

    view! {
        <style>{include_str!("./editor.css")}</style>
        <div id="editor">
            <ul id="files-tab">
                {move || opened_files.get().iter().map(|file| {
                    let file_copy = file.clone();
                    let file_copy2 = file.clone();
                    let is_active_file = active_file.get().as_ref().map(|active_file| active_file.path == file.path).unwrap_or(false);
                    view! {
                        <li
                            class:active=is_active_file
                            on:click=move |_| {
                                active_file.set(Some(file_copy.clone()));
                            }
                        >
                            <span>
                                {
                                    file_to_relative_path(
                                        file.path.clone(),
                                        project.workspace_path.clone(),
                                        project.locales_path.clone(),
                                    )
                                }
                            </span>
                            <CloseTabButton
                                on:click=move |_| {
                                    let mut files = opened_files.get();
                                    let file_path = file_copy2.path.clone();
                                    files.retain(|opened_file| opened_file.path != file_path);
                                    opened_files.set(files.clone());
                                    if active_file.get().as_ref().map(|active_file| active_file.path == file_path).unwrap_or(false) {
                                        active_file.set(None);
                                    }
                                }
                            />
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
            <div class="file">
                {move || {
                    ::leptos::logging::log!("active_file: {:?}", active_file.get());
                    match active_file.get() {
                        None => return view! {}.into_view(),
                        Some(active_file) => match opened_files.get().is_empty() {
                            true => return view! {}.into_view(),
                            false => {
                                for file_path in editor_files.get() {
                                    if active_file.path == file_path {
                                        let content = editor_files_contents.get();
                                        match content.get(&file_path) {
                                            None => return view! {}.into_view(),
                                            Some(file_content) => {
                                                return view! {
                                                    <FileAst file_content=file_content.clone()/>
                                                }.into_view();
                                            }
                                        };
                                    }
                                }
                                return view! {}.into_view();
                            }
                        },
                    };

                }}
            </div>
        </div>
    }
}

#[component]
fn CloseTabButton() -> impl IntoView {
    view! {
        <button class="close-tab">"×"</button>
    }
}

#[component]
pub fn FileAst(file_content: String) -> impl IntoView {
    let ast_ul = create_memo(move |_| {
        let messages= file_content.lines().map(|line| line.to_string()).collect::<Vec<_>>();

        view! {
            <ul class="messages">
                {messages.iter().map(|message| {
                    let message_ident = message.splitn(2, ' ').next().unwrap().trim().to_string();
                    let message_value = message.splitn(3, ' ').last().unwrap().trim().to_string();

                    let mut message_value_parts = vec![];
                    let mut inside_possible_variable = false;
                    let mut inside_variable = false;
                    let mut current_chars = vec![];
                    for character in message_value.chars() {
                        if character == '{' {
                            inside_possible_variable = true;
                        } else if inside_possible_variable && character == ' ' {

                        } else if inside_possible_variable && character == '$' {
                            let mut prev_chars = vec![];
                            loop {
                                let last_char = current_chars.pop().unwrap();
                                prev_chars.push(last_char);
                                if last_char == '{' {
                                    break;
                                }
                            }
                            prev_chars = prev_chars.iter().rev().cloned().collect::<Vec<_>>();

                            message_value_parts.push(current_chars.iter().collect::<String>());
                            inside_possible_variable = false;
                            inside_variable = true;
                            current_chars = prev_chars;
                        } else if inside_possible_variable {
                            inside_possible_variable = false;
                        } else if inside_variable && character == '}' {
                            current_chars.push(character);
                            message_value_parts.push(current_chars.iter().collect::<String>());
                            inside_variable = false;
                            current_chars = vec![];
                            continue;
                        } else {
                            inside_possible_variable = false;
                        }
                        current_chars.push(character);
                    }
                    message_value_parts.push(current_chars.iter().collect::<String>());

                    let mut li_html = format!(
                        concat!(
                            "<span class=\"ident\">{}</span>",
                            "<span class=\"punct\"> = </span>",
                        ),
                        message_ident,
                    );
                    for (i, part) in message_value_parts.iter().enumerate() {
                        if i % 2 == 0 {
                            li_html.push_str(&format!("<span class=\"value\">{}</span>", part));
                        } else {
                            li_html.push_str(&format!("<span class=\"variable\">{}</span>", part));
                        }
                    }

                    view! {
                        <li inner_html=li_html/>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        }.into_view()
    });

    view! {
        <div id="ast">
            {ast_ul}
        </div>
    }
}

#[server(ReadFile, "/api")]
pub async fn read_file(path: String) -> Result<String, ServerFnError> {
    if std::path::Path::new(&path).exists() {
        let content = std::fs::read_to_string(path).unwrap();
        Ok(content)
    } else {
        Err(ServerFnError::ServerError(format!("File \"{}\" not found", path)))
    }

}