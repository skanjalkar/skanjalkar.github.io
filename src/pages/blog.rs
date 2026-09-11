use crate::components::BlogCard;
use crate::content::posts;
use leptos::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <section class="page-heading">
            <p class="eyebrow">"02 / NOTES & STORIES"</p>
            <h1>"A few things "<em>"along the way."</em></h1>
            <p class="intro">"Personal stories and notes from the journey. Each entry is a snapshot of where I was when I wrote it."</p>
        </section>
        <div class="blog-list">
            {posts().into_iter().map(|post| view! { <BlogCard post=post/> }).collect_view()}
        </div>
    }
}
