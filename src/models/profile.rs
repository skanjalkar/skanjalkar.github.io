use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub avatar_url: String,
    pub bio: String,
    pub location: String,
    pub email: String,
    pub linkedin_url: Option<String>,
    pub github_url: String,
    pub resume_url: Option<String>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: "Shreyas Kanjalkar".to_string(),
            username: "skanjalkar".to_string(),
            avatar_url: "https://avatars.githubusercontent.com/u/37182124?v=4".to_string(),
            bio: "SDE at AWS DSQL Storage team, previously completed MS CS at Georgia Tech and Robotics at WPI."
                .to_string(),
            location: "Seattle, WA, USA".to_string(),
            email: "firstname+lastname+at+gmail+com".to_string(),
            linkedin_url: Some("https://www.linkedin.com/in/shreyas1405/".to_string()),
            github_url: "https://github.com/skanjalkar".to_string(),
            resume_url: Some("resume/resume.pdf".to_string()),
        }
    }
}
