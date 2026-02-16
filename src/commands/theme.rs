use leptos::prelude::{Get, Set};

use crate::commands::{CommandOutput, LineStyle};
use crate::state::theme::{ColorScheme, ThemeState};

pub fn theme_command(args: &[String], theme: &ThemeState) -> CommandOutput {
    match args.first().map(|s| s.as_str()) {
        None | Some("help") => theme_help(),
        Some("crt") => toggle_crt(args, theme),
        Some("matrix") => toggle_matrix(args, theme),
        Some(name) => set_color_scheme(name, theme),
    }
}

fn theme_help() -> CommandOutput {
    CommandOutput {
        lines: vec![
            (String::new(), LineStyle::Normal),
            ("  Theme commands:".to_string(), LineStyle::Accent),
            (String::new(), LineStyle::Normal),
            (
                "  theme claude       Switch to Claude orange theme".to_string(),
                LineStyle::Normal,
            ),
            (
                "  theme green        Switch to classic green terminal".to_string(),
                LineStyle::Normal,
            ),
            (
                "  theme amber        Switch to retro amber terminal".to_string(),
                LineStyle::Normal,
            ),
            (
                "  theme crt          Toggle CRT scanline effect".to_string(),
                LineStyle::Normal,
            ),
            (
                "  theme matrix       Toggle matrix rain effect".to_string(),
                LineStyle::Normal,
            ),
            (String::new(), LineStyle::Normal),
        ],
        clear_screen: false,
        typewriter: true,
        start_game: None,
    }
}

fn set_color_scheme(name: &str, theme: &ThemeState) -> CommandOutput {
    match ColorScheme::from_str(name) {
        Some(scheme) => {
            theme.color_scheme.set(scheme);
            CommandOutput {
                lines: vec![(
                    format!("  Switched to {} theme.", scheme.name()),
                    LineStyle::Accent,
                )],
                clear_screen: false,
                typewriter: false,
                start_game: None,
            }
        }
        None => CommandOutput {
            lines: vec![
                (format!("  Unknown theme: {}", name), LineStyle::Error),
                (
                    "  Available: claude, green, amber".to_string(),
                    LineStyle::Muted,
                ),
            ],
            clear_screen: false,
            typewriter: false,
            start_game: None,
        },
    }
}

fn toggle_crt(args: &[String], theme: &ThemeState) -> CommandOutput {
    let new_state = match args.get(1).map(|s| s.as_str()) {
        Some("on") => true,
        Some("off") => false,
        _ => !theme.crt_enabled.get(),
    };
    theme.crt_enabled.set(new_state);
    let msg = if new_state {
        "  CRT effect enabled."
    } else {
        "  CRT effect disabled."
    };
    CommandOutput {
        lines: vec![(msg.to_string(), LineStyle::Accent)],
        clear_screen: false,
        typewriter: false,
        start_game: None,
    }
}

fn toggle_matrix(args: &[String], theme: &ThemeState) -> CommandOutput {
    let new_state = match args.get(1).map(|s| s.as_str()) {
        Some("on") => true,
        Some("off") => false,
        _ => !theme.matrix_rain.get(),
    };
    theme.matrix_rain.set(new_state);

    let msg = if new_state {
        "  Matrix rain enabled."
    } else {
        "  Matrix rain disabled."
    };
    CommandOutput {
        lines: vec![(msg.to_string(), LineStyle::Accent)],
        clear_screen: false,
        typewriter: false,
        start_game: None,
    }
}
