use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlogContentItem {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub date: String,
    pub summary: String,
    pub top_image: String,
    pub content: Vec<BlogContentItem>,
}

impl Default for BlogPost {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            date: String::new(),
            summary: String::new(),
            top_image: String::new(),
            content: Vec::new(),
        }
    }
}
