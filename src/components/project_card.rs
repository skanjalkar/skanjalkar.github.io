use crate::components::Icon;
use crate::models::Project;
use leptos::*;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <a
            href={project.html_url.clone()}
            target="_blank"
            class="project-card"
        >
            <div class="title">{project.name.clone()}</div>
            <div class="description">{project.display_description()}</div>
            <div class="meta">
                <span class="flex items-center gap-1">
                    <Icon icon="code" />
                    {project.display_language()}
                </span>
                <span class="flex items-center gap-1">
                    <Icon icon="star" />
                    {project.stargazers_count}
                </span>
                <span class="flex items-center gap-1">
                    <Icon icon="fork" />
                    {project.forks_count}
                </span>
            </div>
        </a>
    }
}
