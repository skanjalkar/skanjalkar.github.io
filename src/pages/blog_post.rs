use crate::content::posts;
use crate::models::BlogContentItem;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    view! {
        {move || {
            let slug = params.with(|p| p.get("slug").cloned().unwrap_or_default());
            match posts().into_iter().find(|post| post.id == slug) {
                Some(post) => view! {
                    <article class="article">
                        <A href="/blog" class="text-link">"← All writing"</A>
                        <h1>{post.title}</h1>
                        <p class="blog-date">{post.date}</p>
                        <p class="archive-note">"From the archive: details reflect the time this was written. "<A href="/about" class="text-link">"Meet me today →"</A></p>
                        <A href=format!("/terminal?command=read%20{}", post.id) class="text-link">"Read in terminal →"</A>
                        <img class="article-cover" src=post.top_image alt=""/>
                        <div class="article-body">
                            {post.content.into_iter().map(render_content_item).collect_view()}
                        </div>
                    </article>
                }.into_view(),
                None => view! { <super::NotFoundPage/> }.into_view(),
            }
        }}
    }
}

fn render_content_item(item: BlogContentItem) -> View {
    match item.content_type.as_str() {
        "paragraph" => view! { <p>{item.text.unwrap_or_default()}</p> }.into_view(),
        "image" => view! { <img src=item.src.unwrap_or_default() alt=item.alt.unwrap_or_default() loading="lazy"/> }.into_view(),
        _ => ().into_view(),
    }
}
