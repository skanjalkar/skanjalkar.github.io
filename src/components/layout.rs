use crate::components::{Footer, Sidebar};
use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-bg-dark">
            <div class="flex flex-col lg:flex-row">
                <Sidebar />
                <main class="main-content main-animated">
                    <div class="page-transition">
                        {children()}
                    </div>
                    <Footer />
                </main>
            </div>
        </div>
    }
}
