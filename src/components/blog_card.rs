use crate::models::BlogPost;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(post: BlogPost, #[prop(default = 0)] index: usize) -> impl IntoView {
    let blog_url = format!("/blog/{}", post.id);
    let stagger_class = format!("stagger-{}", (index % 8) + 1);

    view! {
        <A href={blog_url} class=format!("blog-card hover-lift hover-glow card-animated {}", stagger_class)>
            <img src={post.top_image.clone()} alt={post.title.clone()} class="transition-transform duration-300 hover:scale-105" />
            <div class="content">
                <div class="title">{post.title}</div>
                <div class="description">{post.summary}</div>
                <div class="text-gray-500 text-sm mt-2">{post.date}</div>
            </div>
        </A>
    }
}
