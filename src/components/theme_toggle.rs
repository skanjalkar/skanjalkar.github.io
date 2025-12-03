use crate::components::Icon;
use crate::state::use_theme;
use leptos::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = use_theme();

    let toggle_theme = move |_| {
        let new_theme = if theme.get() == "dark" {
            "light".to_string()
        } else {
            "dark".to_string()
        };

        // Update the HTML element class for Tailwind dark mode
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.class_list().remove_1(if new_theme == "dark" {
                        "light"
                    } else {
                        "dark"
                    });
                    let _ = html.class_list().add_1(&new_theme);
                }
            }
        }

        // Save to localStorage
        crate::utils::store_theme(&new_theme);

        set_theme.set(new_theme);
    };

    // Apply theme on mount
    create_effect(move |_| {
        let current_theme = theme.get();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.class_list().remove_1("light");
                    let _ = html.class_list().remove_1("dark");
                    let _ = html.class_list().add_1(&current_theme);
                }
            }
        }
    });

    view! {
        <button
            on:click=toggle_theme
            class="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 transition-colors duration-200"
            aria-label="Toggle theme"
        >
            {move || if theme.get() == "dark" {
                view! { <Icon icon="sun" /> " Light" }.into_view()
            } else {
                view! { <Icon icon="moon" /> " Dark" }.into_view()
            }}
        </button>
    }
}
