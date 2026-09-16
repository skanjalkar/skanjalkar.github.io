use crate::components::AboutMe;
use crate::models::Profile;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    let profile = Profile::default();
    view! {
        <section class="hero">
            <AboutMe/>
            <div class="contact-links">
                <A href="/blog/about-me" class="text-link">"Read my story →"</A>
                <a class="text-link" href=format!("mailto:{}", profile.email)>"E-mail ↗"</a>
                <a class="text-link" href=profile.resume_url rel="external">"Read my résumé ↗"</a>
                <a class="text-link" href=profile.linkedin_url>"LinkedIn ↗"</a>
            </div>
        </section>
    }
}
