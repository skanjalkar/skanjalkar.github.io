use leptos::html::{Div, Input};
use leptos::*;
use leptos_router::use_navigate;

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
            Self::Home => "/",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    Cd(Option<String>),
    Ls,
    Pwd,
    Help,
    Clear,
    Empty,
    Unknown(String),
}

impl Command {
    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.first().copied().unwrap_or("");
        let arg = parts.get(1).map(|s| s.to_string());

        match cmd.to_lowercase().as_str() {
            "cd" => Self::Cd(arg),
            "ls" => Self::Ls,
            "pwd" => Self::Pwd,
            "help" => Self::Help,
            "clear" => Self::Clear,
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
            ],
            None,
        ),
        Command::Pwd => (vec!["/home/shreyas".to_string()], None),
        Command::Help => (
            vec![
                "Available commands:".to_string(),
                "  cd <dir>  - Navigate to directory (blog, projects, home)".to_string(),
                "  ls        - List directories".to_string(),
                "  pwd       - Print working directory".to_string(),
                "  clear     - Clear terminal".to_string(),
                "  help      - Show this message".to_string(),
            ],
            None,
        ),
        Command::Clear => (vec![], None),
        Command::Empty => (vec![], None),
        Command::Unknown(cmd) => (
            vec![format!("{}: command not found. Type 'help' for available commands.", cmd)],
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
    let navigate = use_navigate();

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
                content: "Welcome! Type 'help' for commands or 'ls' to see directories.".to_string(),
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
                let navigate = navigate.clone();
                set_timeout(
                    move || {
                        navigate(&path, Default::default());
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

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        match ev.key().as_str() {
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
        }
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
