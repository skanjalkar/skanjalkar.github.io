use crate::components::{Footer, Sidebar};
use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <a class="skip-link" href="#main">"Skip to content"</a>
        <div class="site-shell">
            <Sidebar />
            <main id="main" tabindex="-1">{children()}</main>
            <Footer />
        </div>
    }
}
