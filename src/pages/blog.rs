use crate::components::BlogCard;
use crate::models::BlogPost;
use leptos::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = vec![BlogPost {
        slug: "About-me".to_string(),
        title: "About me".to_string(),
        subtitle: "24th Sept 2022".to_string(),
        date: "2022-09-24".to_string(),
        cover_image: "top_image.jpeg".to_string(),
        tags: vec![],
        content: String::new(),
        visible: true,
    }];

    view! {
        <div>
            <h1 class="section-title">"Blog"</h1>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                {posts.into_iter()
                    .filter(|p| p.visible)
                    .map(|post| view! { <BlogCard post=post /> })
                    .collect_view()}
            </div>
        </div>
    }
}
