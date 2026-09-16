use crate::components::ProjectCard;
use crate::content::{INTERESTS, INTRO, PROJECTS};
use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="hero">
            <h1>"About me"</h1>
            <p class="intro">{INTRO}</p>
            <p class="body-copy">"My path here started with mechanical engineering at Manipal, then robotics at WPI and computer science at Georgia Tech."</p>
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
            <h2 id="interests-title">"Away from the keyboard"</h2>
            <p class="body-copy">{INTERESTS}</p>
            <A href="/blog/about-me" class="text-link">"Read my story →"</A>
        </section>
    }
}
