use leptos::*;
use crate::state::use_theme;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = use_theme();
    
    let toggle_theme = move |_| {
        set_theme.update(|t| {
            *t = if *t == "dark" { "light".to_string() } else { "dark".to_string() }
        });
    };
    
    view! {
        <button 
            on:click=toggle_theme
            class="btn-secondary mt-4"
            aria-label="Toggle theme"
        >
            {move || if theme.get() == "dark" { "Light Mode" } else { "Dark Mode" }}
        </button>
    }
}