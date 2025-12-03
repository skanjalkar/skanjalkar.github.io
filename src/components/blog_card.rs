use crate::models::BlogPost;
use leptos::*;

#[component]
pub fn BlogCard(post: BlogPost) -> impl IntoView {
    let blog_url = format!("/blog/{}", post.slug);
    let image_url = format!("./blog/{}/{}", post.slug, post.cover_image);

    view! {
        <a href={blog_url} class="blog-card">
            <img src={image_url} alt={post.title.clone()} />
            <div class="content">
                <div class="title">{post.title}</div>
                <div class="description">{post.subtitle}</div>
            </div>
        </a>
    }
}
