use crate::models::BlogPost;
use gloo_net::http::Request;

pub async fn fetch_blog_posts() -> Vec<BlogPost> {
    match Request::get("/blog-posts.json").send().await {
        Ok(response) => match response.json::<Vec<BlogPost>>().await {
            Ok(posts) => posts,
            Err(e) => {
                log::error!("Failed to parse blog posts: {:?}", e);
                Vec::new()
            }
        },
        Err(e) => {
            log::error!("Failed to fetch blog posts: {:?}", e);
            Vec::new()
        }
    }
}

pub async fn fetch_blog_post(id: &str) -> Option<BlogPost> {
    let posts = fetch_blog_posts().await;
    posts.into_iter().find(|p| p.id == id)
}