use leptos::*;
use skanjalkar_portfolio::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    log::info!("Starting Leptos portfolio application");

    mount_to_body(App);
}
