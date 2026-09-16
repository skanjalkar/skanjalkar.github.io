use crate::components::{AboutMe, ProjectCard};
use crate::content::PROJECTS;
use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="hero">
            <AboutMe/>
            <div class="hero-actions">
                <A href="/projects" class="text-link">"Explore my work →"</A>
                <A href="/about" class="text-link">"More about me →"</A>
                <A href="/terminal" class="text-link"><code>">_ Open terminal"</code></A>
            </div>
        </section>
        <section class="section-block" aria-labelledby="work-title">
            <div class="section-heading">
                <h2 id="work-title">"Selected projects"</h2>
                <A href="/projects" class="text-link">"All work →"</A>
            </div>
            <div class="project-grid">
                {PROJECTS.into_iter().take(2).enumerate().map(|(index, project)| view! {
                    <ProjectCard project=project index=index />
                }).collect_view()}
            </div>
        </section>
        <section class="section-block" aria-labelledby="interests-title">
            <h2 id="interests-title">"Writing"</h2>
            <A href="/blog/about-me" class="text-link">"Read my story →"</A>
        </section>
    }
}
