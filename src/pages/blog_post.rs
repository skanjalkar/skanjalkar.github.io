use crate::api::fetch_blog_post;
use crate::components::{Icon, Loading};
use crate::models::BlogContentItem;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").cloned().unwrap_or_default());

    let post = create_local_resource(slug, |slug| async move { fetch_blog_post(&slug).await });

    view! {
        <div>
            <a href="/blog" class="flex items-center gap-2 text-gray-400 hover:text-white mb-6">
                <Icon icon="arrow-left" />
                "Back to Blog"
            </a>

            <Suspense fallback=move || view! { <Loading /> }>
                {move || {
                    post.get().map(|maybe_post| {
                        match maybe_post {
                            Some(post) => view! {
                                <article class="max-w-3xl">
                                    <div
                                        class="w-full h-64 rounded-lg bg-cover bg-center bg-no-repeat mb-8"
                                        style=format!("background-image: url('{}')", post.top_image)
                                    />
                                    <h1 class="section-title mb-2">{post.title}</h1>
                                    <p class="text-gray-500 mb-8">{post.date}</p>

                                    <div class="space-y-6">
                                        {post.content.into_iter().map(|item| {
                                            render_content_item(item)
                                        }).collect_view()}
                                    </div>
                                </article>
                            }.into_view(),
                            None => view! {
                                <div class="text-center">
                                    <h1 class="section-title">"Post Not Found"</h1>
                                    <p class="text-gray-400">"The blog post you're looking for doesn't exist."</p>
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

fn render_content_item(item: BlogContentItem) -> impl IntoView {
    match item.content_type.as_str() {
        "paragraph" => {
            let text = item.text.unwrap_or_default();
            view! {
                <p class="text-gray-300 font-display text-lg leading-relaxed">{text}</p>
            }
            .into_view()
        }
        "image" => {
            let src = item.src.unwrap_or_default();
            let alt = item.alt.unwrap_or_default();
            view! {
                <img src={src} alt={alt} class="rounded-lg w-full max-w-xl mx-auto my-8" />
            }
            .into_view()
        }
        _ => view! { <div></div> }.into_view(),
    }
}
