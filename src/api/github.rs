use gloo_net::http::Request;
use crate::models::Project;

const GITHUB_API_BASE: &str = "https://api.github.com";

pub async fn fetch_github_repos(username: &str) -> Vec<Project> {
    let url = format!(
        "{}/users/{}/repos?sort=updated&per_page=50",
        GITHUB_API_BASE, username
    );
    
    let response = match Request::get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            log::error!("Network error fetching repos: {}", e);
            return Vec::new();
        }
    };
    
    if !response.ok() {
        log::error!("GitHub API error: {}", response.status());
        return Vec::new();
    }
    
    match response.json::<Vec<Project>>().await {
        Ok(repos) => repos.into_iter().filter(|r| !r.name.contains(".github")).collect(),
        Err(e) => {
            log::error!("Parse error: {}", e);
            Vec::new()
        }
    }
}