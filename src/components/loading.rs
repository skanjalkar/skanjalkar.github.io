use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center py-12">
            <div class="spinner"></div>
        </div>
    }
}

#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center min-h-[50vh]">
            <div class="flex flex-col items-center gap-4">
                <div class="spinner"></div>
                <span class="text-gray-400 font-display">"Loading..."</span>
            </div>
        </div>
    }
}