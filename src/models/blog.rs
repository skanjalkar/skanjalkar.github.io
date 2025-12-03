use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlogMeta {
    pub url_title: String,
    pub title: String,
    pub sub_title: String,
    pub top_image: String,
    pub visible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub subtitle: String,
    pub date: String,
    pub cover_image: String,
    pub tags: Vec<String>,
    pub content: String,
    pub visible: bool,
}

impl From<BlogMeta> for BlogPost {
    fn from(meta: BlogMeta) -> Self {
        Self {
            slug: meta.url_title,
            title: meta.title,
            subtitle: meta.sub_title,
            date: String::new(),
            cover_image: meta.top_image,
            tags: Vec::new(),
            content: String::new(),
            visible: meta.visible,
        }
    }
}
