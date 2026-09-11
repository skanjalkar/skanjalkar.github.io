use crate::content::{posts, INTERESTS, INTRO, JOURNEY, PROJECTS};
use crate::models::Profile;
use crate::state::AppState;
use leptos::html::{Div, Input};
use leptos::*;
use leptos_router::*;

#[derive(Clone)]
enum Output {
    Text(String),
    Link(String, String),
    Image(String, String),
}

#[derive(Clone)]
struct Entry {
    command: String,
    output: Vec<Output>,
}

#[derive(Clone, Default)]
pub struct TerminalSession {
    entries: Vec<Entry>,
    commands: Vec<String>,
    input: String,
    history_index: Option<usize>,
    draft: String,
}

fn text(value: impl Into<String>) -> Output {
    Output::Text(value.into())
}

fn link(label: impl Into<String>, url: impl Into<String>) -> Output {
    Output::Link(label.into(), url.into())
}

fn command_output(input: &str) -> Vec<Output> {
    let input = input.trim();
    let (command, arg) = input.split_once(char::is_whitespace).unwrap_or((input, ""));
    let arg = arg.trim();
    let profile = Profile::default();
    match command.to_lowercase().as_str() {
        "help" => vec![text("Explore\n  about / whoami      Meet Shreyas\n  projects            Browse the workbench\n  open <project>      Read a project (try: open aries)\n  blog                List writing\n  read <post>         Read a post (try: read about-me)\n  contact / social    Find me elsewhere\n  resume              Open my résumé\n  interests / spiky   Off the clock\n\nNavigate\n  ls / tree           What’s here\n  cd <section>        Read home, projects, blog, or about here\n  cat <file>          Read about.txt, skills.txt, interests.txt, contact.txt\n  browse / exit       Return to your last visual page\n\nUtilities\n  clear / history     Clear output / show command history\n  echo <text> / date  Small familiar comforts\n  neofetch            About this workshop\n\nJust for fun\n  coffee / fortune / cowsay <text> / sl / matrix\n\nUse ↑ and ↓ for history. Tab completes a command; Escape then Tab leaves the input.\nYour session stays here while you switch between Browse and Terminal.")],
        "about" | "whoami" | "home" => {
            let mut output = vec![text(INTRO), text("THE PATH HERE")];
            output.extend(JOURNEY.iter().map(|(field, place)| text(format!("{field} → {place}"))));
            output.push(text(INTERESTS));
            output
        }
        "projects" => {
            let mut output = vec![text("THE WORKBENCH")];
            for project in PROJECTS {
                output.push(text(format!("{} — {}\n{}", project.slug, project.name, project.summary)));
                output.push(link(format!("open {} →", project.slug), format!("/terminal?command=open%20{}", project.slug)));
            }
            output.push(link("All repositories ↗", "https://github.com/skanjalkar?tab=repositories"));
            output
        }
        "open" => PROJECTS.iter().find(|p| p.slug == arg || p.repository == arg).map(|p| vec![
            text(format!("{}\n{} · {}", p.name, p.category, p.language)),
            text(p.summary), text(p.detail), link("Explore the code ↗", p.url()),
            link("View visual page →", format!("/projects/{}", p.slug)),
        ]).unwrap_or_else(|| vec![text("Project not found. Type projects to see the available names.")]),
        "blog" | "writing" => posts().into_iter().flat_map(|post| vec![
            text(format!("{} — {} ({})\n{}", post.id, post.title, post.date, post.summary)),
            link(format!("read {} →", post.id), format!("/terminal?command=read%20{}", post.id)),
        ]).collect(),
        "read" => posts().into_iter().find(|p| p.id == arg).map(|post| {
            let mut output = vec![text(format!("{} · {}\nAn entry from the archive; details reflect the time it was written.", post.title, post.date))];
            output.extend(post.content.into_iter().filter_map(|item| match item.content_type.as_str() {
                "paragraph" => item.text.map(text),
                "image" => item.src.map(|src| Output::Image(src, item.alt.unwrap_or_default())),
                _ => None,
            }));
            output.push(link("View visual page →", format!("/blog/{}", post.id)));
            output
        }).unwrap_or_else(|| vec![text("Post not found. Type blog to see the available posts.")]),
        "contact" | "social" | "socials" => vec![
            link("Email ↗", format!("mailto:{}", profile.email)),
            link("GitHub ↗", profile.github_url),
            link("LinkedIn ↗", profile.linkedin_url.unwrap_or_default()),
        ],
        "resume" => vec![link("Read my résumé (PDF) ↗", profile.resume_url.unwrap_or_default())],
        "interests" => vec![text(INTERESTS)],
        "spiky" => vec![text("Meet Spiky. An important part of the story."), Output::Image("/static/blog/about-me/img_0.jpeg".into(), "Spiky, Shreyas’s dog".into())],
        "skills" => vec![text("My work explores distributed systems, database storage, and robotics.\nThe workbench includes Rust, Go, and Python projects. Type projects to explore.")],
        "ls" | "tree" => vec![text("~/workshop\n├── about.txt\n├── skills.txt\n├── interests.txt\n├── contact.txt\n├── projects/\n│   ├── aries\n│   ├── distributed-systems\n│   ├── watchman\n│   └── workshop\n└── blog/\n    └── about-me\n\nTry: open aries, read about-me, or cat about.txt")],
        "pwd" => vec![text("/home/shreyas/workshop")],
        "cat" => match arg.trim_end_matches(".txt") {
            name @ ("about" | "skills" | "interests" | "contact") => command_output(name),
            _ => vec![text("File not found. Try about.txt, skills.txt, interests.txt, or contact.txt.")],
        },
        "cd" => match arg.trim_matches('/') {
            "" | "~" | "home" | "about" => command_output("about"),
            "projects" => command_output("projects"),
            "blog" => command_output("blog"),
            _ => vec![text("Section not found. Try cd projects, cd blog, or cd home.")],
        },
        "neofetch" | "fetch" => vec![text(format!("sk.  Shreyas’s workshop\n──────────────────────\nHost     GitHub Pages\nBuilt    Rust + Leptos + WebAssembly\nHome     {}\nModes    Browse / Terminal\n\n{}", profile.location, INTRO))],
        "echo" => vec![text(arg)],
        "date" => vec![text(js_sys::Date::new_0().to_utc_string().as_string().unwrap_or_default())],
        "coffee" => vec![text("   ( (\n    ) )\n  .-----._\n  |     | )\n  |     |/\n  '-----'\n\nA little coffee for your curiosity.")],
        "cowsay" => {
            let message = if arg.is_empty() { "hello, world" } else { arg };
            vec![text(format!("< {message} >\n        \\   ^__^\n         \\  (oo)\\_______\n            (__)\\       )\\/\\\n                ||----w |\n                ||     ||"))]
        }
        "fortune" => vec![text("There are only two hard things in Computer Science: cache invalidation and naming things.")],
        "sl" => vec![text("     ___      ______\n  __|   |____|_[]_[]|\n |__        _______|\n    O------O   O--O\n\nThe train has arrived. Were you looking for ls?")],
        "matrix" | "cmatrix" => vec![text("01001000 01100101 01101100 01101100 01101111\n\nFollow your curiosity. The rabbit can wait.")],
        "sudo" | "rm" => vec![text("This is a portfolio terminal. No system commands are run and no files are changed.\nTry help to see what you can explore.")],
        "" => Vec::new(),
        _ => vec![text(format!("Command not found: {command}. Try help, or use one of the buttons above."))],
    }
}

