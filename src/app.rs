use crate::commands::{self, LineStyle};
use crate::components::typewriter::TypewriterLine;
use crate::components::welcome::WelcomeBanner;
use crate::filesystem::builder;
use crate::parser::tokenizer;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Clone)]
struct OutputBlock {
    id: u32,
    command: String,
    lines: Vec<(String, LineStyle)>,
    animate: bool,
}

#[component]
pub fn App() -> impl IntoView {
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let output_ref = NodeRef::<leptos::html::Div>::new();

    // Reactive signals - these are like react's useState
    // RwSignal means "readable and writable signal"
    let (output, set_output) = signal(Vec::<OutputBlock>::new());
    let (input_value, set_input_value) = signal(String::new());
    let (history, set_history) = signal(Vec::<String>::new());
    let (history_idx, set_history_idx) = signal::<Option<usize>>(None);
    let cwd = RwSignal::new("~".to_string());
    let fs = RwSignal::new(builder::build_default_fs());

    Effect::new(move || {
        output.track();

        if let Some(el) = output_ref.get() {
            request_animation_frame(move || {
                el.set_scroll_top(el.scroll_height());
            });
        }
    });

    // This runs when the user presses Enter
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => {
                let cmd = input_value.get();
                let mut current_cwd = cwd.get();
                let parsed = tokenizer::parse(&cmd);
                let result = commands::dispatch(&parsed, &fs.get(), &mut current_cwd);
                cwd.set(current_cwd);

                if result.clear_screen {
                    set_output.set(vec![]);
                } else if !parsed.command.is_empty() || !result.lines.is_empty() {
                    // update the output signal by pushing a new entry
                    set_output.update(|out| {
                        for block in out.iter_mut() {
                            block.animate = false;
                        }
                        let id = out.len() as u32;
                        out.push(OutputBlock {
                            id,
                            command: cmd.trim().to_string(),
                            lines: result.lines,
                            animate: result.typewriter,
                        })
                    });
                }

                // push to history (only non-empty commands)
                let trimmed = cmd.trim().to_string();
                if !trimmed.is_empty() {
                    set_history.update(|h| h.push(trimmed));
                }

                // Rest history index
                set_history_idx.set(None);
                set_input_value.set(String::new());
            }
            "ArrowUp" => {
                ev.prevent_default();
                let h = history.get();
                if h.is_empty() {
                    return;
                }
                let new_idx = match history_idx.get() {
                    None => h.len() - 1,
                    Some(i) => {
                        if i > 0 {
                            i - 1
                        } else {
                            0
                        }
                    }
                };
                set_history_idx.set(Some(new_idx));
                set_input_value.set(h[new_idx].clone());
            }
            "ArrowDown" => {
                ev.prevent_default();
                let h = history.get();
                match history_idx.get() {
                    None => {}
                    Some(i) => {
                        if i + 1 < h.len() {
                            set_history_idx.set(Some(i + 1));
                            set_input_value.set(h[i + 1].clone());
                        } else {
                            set_history_idx.set(None);
                            set_input_value.set(String::new());
                        }
                    }
                }
            }
            "Tab" => {
                ev.prevent_default();
                let current = input_value.get();
                let trimmed = current.trim();

                // If input has no spaces, complete command names
                // If input has spaces, complete file paths
                if !trimmed.contains(' ') {
                    // Command completion
                    let matches: Vec<String> = commands::registry::all_commands()
                        .iter()
                        .filter(|c| c.name.starts_with(trimmed))
                        .map(|c| c.name.to_string())
                        .collect();

                    if matches.len() == 1 {
                        set_input_value.set(format!("{} ", matches[0]));
                    }
                } else {
                    // Path completion
                    let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        let partial = parts[1];
                        let current_fs = fs.get();
                        let current_cwd = cwd.get();

                        // Figure out the directory and partial name to match
                        let (dir_path, name_prefix) = if partial.contains('/') {
                            let last_slash = partial.rfind('/').unwrap();
                            let dir_part = &partial[..=last_slash];
                            let name_part = &partial[last_slash + 1..];
                            (
                                current_fs.resolve_path(&current_cwd, dir_part),
                                name_part.to_string(),
                            )
                        } else {
                            (current_cwd.clone(), partial.to_string())
                        };

                        // Get matching children
                        if let Some(entries) = current_fs.ls(&dir_path) {
                            let matches: Vec<String> = entries
                                .iter()
                                .filter(|(name, _)| name.starts_with(&name_prefix))
                                .map(|(name, is_dir)| {
                                    if *is_dir {
                                        format!("{}/", name)
                                    } else {
                                        name.clone()
                                    }
                                })
                                .collect();

                            if matches.len() == 1 {
                                let completed = if partial.contains('/') {
                                    let last_slash = partial.rfind('/').unwrap();
                                    format!("{}{}", &partial[..=last_slash], matches[0])
                                } else {
                                    matches[0].clone()
                                };
                                set_input_value.set(format!("{} {}", parts[0], completed));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    };

    let line_class = |style: &LineStyle| match style {
        LineStyle::Normal => "line-normal",
        LineStyle::Accent => "line-accent",
        LineStyle::Error => "line-error",
        LineStyle::Muted => "line-muted",
    };

    let focus_input = move |_| {
        if let Some(el) = input_ref.get() {
            let _ = el.focus();
        }
    };

    view! {
        <div class="terminal-wrapper" on:click=focus_input>
            <div class="terminal">
                <div class="terminal-output" node_ref=output_ref>
                    <WelcomeBanner />
                    {move || output.get().into_iter().map(|block| {
                        let lines = block.lines.clone();
                        let should_animate = block.animate;
                        view! {
                            <div>
                                <div class="output-command">
                                    <span class="prompt-user">"visitor"</span>
                                    <span class="prompt-at">"@"</span>
                                    <span class="prompt-host">"portfolio"</span>
                                    <span class="prompt-colon">":"</span>
                                    <span class="prompt-path">"~"</span>
                                    <span class="prompt-dollar">"$ "</span>
                                    {block.command}
                                </div>
                                <div>
                                    {
                                        lines.into_iter().map(|(text, style)| {
                                            let class = line_class(&style);
                                            view! {
                                                <div class=class>
                                                    <TypewriterLine text=text animate=should_animate />
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()
                                    }
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <div class="input-line">
                    <span class="prompt-user">"visitor"</span>
                    <span class="prompt-at">"@"</span>
                    <span class="prompt-host">"portfolio"</span>
                    <span class="prompt-colon">":"</span>
                    <span class="prompt-path">{move || cwd.get()}</span>
                    <span class="prompt-dollar">"$ "</span>
                    <input
                        type="text"
                        class="terminal-input"
                        node_ref=input_ref
                        prop:value=move || input_value.get()
                        on:input=move |ev| {
                            set_input_value.set(event_target_value(&ev));
                        }
                        on:keydown=on_keydown
                        autofocus=true
                        autocomplete="off"
                        spellcheck="false"
                    />
                </div>
            </div>
        </div>
    }
}

fn request_animation_frame(f: impl FnOnce() + 'static) {
    let cb = wasm_bindgen::closure::Closure::once_into_js(f);
    web_sys::window()
        .unwrap()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .unwrap();
}
