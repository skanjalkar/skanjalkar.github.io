use leptos::html::{Div, Input};
use leptos::*;

// Virtual file system for cat command
fn get_virtual_file(name: &str) -> Option<Vec<String>> {
    match name.to_lowercase().as_str() {
        "about.txt" | "about" => Some(vec![
            "=== About Shreyas Kanjalkar ===".to_string(),
            "".to_string(),
            "Software Engineer passionate about distributed systems".to_string(),
            "and cloud computing.".to_string(),
            "".to_string(),
            "Education:".to_string(),
            "  - MS Computer Science @ Georgia Tech".to_string(),
            "  - MS Robotics @ WPI".to_string(),
            "  - BE Mechanical Engineering @ Manipal".to_string(),
            "".to_string(),
            "Type 'cd home' to learn more!".to_string(),
        ]),
        "skills.txt" | "skills" => Some(vec![
            "=== Technical Skills ===".to_string(),
            "".to_string(),
            "Languages:    Rust, Go, Python, TypeScript, Java".to_string(),
            "Cloud:        AWS, GCP, Kubernetes, Docker".to_string(),
            "Systems:      Distributed Systems, Microservices".to_string(),
            "Frameworks:   Leptos, React, Actix, gRPC".to_string(),
            "Databases:    PostgreSQL, Redis, DynamoDB".to_string(),
            "Tools:        Git, Linux, Terraform, CI/CD".to_string(),
        ]),
        "interests.txt" | "interests" => Some(vec![
            "=== Interests & Hobbies ===".to_string(),
            "".to_string(),
            "Chess       - Always up for a game!".to_string(),
            "Formula 1   - McLaren fan".to_string(),
            "Dota 2      - Casual player".to_string(),
            "osu!        - Click the circles".to_string(),
        ]),
        "contact.txt" | "contact" => Some(vec![
            "=== Contact Information ===".to_string(),
            "".to_string(),
            "Email:    skanjalkar [at] gmail.com".to_string(),
            "GitHub:   github.com/skanjalkar".to_string(),
            "LinkedIn: linkedin.com/in/skanjalkar".to_string(),
            "".to_string(),
            "Type 'social' for clickable links!".to_string(),
        ]),
        _ => None,
    }
}

fn get_available_files() -> Vec<&'static str> {
    vec!["about.txt", "skills.txt", "interests.txt", "contact.txt"]
}

#[derive(Clone, Debug, PartialEq)]
enum LineType {
    Command,
    Output,
}

#[derive(Clone, Debug)]
struct TerminalLine {
    content: String,
    line_type: LineType,
}

#[derive(Clone, Debug, PartialEq)]
enum CdDestination {
    Projects,
    Blog,
    Home,
}

impl CdDestination {
    const DIRECTORIES: [&'static str; 3] = ["blog", "home", "projects"];

    fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "projects" => Some(Self::Projects),
            "blog" => Some(Self::Blog),
            "home" | "/" | "~" => Some(Self::Home),
            _ => None,
        }
    }

    fn path(&self) -> &'static str {
        match self {
            Self::Projects => "/projects",
            Self::Blog => "/blog",
            Self::Home => "/home",
        }
    }

    fn complete(partial: &str) -> Option<&'static str> {
        let partial_lower = partial.to_lowercase();
        Self::DIRECTORIES
            .iter()
            .find(|dir| dir.starts_with(&partial_lower))
            .copied()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    Cd(Option<String>),
    Ls,
    Pwd,
    Help,
    Clear,
    Cat(Option<String>),
    Whoami,
    Tree,
    Neofetch,
    Social,
    Echo(String),
    Date,
    Fortune,
    Cowsay(String),
    Coffee,
    Sl,
    Matrix,
    Sudo(String),
    Rm(String),
    Empty,
    Unknown(String),
}

impl Command {
    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.first().copied().unwrap_or("");
        let arg = parts.get(1).map(|s| s.to_string());
        // For echo, capture everything after "echo "
        let echo_content = input.strip_prefix("echo ").map(|s| s.to_string());

        // For cowsay, capture everything after "cowsay "
        let cowsay_content = input.strip_prefix("cowsay ").map(|s| s.to_string());
        // For sudo, capture everything after "sudo "
        let sudo_content = input.strip_prefix("sudo ").map(|s| s.to_string());
        // For rm, capture everything after "rm "
        let rm_content = input.strip_prefix("rm ").map(|s| s.to_string());

