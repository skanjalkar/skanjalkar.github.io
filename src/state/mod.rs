use leptos::*;

#[derive(Clone)]
pub struct AppState {}

#[component]
pub fn AppStateProvider(children: Children) -> impl IntoView {
    provide_context(AppState {});
    children()
}
