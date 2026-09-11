use crate::models::Profile;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    let profile = Profile::default();
    view! {
        <footer class="site-footer">
            <p>"Made with curiosity. Built with Rust."<br/><span>"Seattle, Washington"</span></p>
            <div>
                <a href=profile.github_url>"GitHub ↗"</a>
                <a href=profile.linkedin_url>"LinkedIn ↗"</a>
                <a href=format!("mailto:{}", profile.email)>"Say hello ↗"</a>
            </div>
        </footer>
    }
}
