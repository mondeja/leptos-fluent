use crate::{ctr, move_ctr};
use leptos::*;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub locales_path: String,
    pub workspace_path: String,
    pub languages: Vec<(String, Vec<String>)>,
}

pub(crate) type ProjectsContext = RwSignal<Option<Vec<Project>>>;
pub(crate) type CurrentProjectContext = RwSignal<Option<Project>>;

#[component]
pub(crate) fn Projects() -> impl IntoView {
    let projects_open = create_rw_signal(true);

    view! {
        <style>{include_str!("./projects.css")}</style>
        <div id="projects" class:closed=move || !projects_open.get()>
            <span
                class="title"
                on:click=move |_| if !projects_open.get_untracked() {
                    projects_open.set(true);
                }
                style=move || if !projects_open.get() { Some("cursor: pointer;") } else { None }
            >{move || {
                format!(
                    "📁 {}",
                    if !projects_open.get() { "".to_string() } else { ctr!("projects") }
                )
            }}</span>
            {move || {
                let projects_context = expect_context::<ProjectsContext>();
                let projects = projects_context.get().clone();
                if projects.is_none() {
                    spawn_local(async move {
                        let projects = get_projects().await.unwrap();
                        projects_context.set(Some(projects));
                    });

                    return view! {
                        <span class="loading">{move_ctr!("loading")}</span>
                    }.into_view();
                }

                let projects_iter = projects.unwrap().into_iter();
                let projects_view = projects_iter.map(|project| {
                    let locales_path = project.locales_path.clone();
                    let current_project_context = match use_context::<CurrentProjectContext>() {
                        Some(context) => context,
                        None => {
                            let context = create_rw_signal::<Option<Project>>(None);
                            provide_context::<CurrentProjectContext>(context.clone());
                            context
                        }
                    };
                    view! {
                        <li
                            on:click=move |_| {
                                if projects_open.get() {
                                    current_project_context.set(Some(project.clone()));
                                    projects_open.set(false);
                                } else {
                                    current_project_context.set(None);
                                    projects_open.set(true);
                                }
                            }
                        >{locales_path}</li>
                    }
                }).collect::<Vec<_>>();

                view! {
                    <ul>
                        {projects_view}
                    </ul>
                }.into_view()
            }}
        </div>
    }
}

#[server(GetProjects, "/api")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    // iterate over directories in workspace path
    let workspace_path = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ),
    );

    let projects = search_projects(&workspace_path, &workspace_path);

    Ok(projects)
}

#[cfg(not(feature = "hydrate"))]
fn search_projects(
    maybe_project_path: &std::path::PathBuf,
    workspace_path: &std::path::PathBuf,
) -> Vec<Project> {
    let mut projects = vec![];

    let mut n_dirs_with_fluent_files = 0;

    let mut project = Project {
        locales_path: pathdiff::diff_paths(maybe_project_path, workspace_path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        languages: vec![],
        workspace_path: workspace_path.to_str().unwrap().to_string(),
    };

    for entry in std::fs::read_dir(maybe_project_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let mut fluent_files = vec![];
            for maybe_locale_file in std::fs::read_dir(&path).unwrap() {
                let locale_file = maybe_locale_file.unwrap();
                let locale_file_path = locale_file.path();
                if locale_file_path.is_file() {
                    if let Some(extension) = locale_file_path.extension() {
                        if extension == "ftl" {
                            fluent_files.push(
                                locale_file_path.to_str().unwrap().to_string(),
                            );
                        }
                    }
                }
            }
            if !fluent_files.is_empty() {
                n_dirs_with_fluent_files += 1;
                let lang_id =
                    path.file_name().unwrap().to_str().unwrap().to_string();
                project.languages.push((lang_id, fluent_files));
            }

            projects.extend(search_projects(&path, workspace_path));
        }
    }

    if n_dirs_with_fluent_files > 1 {
        projects.push(project);
    }
    projects
}
