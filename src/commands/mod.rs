pub mod filesystem;
pub mod registry;

use crate::{filesystem::model::VirtualFs, parser::tokenizer::ParsedCommand};
use registry::all_commands;

pub struct CommandOutput {
    pub lines: Vec<(String, LineStyle)>,
    pub clear_screen: bool,
    pub typewriter: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub enum LineStyle {
    Normal,
    Accent,
    Error,
    Muted,
}

pub fn dispatch(cmd: &ParsedCommand, fs: &VirtualFs, cwd: &mut String) -> CommandOutput {
    match cmd.command.as_str() {
        "help" => help_command(),
        "about" => simple_output(vec![
            ("", LineStyle::Normal),
            (
                "  Hey! I'm Pruthvi Parne, a Senior Software Engineer.",
                LineStyle::Accent,
            ),
            (
                "  Full-stack developer with mobile experience.",
                LineStyle::Normal,
            ),
            ("  Passionate about building things.", LineStyle::Normal),
            ("", LineStyle::Normal),
            ("  Type 'skills' to see what I work with,", LineStyle::Muted),
            ("  or 'projects' to see what I've built.", LineStyle::Muted),
            ("", LineStyle::Normal),
        ]),
        "skills" => simple_output(vec![
            ("", LineStyle::Normal),
            (
                "  Languages:   Rust, TypeScript, Python, Swift",
                LineStyle::Normal,
            ),
            ("  Frontend:    React, Leptos, Next.js", LineStyle::Normal),
            (
                "  Mobile:      iOS (Swift), React Native",
                LineStyle::Normal,
            ),
            ("  Backend:     Node.js, Rust, Python", LineStyle::Normal),
            ("  Tools:       Git, Docker, AWS, CI/CD", LineStyle::Normal),
            ("", LineStyle::Normal),
        ]),
        "projects" => simple_output(vec![
            ("", LineStyle::Normal),
            ("  1. Terminal Portfolio", LineStyle::Accent),
            (
                "     A terminal-style portfolio built with Rust + Leptos + WASM",
                LineStyle::Muted,
            ),
            ("", LineStyle::Normal),
            ("  2. CNH mobile app", LineStyle::Accent),
            (
                "     Native iOS application for ag-tech company Case-new Holand",
                LineStyle::Muted,
            ),
            ("", LineStyle::Normal),
            ("  3. Full stack development for Corteva", LineStyle::Accent),
            (
                "     Build a frontend application for Corteva/Granular called Fungicide Timing.",
                LineStyle::Muted,
            ),
            ("", LineStyle::Normal),
        ]),
        "contact" => simple_output(vec![
            ("", LineStyle::Normal),
            ("  Email:    iamparne@email.com", LineStyle::Normal),
            ("  GitHub:   github.com/Superrehtard", LineStyle::Accent),
            ("  LinkedIn: linkedin.com/in/iamparne", LineStyle::Accent),
            ("", LineStyle::Normal),
        ]),
        "resume" => simple_output(vec![
            ("", LineStyle::Normal),
            ("  Resume download coming soon!", LineStyle::Normal),
            (
                "  For now, type 'about', 'skills', or 'projects'.",
                LineStyle::Muted,
            ),
            ("", LineStyle::Normal),
        ]),
        "clear" => CommandOutput {
            lines: vec![],
            clear_screen: true,
            typewriter: false,
        },
        "" => CommandOutput {
            lines: vec![],
            clear_screen: false,
            typewriter: false,
        },
        "pwd" => filesystem::pwd(cwd),
        "cd" => filesystem::cd(fs, cwd, &cmd.args),
        "ls" => filesystem::ls(fs, cwd, &cmd.args),
        "cat" => filesystem::cat(fs, cwd, &cmd.args),
        "tree" => filesystem::tree(fs, cwd, &cmd.args),
        unknown => simple_output(vec![
            (
                &format!("  Command not found: {}", unknown),
                LineStyle::Error,
            ),
            ("  Type 'help' for available commands.", LineStyle::Muted),
        ]),
    }
}

fn help_command() -> CommandOutput {
    let mut lines: Vec<(String, LineStyle)> = vec![
        (String::new(), LineStyle::Normal),
        ("  Available commands:".to_string(), LineStyle::Accent),
        (String::new(), LineStyle::Normal),
    ];

    for cmd in all_commands().iter().filter(|c| !c.hidden) {
        lines.push((
            format!("   {:<12} {}", cmd.name, cmd.description),
            LineStyle::Normal,
        ));
    }

    lines.push((String::new(), LineStyle::Normal));

    CommandOutput {
        lines,
        clear_screen: false,
        typewriter: true,
    }
}

fn simple_output(content: Vec<(&str, LineStyle)>) -> CommandOutput {
    CommandOutput {
        lines: content
            .into_iter()
            .map(|(text, style)| (text.to_string(), style))
            .collect(),
        clear_screen: false,
        typewriter: true,
    }
}
