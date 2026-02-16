pub mod easter_eggs;
pub mod filesystem;
pub mod games;
pub mod registry;
pub mod theme;

use crate::filesystem::content;
use crate::{
    components::typewriter, filesystem::model::VirtualFs, parser::tokenizer::ParsedCommand,
    state::theme::ThemeState,
};
use registry::all_commands;

pub struct CommandOutput {
    pub lines: Vec<(String, LineStyle)>,
    pub clear_screen: bool,
    pub typewriter: bool,
    pub start_game: Option<games::ActiveGame>,
}

impl CommandOutput {
    pub fn new(lines: Vec<(String, LineStyle)>, clear_screen: bool, typewriter: bool) -> Self {
        Self {
            lines,
            clear_screen,
            typewriter,
            start_game: None,
        }
    }

    pub fn with_game(mut self, game: games::ActiveGame) -> Self {
        self.start_game = Some(game);
        self
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum LineStyle {
    Normal,
    Accent,
    Error,
    Muted,
}

pub fn dispatch(
    cmd: &ParsedCommand,
    fs: &VirtualFs,
    cwd: &mut String,
    theme: &ThemeState,
    history: &[String],
) -> CommandOutput {
    match cmd.command.as_str() {
        "help" => help_command(),
        "about" => content_output(content::CMD_ABOUT),
        "skills" => content_output(content::CMD_SKILLS),
        "projects" => content_output(content::CMD_PROJECTS),
        "contact" => content_output(content::CMD_CONTACT),
        "resume" => content_output(content::CMD_RESUME),
        "clear" => CommandOutput::new(vec![], true, false),
        "" => CommandOutput::new(vec![], false, false),
        "pwd" => filesystem::pwd(cwd),
        "cd" => filesystem::cd(fs, cwd, &cmd.args),
        "ls" => filesystem::ls(fs, cwd, &cmd.args),
        "cat" => filesystem::cat(fs, cwd, &cmd.args),
        "tree" => filesystem::tree(fs, cwd, &cmd.args),
        "theme" => theme::theme_command(&cmd.args, theme),
        "sudo" => easter_eggs::sudo(&cmd.args),
        "rm" => {
            // Check if args contain "-rf" patterns
            let args_str = cmd.args.join(" ");
            if args_str.contains("-rf") || args_str.contains("-r") {
                easter_eggs::rm_rf()
            } else {
                simple_output(vec![("  rm: missing operand", LineStyle::Error)])
            }
        }
        "neofetch" => easter_eggs::neofetch(),
        "cowsay" => easter_eggs::cowsay(&cmd.args),
        "whoami" => easter_eggs::whoami(),
        "date" => easter_eggs::date(),
        "echo" => easter_eggs::echo(&cmd.args),
        "exit" | "quit" | "logout" => easter_eggs::exit(),
        "history" => easter_eggs::history_cmd(&history),
        "ttt" | "tictactoe" => {
            let game = games::tictactoe::TicTacToe::new();
            let output = games::tictactoe::TicTacToe::start_output();
            CommandOutput {
                lines: output.lines,
                clear_screen: false,
                typewriter: false,
                start_game: Some(games::ActiveGame::TicTacToe(game)),
            }
        }
        "typing" => {
            let test = games::typing_test::TypingTest::new();
            let output = test.start_output().lines;
            CommandOutput {
                lines: output,
                clear_screen: false,
                typewriter: false,
                start_game: Some(games::ActiveGame::TypingTest(test)),
            }
        }
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
        ("Available commands:".to_string(), LineStyle::Accent),
        (String::new(), LineStyle::Normal),
    ];

    for cmd in all_commands().iter().filter(|c| !c.hidden) {
        lines.push((
            format!("{:<12} {}", cmd.name, cmd.description),
            LineStyle::Normal,
        ));
    }

    lines.push((String::new(), LineStyle::Normal));

    CommandOutput {
        lines,
        clear_screen: false,
        typewriter: false,
        start_game: None,
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
        start_game: None,
    }
}

fn content_output(content: &[(&str, bool)]) -> CommandOutput {
    CommandOutput {
        lines: content
            .iter()
            .map(|(text, is_accent)| {
                let style = if *is_accent {
                    LineStyle::Accent
                } else if text.trim().is_empty() {
                    LineStyle::Normal
                } else if text.trim_start().starts_with("Run ")
                    || text.trim_start().starts_with("Type ")
                {
                    LineStyle::Muted
                } else {
                    LineStyle::Normal
                };
                (text.to_string(), style)
            })
            .collect(),
        clear_screen: false,
        typewriter: true,
        start_game: None,
    }
}
