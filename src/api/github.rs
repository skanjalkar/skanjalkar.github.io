use gloo_net::http::Request;
use crate::models::Project;

const GITHUB_API_BASE: &str = "https://api.github.com";

#[derive(Debug)]
pub struct ApiError(String);

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub async fn fetch_github_repos(username: &str) -> Result<Vec<Project>, ApiError> {
    let url = format!(
        "{}/users/{}/repos?sort=updated&per_page=50",
        GITHUB_API_BASE, username
    );
    
    let response = Request::get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| ApiError(format!("Network error: {}", e)))?;
    
    if !response.ok() {
        return Err(ApiError(format!(
            "GitHub API error: {}",
            response.status()
        )));
    }
    
    let repos: Vec<Project> = response
        .json()
        .await
        .map_err(|e| ApiError(format!("Parse error: {}", e)))?;
    
    Ok(repos.into_iter().filter(|r| !r.name.contains(".github")).collect())
}