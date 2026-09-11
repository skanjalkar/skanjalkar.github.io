use crate::models::BlogPost;

pub const INTRO: &str = "I’m Shreyas, a software engineer on the AWS DSQL Storage team in Seattle. I work on distributed systems: the behind-the-scenes software that helps many computers work together.";
pub const JOURNEY: [(&str, &str); 4] = [
    ("Mechanical engineering", "Manipal Institute of Technology"),
    ("Robotics", "Worcester Polytechnic Institute"),
    ("Computer science", "Georgia Institute of Technology"),
    ("Distributed systems", "AWS DSQL Storage · Seattle"),
];
pub const INTERESTS: &str = "Away from the keyboard: chess, McLaren on race weekends, Dota 2, and clicking circles in osu!. And Spiky, the dog in my profile picture.";

#[derive(Clone, Copy)]
pub struct WorkshopProject {
    pub slug: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub language: &'static str,
    pub summary: &'static str,
    pub detail: &'static str,
    pub repository: &'static str,
}

pub const PROJECTS: [WorkshopProject; 4] = [
    WorkshopProject {
        slug: "aries",
        name: "Putting the pieces back together",
        category: "Database recovery",
        language: "Rust",
        summary: "What happens to a database when a computer stops halfway through its work?",
        detail: "An implementation of ARIES, a database recovery protocol. This project explores how a database uses its log to recover work after a crash.",
        repository: "aries-rust",
    },
    WorkshopProject {
        slug: "distributed-systems",
        name: "Getting computers to agree",
        category: "Distributed systems",
        language: "Go",
        summary: "Exploring the problems that appear when software runs across multiple machines.",
        detail: "My work through Fly.io’s distributed systems challenges, using Go to explore communication between nodes.",
        repository: "flyio_dis_sys_challenge",
    },
    WorkshopProject {
        slug: "watchman",
        name: "Finding a route that sees it all",
        category: "Robotics & motion planning",
        language: "Python",
        summary: "A motion-planning project about finding a route through a space.",
        detail: "A group project exploring the watchman route problem: finding a route from which an environment can be observed.",
        repository: "Watchman-Route-Optimal",
    },
    WorkshopProject {
        slug: "workshop",
        name: "One workshop. Two ways in.",
        category: "The site you’re exploring",
        language: "Rust · Leptos",
        summary: "A personal website you can browse with a mouse or explore from a terminal.",
        detail: "Built with Rust and Leptos, compiled to WebAssembly, and hosted on GitHub Pages. The visual pages and terminal share the same project and profile content.",
        repository: "skanjalkar.github.io",
    },
];

impl WorkshopProject {
    pub fn url(&self) -> String {
        format!("https://github.com/skanjalkar/{}", self.repository)
    }
}

pub fn posts() -> Vec<BlogPost> {
    serde_json::from_str(include_str!("../static/blog-posts.json"))
        .expect("bundled blog posts must be valid JSON")
}
