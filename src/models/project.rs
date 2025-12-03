use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub topics: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub featured: bool,
}

impl Project {
    pub fn display_description(&self) -> String {
        self.description
            .clone()
            .unwrap_or_else(|| "No description available".to_string())
    }

    pub fn display_language(&self) -> String {
        self.language
            .clone()
            .unwrap_or_else(|| "Unknown".to_string())
    }
}
