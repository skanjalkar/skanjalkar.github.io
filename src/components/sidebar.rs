use crate::models::Profile;
use crate::state::AppState;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let username = Profile::default().username;
    let home_label = format!("{username} home");
    let state = expect_context::<AppState>();
    let location = use_location();
    let terminal = move || location.pathname.get() == "/terminal";

    create_effect(move |_| {
        let path = location.pathname.get();
        if path != "/terminal" {
            state.browse_path.set(path);
        }
    });

    view! {
        <header class="site-header">
            <A href="/" class="wordmark" attr:aria-label=home_label>
                <span>{username}</span>
            </A>
            <nav class="site-nav" aria-label="Main navigation">
                <A href="/" exact=true>"Home"</A>
                <A href="/projects">"Work"</A>
                <A href="/blog">"Writing"</A>
                <A href="/about">"About"</A>
            </nav>
            <nav class="mode-switch" aria-label="Explore mode">
                <A href=move || state.browse_path.get() class=move || if terminal() { "" } else { "selected" }>
                    "Browse"
                </A>
                <A href="/terminal" class=move || if terminal() { "selected" } else { "" }>
                    <span aria-hidden="true">">_ "</span>"Terminal"
                </A>
            </nav>
        </header>
    }
}
