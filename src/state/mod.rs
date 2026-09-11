use crate::pages::terminal::TerminalSession;
use leptos::*;

#[derive(Clone, Copy)]
pub struct AppState {
    pub browse_path: RwSignal<String>,
    pub terminal: RwSignal<TerminalSession>,
}

#[component]
pub fn AppStateProvider(children: Children) -> impl IntoView {
    provide_context(AppState {
        browse_path: create_rw_signal("/".to_string()),
        terminal: create_rw_signal(TerminalSession::default()),
    });
    children()
}
