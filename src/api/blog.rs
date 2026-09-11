use crate::models::BlogPost;

pub async fn fetch_blog_posts() -> Vec<BlogPost> {
    crate::content::posts()
}

pub async fn fetch_blog_post(id: &str) -> Option<BlogPost> {
    fetch_blog_posts().await.into_iter().find(|p| p.id == id)
}
