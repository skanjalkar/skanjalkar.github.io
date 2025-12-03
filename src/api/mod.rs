mod blog;
mod github;

pub use blog::{fetch_blog_post, fetch_blog_posts};
pub use github::fetch_github_repos;
