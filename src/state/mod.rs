use crate::utils::get_stored_theme;
use leptos::*;

#[derive(Clone)]
pub struct AppState {
    pub theme: RwSignal<String>,
}

pub fn use_theme() -> (ReadSignal<String>, WriteSignal<String>) {
    let theme = use_context::<AppState>()
        .expect("AppState not found in context")
        .theme;

    (theme.read_only(), theme.write_only())
}

#[component]
pub fn AppStateProvider(children: Children) -> impl IntoView {
    // Initialize theme from localStorage or default to dark
    let initial_theme = get_stored_theme().unwrap_or_else(|| "dark".to_string());
    let theme = create_rw_signal(initial_theme);

    provide_context(AppState { theme });

    children()
}
