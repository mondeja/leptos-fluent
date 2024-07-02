use leptos::*;
use crate::move_ctr;

#[component]
pub(crate) fn Projects() -> impl IntoView {
    let projects_signal = create_rw_signal::<Option<Vec<Project>>>(None);
    spawn_local(async move {
        let projects = get_projects().await.unwrap();
        projects_signal.set(Some(projects));
    });


    view! {
        <style>{include_str!("./projects.css")}</style>
        <div id="projects">
            <span class="title">{move_ctr!("projects")}</span>
            {move || {
                match projects_signal.get() {
                    Some(projects) => {
                        view! {
                            <ul>
                                {projects.iter().map(|project| {
                                    view! {
                                        <li>{project.path}</li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        }.into_view()
                    }
                    None => {
                        view! {
                            <span>{move_ctr!("loading")}</span>
                        }.into_view()
                    }
                }
            }}
        </div>
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, serde::Serialize)]
pub(crate) struct Project {
    path: String
}

#[server(GetProjects, "/api")]
pub async fn get_projects(
) -> Result<Vec<Project>, ServerFnError> {
    // .. replace with your own logic
    Ok(vec![
        Project { path: "leptos-fluent".to_string() },
        Project { path: "leptos".to_string() },
    ])
}