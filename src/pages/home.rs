use crate::components::ProjectCard;
use crate::content::{INTRO, PROJECTS};
use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-copy">
                <p class="eyebrow"><span class="status-dot"></span>"ENGINEER / BUILDER / CURIOUS HUMAN"</p>
                <h1>"Big systems."<br/>"Small details."<br/><em>"Endless curiosity."</em></h1>
                <p class="intro">{INTRO}</p>
                <div class="hero-actions">
                    <A href="/projects" class="button primary">"Explore my work"<span>"↗"</span></A>
                    <A href="/about" class="text-link">"A little about me →"</A>
                </div>
            </div>
            <div class="workshop-note">
                <span class="note-label">"A NOTE FROM THE WORKSHOP"</span>
                <div class="system-sketch" aria-hidden="true">
                    <span class="sketch-node node-one">"ideas"</span>
                    <span class="sketch-node node-two">"build"</span>
                    <span class="sketch-node node-three">"learn"</span>
                    <span class="sketch-center">"↻"</span>
                </div>
                <p>"From moving robots"<br/>"to moving data."</p>
                <span class="note-caption">"Different problems. Same curiosity."</span>
                <A href="/terminal" class="terminal-invitation">
                    <code>">_ hello, world"</code>
                    <span>"Prefer a keyboard? Step inside →"</span>
                </A>
            </div>
        </section>
        <section class="section-block" aria-labelledby="work-title">
            <div class="section-heading">
                <div><p class="eyebrow">"01 / SELECTED WORK"</p><h2 id="work-title">"Things I’ve been figuring out."</h2></div>
                <A href="/projects" class="text-link">"All work ↗"</A>
            </div>
            <div class="project-grid">
                {PROJECTS.into_iter().take(2).enumerate().map(|(index, project)| view! {
                    <ProjectCard project=project index=index />
                }).collect_view()}
            </div>
        </section>
        <section class="personal-grid section-block" aria-label="Beyond the work">
            <div class="personal-note">
                <p class="eyebrow">"02 / OFF THE CLOCK"</p>
                <h2>"There’s a person"<br/>"behind the prompt."</h2>
                <p>"Chess positions. McLaren race weekends. A few too many games of Dota 2. And a dog called Spiky."</p>
                <A href="/about" class="text-link">"Meet the rest of me →"</A>
            </div>
            <A href="/blog/about-me" class="spiky-card">
                <img src="/static/blog/about-me/img_0.jpeg" alt="Spiky, Shreyas’s dog" loading="lazy"/>
                <span>"Meet Spiky"<small>"An important part of the story. ↗"</small></span>
            </A>
        </section>
    }
}
