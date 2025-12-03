use gloo_storage::{LocalStorage, Storage};

const THEME_KEY: &str = "theme";

pub fn get_stored_theme() -> Option<String> {
    LocalStorage::get(THEME_KEY).ok()
}

pub fn store_theme(theme: &str) {
    let _ = LocalStorage::set(THEME_KEY, theme);
}