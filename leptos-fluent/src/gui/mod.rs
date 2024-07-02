#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Graphical user interface for leptos-fluent.
//!
//! See [leptos-fluent] for more information.
//!
//! [leptos-fluent]: https://crates.io/crates/leptos-fluent
//!

mod editor;
mod projects;

use crate::{ctr, leptos_fluent};
use editor::Editor;
use fluent_templates::static_loader;
use leptos::*;
use leptos_meta::{provide_meta_context, Title};
use projects::{CurrentProjectContext, Project, Projects, ProjectsContext};

#[cfg(feature = "gui")]
#[macro_export]
#[doc(hidden)]
macro_rules! ctr {
    ($text_id:literal$(,)?) => {$crate::tr_impl($text_id)};
    ($text_id:literal, {$($key:literal => $value:expr),*$(,)?}$(,)?) => {{
        $crate::tr_with_args_impl($text_id, &{
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.into());
            )*
            map
        })
    }}
}

#[cfg(feature = "gui")]
#[macro_export]
#[doc(hidden)]
macro_rules! move_ctr {
    ($text_id:literal$(,)?) => {
        ::leptos::Signal::derive(move || $crate::ctr!($text_id))
    };
    ($text_id:literal, {$($key:literal => $value:expr),*$(,)?}$(,)?) => {
        ::leptos::Signal::derive(move || $crate::ctr!($text_id, {
            $(
                $key => $value,
            )*
        }))
    };
}

static_loader! {
    static TRANSLATIONS = {
        locales: "./src/gui/locales",
        fallback_language: "en",
    };
}

/// Main entry point for the GUI.
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
#[component]
pub fn LeptosFluentGui() -> impl IntoView {
    provide_meta_context();

    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "./src/gui/locales",
        leptos_fluent_prefix: crate,
        #[cfg(debug_assertions)]
        check_translations: "./src/gui/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; max-age=1209600",  // one week
        initial_language_from_cookie: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_cookie: true,
        initial_language_from_accept_language_header: true,
        initial_language_from_url_param: true,
        url_param: "lang",
        initial_language_from_url_param_to_cookie: true,
        #[cfg(feature = "system")]
        data_file_key: "leptos-fluent-gui",
    }};

    let projects_context = create_rw_signal::<Option<Vec<Project>>>(None);
    provide_context::<ProjectsContext>(projects_context);

    let current_project_context = create_rw_signal::<Option<Project>>(None);
    provide_context::<CurrentProjectContext>(current_project_context);

    view! {
        <Title text=|| ctr!("leptos-fluent-translator-gui")/>
        <style>{include_str!("./mod.css")}</style>
        <div id="app">
            <Projects/>
            <Main/>
        </div>
    }
}

#[component]
fn Main() -> impl IntoView {
    let get_current_project =
        || expect_context::<RwSignal<Option<Project>>>().get();

    view! {
        <lf-main>
            {move || match get_current_project() {
                Some(project) => view! { <ProjectView project /> }.into_view(),
                None => view! {}.into_view(),
            }}
        </lf-main>
    }
}

#[component]
fn ProjectView(project: Project) -> impl IntoView {
    let opened_files = create_rw_signal::<Vec<editor::OpenedFile>>(Vec::new());
    let active_file = create_rw_signal::<Option<editor::OpenedFile>>(None);

    view! {
        <h1>"📂 "{project.locales_path.clone()}</h1>
        <Editor project=project.clone() opened_files active_file/>
        <FilesSelector project=project.clone() opened_files active_file/>
    }
}

#[component]
fn FilesSelector(
    project: Project,
    opened_files: RwSignal<Vec<editor::OpenedFile>>,
    active_file: RwSignal<Option<editor::OpenedFile>>,
) -> impl IntoView {
    let render_file = |file: String| {
        let file_copy = file.clone();
        view! {
            <li
                on:click=move |_| {
                    let mut files = opened_files.get();
                    let new_file = editor::OpenedFile::new(file.clone());
                    if !files.contains(&new_file) {
                        files.push(new_file.clone());
                    }
                    opened_files.set(files);
                    active_file.set(Some(new_file));
                }
            >{file_to_relative_path(file_copy, project.workspace_path.clone(), project.locales_path.clone())}</li>
        }
    };

    let render_files = |lang: String, files: Vec<String>| {
        view! {
            <li>
                <h3>{lang}</h3>
                <ul>
                    {files.iter().map(|file| render_file(file.to_string())).collect::<Vec<_>>()}
                </ul>
            </li>
        }
    };

    view! {
        <div class="files-selector">
            <ul>
                {{
                    project.languages.iter().map(
                        |(lang, files)| render_files(lang.to_string(), files.clone())
                    )
                }.collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

pub(crate) fn file_to_relative_path(
    file: String,
    workspace_path: String,
    locales_path: String,
) -> String {
    let rel_pathbuf =
        pathdiff::diff_paths(file.as_str(), &workspace_path).unwrap();
    let rel_pathstr = rel_pathbuf.to_str().unwrap().to_string();
    rel_pathstr.split(&locales_path).collect::<Vec<_>>()[1]
        .trim_start_matches('/')
        .to_string()
}