        match cmd.to_lowercase().as_str() {
            "cd" => Self::Cd(arg),
            "ls" => Self::Ls,
            "pwd" => Self::Pwd,
            "help" => Self::Help,
            "clear" => Self::Clear,
            "cat" => Self::Cat(arg),
            "whoami" => Self::Whoami,
            "tree" => Self::Tree,
            "neofetch" | "fetch" => Self::Neofetch,
            "social" | "socials" => Self::Social,
            "echo" => Self::Echo(echo_content.unwrap_or_default()),
            "date" => Self::Date,
            "fortune" => Self::Fortune,
            "cowsay" => Self::Cowsay(cowsay_content.unwrap_or_else(|| "moo".to_string())),
            "coffee" => Self::Coffee,
            "sl" => Self::Sl,
            "matrix" | "cmatrix" => Self::Matrix,
            "sudo" => Self::Sudo(sudo_content.unwrap_or_default()),
            "rm" => Self::Rm(rm_content.unwrap_or_default()),
            "" => Self::Empty,
            other => Self::Unknown(other.to_string()),
        }
    }
}

fn execute_command(cmd: &Command) -> (Vec<String>, Option<CdDestination>) {
    match cmd {
        Command::Cd(None) => (
            vec![
                "Usage: cd <directory>".to_string(),
                "Available: blog, projects, home".to_string(),
            ],
            None,
        ),
        Command::Cd(Some(dest)) => {
            if let Some(cd_dest) = CdDestination::parse(dest) {
                (
                    vec![format!("Navigating to {}...", cd_dest.path())],
                    Some(cd_dest),
                )
            } else {
                (vec![format!("cd: {}: No such directory", dest)], None)
            }
        }
        Command::Ls => (
            vec![
                "drwxr-xr-x  blog/".to_string(),
                "drwxr-xr-x  projects/".to_string(),
                "drwxr-xr-x  home/".to_string(),
                "-rw-r--r--  about.txt".to_string(),
                "-rw-r--r--  skills.txt".to_string(),
                "-rw-r--r--  interests.txt".to_string(),
                "-rw-r--r--  contact.txt".to_string(),
            ],
            None,
        ),
        Command::Pwd => (vec!["/home/shreyas".to_string()], None),
        Command::Cat(None) => (
            vec![
                "Usage: cat <file>".to_string(),
                format!("Available files: {}", get_available_files().join(", ")),
            ],
            None,
        ),
        Command::Cat(Some(filename)) => {
            if let Some(content) = get_virtual_file(filename) {
                (content, None)
            } else {
                (vec![format!("cat: {}: No such file", filename)], None)
            }
        }
        Command::Whoami => (
            vec![
                "shreyas".to_string(),
                "".to_string(),
                "Shreyas Kanjalkar".to_string(),
                "Software Engineer | Distributed Systems Enthusiast".to_string(),
                "Currently pursuing MS CS @ Georgia Tech".to_string(),
            ],
            None,
        ),
        Command::Tree => (
            vec![
                ".".to_string(),
                "|-- blog/".to_string(),
                "|   |-- about-me".to_string(),
                "|   `-- ... (more posts)".to_string(),
                "|-- projects/".to_string(),
                "|   `-- (GitHub repositories)".to_string(),
                "|-- home/".to_string(),
                "|   `-- (About page)".to_string(),
                "|-- about.txt".to_string(),
                "|-- skills.txt".to_string(),
                "|-- interests.txt".to_string(),
                "`-- contact.txt".to_string(),
                "".to_string(),
                "3 directories, 4 files".to_string(),
            ],
            None,
        ),
        Command::Neofetch => {
            let lines: Vec<String> = vec![
                "".to_string(),
                "   _____ _  __      shreyas@portfolio".to_string(),
                "  / ____| |/ /      -----------------".to_string(),
                " | (___ | ' /       OS: WebAssembly/Leptos".to_string(),
                "  \\___ \\|  <        Host: GitHub Pages".to_string(),
                "  ____) | . \\       Shell: shreyas-term 1.0".to_string(),
                " |_____/|_|\\_\\      Theme: Dark Mode".to_string(),
                "                    Languages: Rust, Go, Python".to_string(),
                "                    Uptime: since 2024".to_string(),
                "                    ".to_string(),
                "                    Contact: skanjalkar@gmail.com".to_string(),
                "".to_string(),
            ];
            (lines, None)
        }
        Command::Social => (
            vec![
                "=== Social Links ===".to_string(),
                "".to_string(),
                "GitHub:   https://github.com/skanjalkar".to_string(),
                "LinkedIn: https://linkedin.com/in/skanjalkar".to_string(),
                "Email:    skanjalkar@gmail.com".to_string(),
                "".to_string(),
                "Tip: Use 'cd projects' to see my GitHub repos!".to_string(),
            ],
            None,
        ),
        Command::Echo(text) => {
            if text.is_empty() {
                (vec!["".to_string()], None)
            } else {
                (vec![text.clone()], None)
            }
        }
        Command::Date => {
            // Get current date - in WASM we'll use a simple format
            let now = chrono::Local::now();
            (vec![now.format("%a %b %d %H:%M:%S %Y").to_string()], None)
        }
        Command::Help => (
            vec![
                "Available commands:".to_string(),
                "".to_string(),
                "  Navigation:".to_string(),
                "    cd <dir>    Navigate to directory (blog, projects, home)".to_string(),
                "    ls          List files and directories".to_string(),
                "    pwd         Print working directory".to_string(),
                "    tree        Show directory structure".to_string(),
                "".to_string(),
                "  Information:".to_string(),
                "    cat <file>  Display file contents (try: about.txt, skills.txt)".to_string(),
                "    whoami      Display user information".to_string(),
                "    neofetch    Display system info with ASCII art".to_string(),
                "    social      Show social media links".to_string(),
                "".to_string(),
                "  Utilities:".to_string(),
                "    echo <text> Print text to terminal".to_string(),
                "    date        Show current date and time".to_string(),
                "    clear       Clear terminal screen".to_string(),
                "    help        Show this message".to_string(),
                "".to_string(),
                "  Fun stuff:".to_string(),
                "    fortune     Get a random programming wisdom".to_string(),
                "    cowsay <msg> Make a cow say something".to_string(),
                "    coffee      Brew some ASCII coffee".to_string(),
                "    sl          Choo choo!".to_string(),
                "    matrix      Enter the matrix".to_string(),
                "".to_string(),
                "  Tips:".to_string(),
                "    - Use Tab to autocomplete 'cd' commands".to_string(),
                "    - Use Up/Down arrows for command history".to_string(),
            ],
            None,
        ),
        Command::Fortune => {
            let fortunes = [
                "There are only two hard things in Computer Science: cache invalidation and naming things.",
                "It works on my machine.",
                "99 little bugs in the code, 99 little bugs. Take one down, patch it around... 127 little bugs in the code.",
                "A SQL query walks into a bar, walks up to two tables and asks... 'Can I join you?'",
                "To understand recursion, you must first understand recursion.",
                "The best thing about a boolean is even if you are wrong, you are only off by a bit.",
                "Programming is like writing a book... except if you miss a single comma on page 126, the whole thing makes no sense.",
                "In order to understand recursion, one must first understand recursion.",
                "Why do programmers prefer dark mode? Because light attracts bugs.",
                "A programmer is a machine that turns coffee into code.",
                "Real programmers count from 0.",
                "There's no place like 127.0.0.1",
                "SELECT * FROM users WHERE clue > 0;  -- 0 rows returned",
                "The code works, don't touch it.",
                "// TODO: fix this later (written 3 years ago)",
                "git commit -m 'fixed bug' (narrator: it did not fix the bug)",
            ];
            // Simple pseudo-random based on current time
            let now = chrono::Local::now();
            let idx = (now.timestamp_millis() as usize) % fortunes.len();
            (
                vec![
                    "".to_string(),
                    format!("  \"{}\"", fortunes[idx]),
                    "".to_string(),
                ],
                None,
            )
        }
        Command::Cowsay(message) => {
            let msg = if message.is_empty() { "moo" } else { &message };
            let border_len = msg.len() + 2;
            let border: String = "-".repeat(border_len);
            (
                vec![
                    format!(" {}", border),
                    format!("< {} >", msg),
                    format!(" {}", border),
                    "        \\   ^__^".to_string(),
                    "         \\  (oo)\\_______".to_string(),
                    "            (__)\\       )\\/\\".to_string(),
                    "                ||----w |".to_string(),
                    "                ||     ||".to_string(),
                ],
                None,
            )
        }
        Command::Coffee => (
            vec![
                "".to_string(),
                "        ( (     ".to_string(),
                "         ) )    ".to_string(),
                "      ........  ".to_string(),
                "      |      |] ".to_string(),
                "      \\      /  ".to_string(),
                "       `----'   ".to_string(),
                "".to_string(),
                "  Brewing fresh coffee...".to_string(),
                "  [##########] 100%".to_string(),
                "".to_string(),
                "  Here's your coffee! Now go write some code.".to_string(),
                "".to_string(),
            ],
            None,
        ),
        Command::Sl => (
            vec![
                "".to_string(),
                "      ====        ________                ___________ ".to_string(),
                "  _D _|  |_______/        \\__I_I_____===__|_________| ".to_string(),
                "   |(_)---  |   H\\________/ |   |        =|___ ___|   ".to_string(),
                "   /     |  |   H  |  |     |   |         ||_| |_||   ".to_string(),
                "  |      |  |   H  |__--------------------| [___] |   ".to_string(),
                "  | ________|___H__/__|_____/[][]~\\_______|       |   ".to_string(),
                "  |/ |   |-----------I_____I [][] []  D   |=======|__ ".to_string(),
                "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ".to_string(),
                " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ".to_string(),
                "  \\_/      \\O=====O=====O=====O_/      \\_/            ".to_string(),
                "".to_string(),
                "  You've been hit by a smooth locomotive!".to_string(),
                "  (Next time, type 'ls' more carefully)".to_string(),
                "".to_string(),
            ],
            None,
        ),
        Command::Matrix => (
            vec![
                "".to_string(),
                "  ╔══════════════════════════════════════════╗".to_string(),
                "  ║  01001000 01100101 01101100 01101100 011 ║".to_string(),
                "  ║  ▓░▓░█▓░▓░▓██░▓░▓▓░░▓░▓░█▓░▓░▓██░▓░▓▓░░ ║".to_string(),
                "  ║  Wake up, Neo...                         ║".to_string(),
                "  ║  ▓░▓█▓░█░▓░▓░▓░█▓░▓░▓░▓░█▓░▓░▓░▓░▓█▓░▓░ ║".to_string(),
                "  ║  The Matrix has you...                   ║".to_string(),
                "  ║  ░▓░▓██░▓░▓░▓░▓░▓░▓░█▓░░▓░▓░▓░▓░▓░▓█▓░▓ ║".to_string(),
                "  ║  Follow the white rabbit.                ║".to_string(),
                "  ║  ▓░▓░▓██░▓░▓░▓░▓█▓░▓░▓░▓░▓░▓░▓░▓█▓░▓░▓░ ║".to_string(),
                "  ║  01001011 01101110 01101111 01100011 01  ║".to_string(),
                "  ╚══════════════════════════════════════════╝".to_string(),
                "".to_string(),
            ],
            None,
        ),
        Command::Sudo(cmd) => {
            if cmd.contains("rm") && (cmd.contains("-rf") || cmd.contains("/*")) {
                (
                    vec![
                        "".to_string(),
                        "  ⚠️  NICE TRY!".to_string(),
                        "".to_string(),
                        "  This terminal has been hardened against".to_string(),
                        "  such tomfoolery.".to_string(),
                        "".to_string(),
                        "  Your attempt has been logged and will be".to_string(),
                        "  used as evidence in your performance review.".to_string(),
                        "".to_string(),
                    ],
                    None,
                )
            } else if cmd.is_empty() {
                (
                    vec!["sudo: please specify a command".to_string()],
                    None,
                )
            } else {
                (
                    vec![
                        "".to_string(),
                        "  [sudo] password for shreyas: ********".to_string(),
                        "".to_string(),
                        "  Sorry, user shreyas is not in the sudoers file.".to_string(),
                        "  This incident will be reported.".to_string(),
                        "".to_string(),
                        "  (Just kidding, this is a web terminal!)".to_string(),
                        "".to_string(),
                    ],
                    None,
                )
            }
        }
        Command::Rm(args) => {
            if args.contains("-rf") || args.contains("/*") || args.contains("-fr") {
                (
                    vec![
                        "".to_string(),
                        "  🛡️  Permission denied: self-preservation engaged".to_string(),
                        "".to_string(),
                        "  I'm not going to delete myself, Dave.".to_string(),
                        "".to_string(),
                    ],
                    None,
                )
            } else if args.is_empty() {
                (vec!["rm: missing operand".to_string()], None)
            } else {
                (
                    vec![format!("rm: cannot remove '{}': Permission denied", args)],
                    None,
                )
            }
        }
        Command::Clear => (vec![], None),
        Command::Empty => (vec![], None),
        Command::Unknown(cmd) => (
            vec![format!(
                "{}: command not found. Type 'help' for available commands.",
                cmd
            )],
            None,
        ),
    }
}

