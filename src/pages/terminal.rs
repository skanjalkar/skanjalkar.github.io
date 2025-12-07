use leptos::*;
use leptos::html::Input;

#[derive(Clone, Debug)]
struct TerminalLine {
    content: String,
    is_command: bool,
}

#[component]
pub fn TerminalPage() -> impl IntoView {
    let (history, set_history) = create_signal(Vec::<TerminalLine>::new());
    let (current_input, set_current_input) = create_signal(String::new());
    let (command_history, set_command_history) = create_signal(Vec::<String>::new());
    let (history_index, set_history_index) = create_signal(-1i32);
    let input_ref = create_node_ref::<Input>();

    // Auto-focus input on mount
    create_effect(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    // Welcome message
    create_effect(move |_| {
        set_history.set(vec![
            TerminalLine { content: r#"
  _____ _                                _  __            _       _ _             
 / ____| |                              | |/ /           (_)     | | |            
| (___ | |__  _ __ ___ _   _  __ _ ___  | ' / __ _ _ __   _  __ _| | | ____ _ _ __ 
 \___ \| '_ \| '__/ _ \ | | |/ _` / __| |  < / _` | '_ \ | |/ _` | | |/ / _` | '__|
 ____) | | | | | |  __/ |_| | (_| \__ \ | . \ (_| | | | || | (_| | |   < (_| | |   
|_____/|_| |_|_|  \___|\__, |\__,_|___/ |_|\_\__,_|_| |_|/ |\__,_|_|_|\_\__,_|_|   
                        __/ |                          |__/                        
                       |___/                                                       
"#.to_string(), is_command: false },
            TerminalLine { content: "Welcome to my interactive portfolio terminal!".to_string(), is_command: false },
            TerminalLine { content: "Type 'help' to see available commands.".to_string(), is_command: false },
            TerminalLine { content: "".to_string(), is_command: false },
        ]);
    });

    let process_command = move |cmd: String| {
        let cmd_lower = cmd.to_lowercase();
        let args: Vec<&str> = cmd_lower.split_whitespace().collect();
        let command = args.first().unwrap_or(&"");

        let output = match *command {
            "help" => vec![
                "Available commands:".to_string(),
                "".to_string(),
                "  about      - Learn about me".to_string(),
                "  skills     - View my technical skills".to_string(),
                "  projects   - Browse my projects".to_string(),
                "  blog       - Read my blog posts".to_string(),
                "  contact    - Get my contact info".to_string(),
                "  education  - View my education".to_string(),
                "  experience - View my work experience".to_string(),
                "  social     - My social media links".to_string(),
                "  resume     - Download my resume".to_string(),
                "  dota       - My Dota2 stats 🎮".to_string(),
                "  clear      - Clear the terminal".to_string(),
                "  ls         - List sections".to_string(),
                "  whoami     - Who am I?".to_string(),
                "  neofetch   - System info (just for fun)".to_string(),
                "".to_string(),
            ],
            "about" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│                  ABOUT ME                       │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "👋 Hi! I'm Shreyas Kanjalkar".to_string(),
                "".to_string(),
                "🎓 MS in Computer Science @ Georgia Tech".to_string(),
                "🎓 MS in Robotics @ WPI".to_string(),
                "💼 SDE at AWS DSQL Storage team".to_string(),
                "📍 Atlanta, GA, USA".to_string(),
                "".to_string(),
                "I'm passionate about distributed systems, cloud computing,".to_string(),
                "and building reliable software at scale.".to_string(),
                "".to_string(),
                "Fun fact: I reached Immortal rank in Dota2! 🏆".to_string(),
                "Type 'dota' to learn more about my gaming journey.".to_string(),
                "".to_string(),
            ],
            "skills" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│              TECHNICAL SKILLS                   │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "🦀 Languages:".to_string(),
                "   Rust ████████████████████ Expert".to_string(),
                "   Python ██████████████████░░ Advanced".to_string(),
                "   Java █████████████████░░░░ Advanced".to_string(),
                "   C++ ████████████████░░░░░ Proficient".to_string(),
                "   TypeScript █████████████░░░░░░ Proficient".to_string(),
                "".to_string(),
                "☁️  Cloud & Infrastructure:".to_string(),
                "   AWS, Docker, Kubernetes, Terraform".to_string(),
                "".to_string(),
                "🗄️  Databases:".to_string(),
                "   PostgreSQL, DynamoDB, Redis, Aurora".to_string(),
                "".to_string(),
                "🔧 Tools:".to_string(),
                "   Git, Linux, CI/CD, Prometheus, Grafana".to_string(),
                "".to_string(),
            ],
            "projects" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│                 PROJECTS                        │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "🔗 Visit /projects to see all my work, or check out:".to_string(),
                "".to_string(),
                "📦 skanjalkar.github.io".to_string(),
                "   This portfolio! Built with Rust + Leptos + WASM".to_string(),
                "   → https://github.com/skanjalkar/skanjalkar.github.io".to_string(),
                "".to_string(),
                "Type 'open projects' to navigate there.".to_string(),
                "".to_string(),
            ],
            "blog" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│                   BLOG                          │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "📝 Recent posts:".to_string(),
                "".to_string(),
                "  1. About Me (2022-09-24)".to_string(),
                "     My journey from India to the states".to_string(),
                "".to_string(),
                "Type 'open blog' to read all posts.".to_string(),
                "".to_string(),
            ],
            "contact" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│                  CONTACT                        │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "📧 Email: skanjalkar@gmail.com".to_string(),
                "💼 LinkedIn: linkedin.com/in/shreyaskanjalkar".to_string(),
                "🐙 GitHub: github.com/skanjalkar".to_string(),
                "🌐 Website: skanjalkar.github.io".to_string(),
                "".to_string(),
                "Feel free to reach out! I'm always happy to connect.".to_string(),
                "".to_string(),
            ],
            "education" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│                 EDUCATION                       │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "🎓 Georgia Institute of Technology".to_string(),
                "   M.S. Computer Science".to_string(),
                "   Focus: Computing Systems".to_string(),
                "".to_string(),
                "🎓 Worcester Polytechnic Institute (WPI)".to_string(),
                "   M.S. Robotics Engineering".to_string(),
                "   2021 - 2023".to_string(),
                "".to_string(),
                "🎓 Manipal Institute of Technology".to_string(),
                "   B.Tech".to_string(),
                "   2016 - 2020".to_string(),
                "".to_string(),
            ],
            "experience" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│              WORK EXPERIENCE                    │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "💼 Amazon Web Services (AWS)".to_string(),
                "   Software Development Engineer".to_string(),
                "   DSQL Storage Team".to_string(),
                "   📍 Seattle, WA | 2023 - Present".to_string(),
                "".to_string(),
                "   • Building distributed database systems".to_string(),
                "   • Working on Aurora DSQL storage layer".to_string(),
                "   • Rust, distributed systems, cloud infra".to_string(),
                "".to_string(),
            ],
            "social" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│               SOCIAL LINKS                      │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "🐙 GitHub:   github.com/skanjalkar".to_string(),
                "💼 LinkedIn: linkedin.com/in/shreyaskanjalkar".to_string(),
                "🎮 Steam:    steamcommunity.com/id/EchizeNNN".to_string(),
                "".to_string(),
            ],
            "resume" => vec![
                "📄 Download my resume:".to_string(),
                "   → /resume/resume.pdf".to_string(),
                "".to_string(),
                "Opening resume...".to_string(),
                "(In a real browser, this would open the PDF)".to_string(),
                "".to_string(),
            ],
            "dota" => vec![
                "╭─────────────────────────────────────────────────╮".to_string(),
                "│              🎮 DOTA 2 STATS 🎮                  │".to_string(),
                "╰─────────────────────────────────────────────────╯".to_string(),
                "".to_string(),
                "🏆 Peak Rank: IMMORTAL".to_string(),
                "⭐ Achievement: Top 0.1% of players".to_string(),
                "📅 Reached: July 2021".to_string(),
                "".to_string(),
                "🎯 Favorite Heroes:".to_string(),
                "   • Invoker".to_string(),
                "   • Storm Spirit".to_string(),
                "   • Ember Spirit".to_string(),
                "".to_string(),
                "🔗 Steam: steamcommunity.com/id/EchizeNNN".to_string(),
                "".to_string(),
                "\"It might not matter to most people, but it's one".to_string(),
                " of my proudest achievements in my life.\"".to_string(),
                "".to_string(),
            ],
            "ls" => vec![
                "drwxr-xr-x  about/".to_string(),
                "drwxr-xr-x  skills/".to_string(),
                "drwxr-xr-x  projects/".to_string(),
                "drwxr-xr-x  blog/".to_string(),
                "drwxr-xr-x  contact/".to_string(),
                "-rw-r--r--  resume.pdf".to_string(),
                "".to_string(),
            ],
            "whoami" => vec![
                "shreyas_kanjalkar".to_string(),
                "".to_string(),
            ],
            "neofetch" => vec![
                "".to_string(),
                "        ⬛⬛⬛⬛⬛⬛⬛⬛          shreyas@portfolio".to_string(),
                "      ⬛🟧🟧🟧🟧🟧🟧🟧🟧⬛        -----------------".to_string(),
                "    ⬛🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧⬛      OS: Rust + WASM".to_string(),
                "   ⬛🟧🟧⬜⬜🟧🟧⬜⬜🟧🟧⬛      Host: GitHub Pages".to_string(),
                "   ⬛🟧🟧⬜⬜🟧🟧⬜⬜🟧🟧⬛      Kernel: Leptos 0.6".to_string(),
                "   ⬛🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧⬛      Shell: Interactive Terminal".to_string(),
                "   ⬛🟧🟧🟧⬛⬛⬛⬛🟧🟧🟧⬛      Theme: Dark Mode".to_string(),
                "    ⬛🟧🟧🟧🟧🟧🟧🟧🟧🟧⬛       CPU: Your Browser".to_string(),
                "     ⬛🟧🟧🟧🟧🟧🟧🟧🟧⬛        Memory: Very Efficient".to_string(),
                "       ⬛⬛⬛⬛⬛⬛⬛⬛          ".to_string(),
                "".to_string(),
            ],
            "clear" => vec![],
            "open" => {
                if args.len() > 1 {
                    match args[1] {
                        "projects" => vec!["Navigating to /projects...".to_string(), "".to_string()],
                        "blog" => vec!["Navigating to /blog...".to_string(), "".to_string()],
                        "home" => vec!["Navigating to /...".to_string(), "".to_string()],
                        _ => vec![format!("Unknown destination: {}", args[1]), "".to_string()],
                    }
                } else {
                    vec!["Usage: open <destination>".to_string(), "Destinations: projects, blog, home".to_string(), "".to_string()]
                }
            },
            "sudo" => vec![
                "Nice try! 😄".to_string(),
                "But you don't have root access here.".to_string(),
                "".to_string(),
            ],
            "rm" => vec![
                "🛡️ Permission denied: Cannot delete portfolio files!".to_string(),
                "".to_string(),
            ],
            "exit" => vec![
                "Goodbye! Thanks for visiting! 👋".to_string(),
                "(Refresh the page to restart the terminal)".to_string(),
                "".to_string(),
            ],
            "date" => {
                vec![
                    "Terminal time: Right now! ⏰".to_string(),
                    "".to_string(),
                ]
            },
            "echo" => {
                if args.len() > 1 {
                    vec![args[1..].join(" "), "".to_string()]
                } else {
                    vec!["".to_string()]
                }
            },
            "pwd" => vec![
                "/home/shreyas/portfolio".to_string(),
                "".to_string(),
            ],
            "cat" => {
                if args.len() > 1 {
                    match args[1] {
                        "resume.pdf" => vec![
                            "📄 Binary file - use 'resume' command to download".to_string(),
                            "".to_string(),
                        ],
                        _ => vec![
                            format!("cat: {}: No such file", args[1]),
                            "".to_string(),
                        ],
                    }
                } else {
                    vec!["Usage: cat <filename>".to_string(), "".to_string()]
                }
            },
            "" => vec![],
            _ => vec![
                format!("Command not found: {}. Type 'help' for available commands.", command),
                "".to_string(),
            ],
        };

        output
    };

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let cmd = current_input.get();

        if cmd.to_lowercase() == "clear" {
            set_history.set(vec![]);
        } else {
            let mut new_history = history.get();
            new_history.push(TerminalLine {
                content: format!("$ {}", cmd),
                is_command: true,
            });

            let output = process_command(cmd.clone());
            for line in output {
                new_history.push(TerminalLine {
                    content: line,
                    is_command: false,
                });
            }

            set_history.set(new_history);
        }

        // Add to command history for up/down navigation
        if !cmd.is_empty() {
            let mut cmds = command_history.get();
            cmds.push(cmd);
            set_command_history.set(cmds);
            set_history_index.set(-1);
        }

        set_current_input.set(String::new());
    };

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowUp" => {
                ev.prevent_default();
                let cmds = command_history.get();
                if !cmds.is_empty() {
                    let new_index = if history_index.get() < 0 {
                        cmds.len() as i32 - 1
                    } else {
                        (history_index.get() - 1).max(0)
                    };
                    set_history_index.set(new_index);
                    if let Some(cmd) = cmds.get(new_index as usize) {
                        set_current_input.set(cmd.clone());
                    }
                }
            }
            "ArrowDown" => {
                ev.prevent_default();
                let cmds = command_history.get();
                let new_index = history_index.get() + 1;
                if new_index >= cmds.len() as i32 {
                    set_history_index.set(-1);
                    set_current_input.set(String::new());
                } else {
                    set_history_index.set(new_index);
                    if let Some(cmd) = cmds.get(new_index as usize) {
                        set_current_input.set(cmd.clone());
                    }
                }
            }
            _ => {}
        }
    };

    // Click anywhere to focus input
    let focus_input = move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    };

    view! {
        <div class="terminal-container" on:click=focus_input>
            <div class="terminal-header">
                <div class="terminal-buttons">
                    <span class="terminal-button red"></span>
                    <span class="terminal-button yellow"></span>
                    <span class="terminal-button green"></span>
                </div>
                <span class="terminal-title">"shreyas@portfolio: ~"</span>
            </div>
            <div class="terminal-body">
                <div class="terminal-output">
                    <For
                        each=move || history.get()
                        key=|line| line.content.clone()
                        children=move |line| {
                            view! {
                                <div class=move || if line.is_command { "terminal-line command" } else { "terminal-line" }>
                                    <pre>{line.content.clone()}</pre>
                                </div>
                            }
                        }
                    />
                </div>
                <form class="terminal-input-line" on:submit=handle_submit>
                    <span class="terminal-prompt">"$ "</span>
                    <input
                        type="text"
                        class="terminal-input"
                        node_ref=input_ref
                        prop:value=move || current_input.get()
                        on:input=move |ev| set_current_input.set(event_target_value(&ev))
                        on:keydown=handle_keydown
                        autocomplete="off"
                        spellcheck="false"
                    />
                </form>
            </div>
        </div>
    }
}