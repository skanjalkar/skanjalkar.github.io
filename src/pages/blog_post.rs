use crate::components::Icon;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").cloned().unwrap_or_default());

    view! {
        <div>
            <a href="/blog" class="flex items-center gap-2 text-gray-400 hover:text-white mb-6">
                <Icon icon="arrow-left" />
                "Back to Blog"
            </a>

            <article class="prose prose-invert max-w-none">
                <h1 class="section-title">{slug}</h1>
                <p class="text-gray-400">
                    "Blog post content will be loaded here..."
                </p>
            </article>
        </div>
    }
}