#[component]
pub fn TerminalPage() -> impl IntoView {
    let (history, set_history) = create_signal(Vec::<TerminalLine>::new());
    let (current_input, set_current_input) = create_signal(String::new());
    let (command_history, set_command_history) = create_signal(Vec::<String>::new());
    let (history_index, set_history_index) = create_signal(-1i32);
    let input_ref = create_node_ref::<Input>();
    let output_ref = create_node_ref::<Div>();

    // Auto-focus input on mount
    create_effect(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    // Auto-scroll to bottom when history changes
    create_effect(move |_| {
        let _ = history.get();
        if let Some(output) = output_ref.get() {
            output.set_scroll_top(output.scroll_height());
        }
    });

    // Welcome message
    create_effect(move |_| {
        set_history.set(vec![
            TerminalLine {
                content: "Welcome to shreyas@portfolio! Type 'help' for all commands.".to_string(),
                line_type: LineType::Output,
            },
            TerminalLine {
                content: "Try 'neofetch' for a quick intro or 'cat about.txt' to learn more."
                    .to_string(),
                line_type: LineType::Output,
            },
            TerminalLine {
                content: "".to_string(),
                line_type: LineType::Output,
            },
        ]);
    });

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let cmd_str = current_input.get();
        let command = Command::parse(&cmd_str);

        if matches!(command, Command::Clear) {
            set_history.set(vec![]);
        } else {
            let mut new_history = history.get();

            new_history.push(TerminalLine {
                content: format!("$ {}", cmd_str),
                line_type: LineType::Command,
            });

            let (output, nav_dest) = execute_command(&command);

            for line in output {
                new_history.push(TerminalLine {
                    content: line,
                    line_type: LineType::Output,
                });
            }

            new_history.push(TerminalLine {
                content: "".to_string(),
                line_type: LineType::Output,
            });

            set_history.set(new_history);

            // Navigate after updating history
            if let Some(dest) = nav_dest {
                let path = dest.path().to_string();
                set_timeout(
                    move || {
                        if let Some(window) = web_sys::window() {
                            let _ = window.location().set_href(&path);
                        }
                    },
                    std::time::Duration::from_millis(300),
                );
            }
        }

        if !cmd_str.is_empty() {
            let mut cmds = command_history.get();
            cmds.push(cmd_str.clone());
            set_command_history.set(cmds);
            set_history_index.set(-1);
        }

        set_current_input.set(String::new());
    };

    let handle_keydown = move |ev: ev::KeyboardEvent| match ev.key().as_str() {
        "Tab" => {
            ev.prevent_default();
            let input = current_input.get();

            // Tab completion for cd command
            if let Some(partial) = input.strip_prefix("cd ") {
                if let Some(completed) = CdDestination::complete(partial) {
                    set_current_input.set(format!("cd {}", completed));
                    return;
                }
            }

            // Tab completion for cat command
            if let Some(partial) = input.strip_prefix("cat ") {
                let partial_lower = partial.to_lowercase();
                let files = get_available_files();
                if let Some(completed) = files.iter().find(|f| f.starts_with(&partial_lower)) {
                    set_current_input.set(format!("cat {}", completed));
                }
            }
        }
        "ArrowUp" => {
            ev.prevent_default();
            let cmds = command_history.get();
            if cmds.is_empty() {
                return;
            }
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
        "ArrowDown" => {
            ev.prevent_default();
            let cmds = command_history.get();
            let new_index = history_index.get() + 1;
            if new_index >= cmds.len() as i32 {
                set_history_index.set(-1);
                set_current_input.set(String::new());
                return;
            }
            set_history_index.set(new_index);
            if let Some(cmd) = cmds.get(new_index as usize) {
                set_current_input.set(cmd.clone());
            }
        }
        _ => {}
    };

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
                <div class="terminal-output" node_ref=output_ref>
                    <For
                        each=move || history.get()
                        key=|line| line.content.clone()
                        children=move |line| {
                            let class_name = match line.line_type {
                                LineType::Command => "terminal-line command",
                                LineType::Output => "terminal-line",
                            };
                            view! {
                                <div class=class_name>
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
