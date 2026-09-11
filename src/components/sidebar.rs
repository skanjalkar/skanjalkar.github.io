use crate::state::AppState;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
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
            <A href="/" class="wordmark" attr:aria-label="Shreyas Kanjalkar home">
                <span class="brand-mark" aria-hidden="true">"sk."</span>
                <span>"Shreyas Kanjalkar"<small>"A personal workshop"</small></span>
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
