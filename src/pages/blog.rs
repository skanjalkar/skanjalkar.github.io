use crate::api::fetch_blog_posts;
use crate::components::{BlogCard, Loading};
use leptos::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = create_local_resource(|| (), |_| async move { fetch_blog_posts().await });

    view! {
        <div>
            <h1 class="section-title section-title-animated animate-fade-in-down">"Blog"</h1>

            <Suspense fallback=move || view! { <Loading /> }>
                {move || {
                    posts.get().map(|posts| {
                        if posts.is_empty() {
                            view! {
                                <p class="text-gray-400">"No blog posts yet."</p>
                            }.into_view()
                        } else {
                            view! {
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    {posts.into_iter()
                                        .enumerate()
                                        .map(|(index, post)| view! { <BlogCard post=post index=index /> })
                                        .collect_view()}
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
