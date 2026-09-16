use crate::content::ABOUT;
use crate::models::Profile;
use leptos::*;

#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <div class="about-heading">
            <img src=Profile::default().avatar_url alt="My dog, Spiky" width="80" height="80"/>
            <h1>"About me"</h1>
        </div>
        {ABOUT.into_iter().map(|paragraph| view! {
            <p class="body-copy">{paragraph}</p>
        }).collect_view()}
    }
}
