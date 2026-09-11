use crate::content::{INTERESTS, INTRO, JOURNEY};
use crate::models::Profile;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    let profile = Profile::default();
    view! {
        <section class="page-heading">
            <p class="eyebrow">"THE PERSON BEHIND THE WORK"</p>
            <h1>"A winding path."<br/><em>"A consistent curiosity."</em></h1>
            <p class="intro">{INTRO}</p>
        </section>
        <div class="about-grid section-block">
            <section aria-labelledby="journey-title">
                <h2 id="journey-title">"From robots to distributed systems."</h2>
                <p class="body-copy">"I started in mechanical engineering, found my way into robotics, and grew to love software. My studies took me from Manipal to WPI and Georgia Tech. Today, I work on database storage at AWS."</p>
                <ol class="journey">
                    {JOURNEY.into_iter().map(|(field, place)| view! {
                        <li><strong>{field}</strong><span>{place}</span></li>
                    }).collect_view()}
                </ol>
            </section>
            <section class="personal-note" aria-labelledby="off-clock-title">
                <p class="eyebrow">"OFF THE CLOCK"</p>
                <h2 id="off-clock-title">"More than a résumé."</h2>
                <p>{INTERESTS}</p>
                <A href="/blog/about-me" class="text-link">"Read my story from 2022 →"</A>
                <div class="contact-links">
                    <a class="button primary" href=format!("mailto:{}", profile.email)>"Say hello ↗"</a>
                    <a class="text-link" href=profile.resume_url rel="external">"Read my résumé ↗"</a>
                    <a class="text-link" href=profile.linkedin_url>"LinkedIn ↗"</a>
                </div>
            </section>
        </div>
    }
}
