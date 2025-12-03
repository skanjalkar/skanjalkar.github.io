use crate::models::BlogPost;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(post: BlogPost) -> impl IntoView {
    let blog_url = format!("/blog/{}", post.id);

    view! {
        <A href={blog_url} class="blog-card">
            <img src={post.top_image.clone()} alt={post.title.clone()} />
            <div class="content">
                <div class="title">{post.title}</div>
                <div class="description">{post.summary}</div>
                <div class="text-gray-500 text-sm mt-2">{post.date}</div>
            </div>
        </A>
    }
}
