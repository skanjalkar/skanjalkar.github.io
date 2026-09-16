use crate::models::BlogPost;

pub const ABOUT: [&str; 3] = [
    "My name is Shreyas Kanjalkar and that is my dog in my pfp. His name is Spiky. Cute right? I am a Masters Student studying Computer Science at Georgia Institute of Technology (Gatech) in Atlanta, Georgia. I have done masters in Robotics at Worcester Polytechnic Institute, WPI.",
    "I did my undergrad in Mechanical Engineering at Manipal Institute of Technology. During my time at WPI, I have grown interest and affection towards Software Engineering. Now I wish to be able to work in the industry. I am still learning about all there is to offer about Computer Science. My main interest is in cloud computing and distributed systems, hoping to work in the industry on those topics.",
    "When I am not working, I enjoy watching and playing chess. I religiously follow Formula 1 and no, I am not a \"Big 3\" fan. I am a McLaren life long fan. I am currently taking a break from Dota2 and sometimes I click circles on osu!",
];

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
