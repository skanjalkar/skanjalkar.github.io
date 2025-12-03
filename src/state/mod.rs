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
    let theme = create_rw_signal("dark".to_string());
    
    provide_context(AppState { theme });
    
    children()
}