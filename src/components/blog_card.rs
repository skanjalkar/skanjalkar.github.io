use crate::models::BlogPost;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(post: BlogPost) -> impl IntoView {
    view! {
        <A href=format!("/blog/{}", post.id) class="blog-card">
            <img src=post.top_image alt="" loading="lazy"/>
            <div>
                <p class="blog-date">{post.date}</p>
                <h2 class="title">{post.title}</h2>
                <p class="description">{post.summary}</p>
                <span class="text-link">"Read the story →"</span>
            </div>
        </A>
    }
}
