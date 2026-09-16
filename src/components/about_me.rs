use crate::content::ABOUT;
use leptos::*;

#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <div class="about-heading">
            <img src="/static/blog/about-me/img_0.jpeg" alt="My dog, Spiky" width="726" height="968"/>
            <h1>"About me"</h1>
        </div>
        {ABOUT.into_iter().map(|paragraph| view! {
            <p class="body-copy">{paragraph}</p>
        }).collect_view()}
    }
}
