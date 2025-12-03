use leptos::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-[50vh] text-center">
            <h1 class="text-6xl font-bold mb-4">"404"</h1>
            <p class="text-xl text-gray-400 mb-8">"Page not found"</p>
            <a href="/" class="btn-primary">"Go Home"</a>
        </div>
    }
}