fn completions(input: &str) -> Vec<String> {
    let commands = [
        "about",
        "projects",
        "open",
        "blog",
        "read",
        "contact",
        "resume",
        "interests",
        "spiky",
        "help",
        "ls",
        "tree",
        "pwd",
        "cd",
        "cat",
        "whoami",
        "skills",
        "social",
        "clear",
        "history",
        "browse",
        "exit",
        "echo",
        "date",
        "neofetch",
        "coffee",
        "fortune",
        "cowsay",
        "sl",
        "matrix",
        "sudo",
        "rm",
    ];
    let (prefix, partial, values): (String, &str, Vec<String>) =
        if let Some((cmd, arg)) = input.split_once(' ') {
            let values = match cmd {
                "open" => PROJECTS.iter().map(|p| p.slug.to_string()).collect(),
                "read" => posts().into_iter().map(|p| p.id).collect(),
                "cd" => ["home", "about", "projects", "blog"]
                    .map(str::to_string)
                    .to_vec(),
                "cat" => ["about.txt", "skills.txt", "interests.txt", "contact.txt"]
                    .map(str::to_string)
                    .to_vec(),
                _ => Vec::new(),
            };
            (format!("{cmd} "), arg, values)
        } else {
            (String::new(), input, commands.map(str::to_string).to_vec())
        };
    values
        .into_iter()
        .filter(|v| v.starts_with(partial))
        .map(|v| format!("{prefix}{v}"))
        .collect()
}

