use leptos::*;
use crate::components::{Sidebar, Footer};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-bg-dark">
            <div class="flex flex-col lg:flex-row">
                <Sidebar />
                <main class="main-content">
                    {children()}
                    <Footer />
                </main>
            </div>
        </div>
    }
}