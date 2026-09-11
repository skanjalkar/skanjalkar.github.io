use crate::components::ProjectCard;
use crate::content::PROJECTS;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <section class="page-heading">
            <p class="eyebrow">"01 / THE WORKBENCH"</p>
            <h1>"Learning by "<em>"building."</em></h1>
            <p class="intro">"A selection of personal projects and explorations, from robot motion to database recovery. Start with the idea; follow the code as far as you like."</p>
        </section>
        <div class="project-grid">
            {PROJECTS.into_iter().enumerate().map(|(index, project)| view! {
                <ProjectCard project=project index=index />
            }).collect_view()}
        </div>
        <p class="section-block"><a class="text-link" href="https://github.com/skanjalkar?tab=repositories">"More experiments and coursework on GitHub ↗"</a></p>
    }
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    view! {
        {move || {
            let slug = params.with(|p| p.get("slug").cloned().unwrap_or_default());
            PROJECTS.iter().find(|p| p.slug == slug).map(|project| view! {
                <article class="page-heading project-detail">
                    <A href="/projects" class="text-link">"← All work"</A>
                    <p class="eyebrow">{project.category}</p>
                    <h1>{project.name}</h1>
                    <p class="intro">{project.summary}</p>
                    <p class="body-copy">{project.detail}</p>
                    <div class="hero-actions">
                        <a class="button primary" href=project.url()>"Explore the code ↗"</a>
                        <A class="text-link" href=format!("/terminal?command=open%20{}", project.slug)>"Read in terminal →"</A>
                    </div>
                    <p class="eyebrow">{project.language}</p>
                </article>
            }).map(IntoView::into_view).unwrap_or_else(|| view! { <super::NotFoundPage/> }.into_view())
        }}
    }
}
