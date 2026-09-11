use crate::content::WorkshopProject;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ProjectCard(project: WorkshopProject, #[prop(default = 0)] index: usize) -> impl IntoView {
    view! {
        <article class="work-card">
            <div class="work-card-top"><span>{format!("{:02}", index + 1)}</span><span>{project.category}</span><span aria-hidden="true">"↗"</span></div>
            <h3><A href=format!("/projects/{}", project.slug)>{project.name}</A></h3>
            <p>{project.summary}</p>
            <div class="work-card-bottom">
                <span>{project.language}</span>
                <A href=format!("/terminal?command=open%20{}", project.slug) attr:aria-label=format!("Open {} in terminal", project.name)>
                    <code>{format!("open {}", project.slug)}</code>
                </A>
            </div>
        </article>
    }
}