#[component]
pub fn TerminalPage() -> impl IntoView {
    let state = expect_context::<AppState>();
    let session = state.terminal;
    let navigate = use_navigate();
    let query = use_query_map();
    let input_ref = create_node_ref::<Input>();
    let output_ref = create_node_ref::<Div>();
    let release_tab = create_rw_signal(false);
    let suggestions = create_rw_signal(Vec::<String>::new());
    let run = store_value(move |command: String| {
        let command = command.trim().to_string();
        if command.is_empty() {
            return;
        }
        if command == "browse" || command == "exit" {
            session.update(|s| {
                s.input.clear();
                s.history_index = None;
            });
            navigate(&state.browse_path.get_untracked(), Default::default());
            return;
        }
        session.update(|s| {
            let output = if command == "history" {
                vec![text(
                    s.commands
                        .iter()
                        .enumerate()
                        .map(|(i, command)| format!("{}  {}", i + 1, command))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )]
            } else if command == "clear" {
                s.entries.clear();
                Vec::new()
            } else {
                command_output(&command)
            };
            s.commands.push(command.clone());
            if command != "clear" {
                s.entries.push(Entry { command, output });
            }
            s.input.clear();
            s.draft.clear();
            s.history_index = None;
        });
        suggestions.set(Vec::new());
    });

    let consume_command = use_navigate();
    create_effect(move |_| {
        if let Some(command) = query.with(|q| q.get("command").cloned()) {
            run.with_value(|run| run(command));
            consume_command(
                "/terminal",
                NavigateOptions {
                    replace: true,
                    scroll: false,
                    ..Default::default()
                },
            );
        }
    });

    let entry_count = create_memo(move |_| session.with(|s| s.entries.len()));
    create_effect(move |_| {
        entry_count.get();
        request_animation_frame(move || {
            if let Some(output) = output_ref.get() {
                if let Some(entry) = output.last_element_child() {
                    let top = entry.get_bounding_client_rect().top()
                        - output.get_bounding_client_rect().top()
                        + f64::from(output.scroll_top());
                    output.set_scroll_top(top as i32);
                }
            }
        });
    });

    let keydown = move |ev: ev::KeyboardEvent| match ev.key().as_str() {
        "Escape" => {
            release_tab.set(true);
            suggestions.set(Vec::new());
        }
        "Tab" if !ev.shift_key() && !release_tab.get_untracked() => {
            let matches = completions(&session.get_untracked().input);
            if !matches.is_empty() {
                ev.prevent_default();
                if matches.len() == 1 {
                    session.update(|s| s.input = matches[0].clone());
                    suggestions.set(Vec::new());
                } else {
                    suggestions.set(matches);
                }
            }
        }
        "ArrowUp" | "ArrowDown" => {
            ev.prevent_default();
            session.update(|s| {
                if s.commands.is_empty() {
                    return;
                }
                if ev.key() == "ArrowUp" {
                    if s.history_index.is_none() {
                        s.draft = s.input.clone();
                    }
                    let index = s
                        .history_index
                        .map(|i| i.saturating_sub(1))
                        .unwrap_or(s.commands.len() - 1);
                    s.history_index = Some(index);
                    s.input = s.commands[index].clone();
                } else if let Some(index) = s.history_index {
                    if index + 1 < s.commands.len() {
                        s.history_index = Some(index + 1);
                        s.input = s.commands[index + 1].clone();
                    } else {
                        s.history_index = None;
                        s.input = s.draft.clone();
                    }
                }
            });
        }
        _ => release_tab.set(false),
    };

    view! {
        <section class="terminal-page" aria-labelledby="terminal-heading">
            <div class="section-heading">
                <div><p class="eyebrow">"SAME WORKSHOP. DIFFERENT ENTRANCE."</p><h1 id="terminal-heading">"Make yourself at "<em>"~/home."</em></h1></div>
                <A href=move || state.browse_path.get() class="text-link">"← Back to browsing"</A>
            </div>
            <div class="terminal-window">
                <div class="terminal-bar"><span class="terminal-lights" aria-hidden="true">"● ● ●"</span><span>"shreyas / workshop"</span><span>"a place to explore"</span></div>
                <div class="terminal-welcome"><strong>"Hello, curious human."</strong><p>"Read, explore, follow a rabbit hole. Type a command or choose one below."</p></div>
                <div class="command-chips" aria-label="Suggested commands">
                    {["about", "projects", "blog", "contact", "spiky", "help"].into_iter().map(|command| view! {
                        <button type="button" on:click=move |_| run.with_value(|run| run(command.to_string()))>{command}</button>
                    }).collect_view()}
                </div>
                <div class="terminal-output" node_ref=output_ref role="log" aria-label="Terminal output" aria-live="polite" aria-relevant="additions" tabindex="0">
                    <For each=move || session.with(|s| s.entries.clone().into_iter().enumerate().collect::<Vec<_>>()) key=|(index, _)| *index children=move |(_, entry)| view! {
                        <div class="terminal-entry">
                            <p class="terminal-command"><span aria-hidden="true">"❯ "</span>{entry.command}</p>
                            {entry.output.into_iter().map(|output| match output {
                                Output::Text(value) => view! { <p class="terminal-text">{value}</p> }.into_view(),
                                Output::Link(label, url) => {
                                    let relation = if url.ends_with(".pdf") { "external" } else { "" };
                                    view! { <p class="terminal-link"><A href=url attr:rel=relation>{label}</A></p> }.into_view()
                                },
                                Output::Image(src, alt) => view! { <img class="terminal-image" src=src alt=alt loading="lazy"/> }.into_view(),
                            }).collect_view()}
                        </div>
                    }/>
                </div>
                <form class="terminal-form" on:submit=move |ev| {
                    ev.prevent_default();
                    run.with_value(|run| run(session.get_untracked().input));
                }>
                    <label for="command-input"><span aria-hidden="true">"❯"</span><span class="sr-only">"Terminal command"</span></label>
                    <input id="command-input" node_ref=input_ref prop:value=move || session.with(|s| s.input.clone()) on:input=move |ev| {
                        session.update(|s| s.input = event_target_value(&ev));
                        suggestions.set(Vec::new());
                    } on:keydown=keydown autocomplete="off" autocapitalize="off" spellcheck="false" placeholder="Try projects or help" aria-describedby="terminal-hint"/>
                    <button type="submit">"Run ↵"</button>
                </form>
                <div class="command-chips" hidden=move || suggestions.get().is_empty()>
                    {move || suggestions.get().into_iter().map(|suggestion| {
                        let label = suggestion.clone();
                        view! { <button type="button" on:click=move |_| {
                            session.update(|s| s.input = suggestion.clone());
                            suggestions.set(Vec::new());
                            if let Some(input) = input_ref.get() { let _ = input.focus(); }
                        }>{label}</button> }
                    }).collect_view()}
                </div>
                <p id="terminal-hint" class="terminal-hint">"Tab to complete · ↑ ↓ history · Escape then Tab to move on · No real shell commands are executed"</p>
            </div>
        </section>
    }
}
