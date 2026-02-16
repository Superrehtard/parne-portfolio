use crate::commands::{CommandOutput, LineStyle};

pub fn sudo(args: &[String]) -> CommandOutput {
    let _cmd_text = if args.is_empty() {
        "sudo".to_string()
    } else {
        format!("sudo {}", args.join(" "))
    };

    CommandOutput {
        lines: vec![
            (
                "  Nice try! But you don't have root access here.".to_string(),
                LineStyle::Error,
            ),
            (
                "  This incident will be reported. ;)".to_string(),
                LineStyle::Muted,
            ),
        ],
        clear_screen: false,
        typewriter: true,
        start_game: None,
    }
}

pub fn rm_rf() -> CommandOutput {
    let mut lines = vec![
        ("  Deleting everything...".to_string(), LineStyle::Error),
        (
            "  rm: /usr/bin/: Permission denied".to_string(),
            LineStyle::Normal,
        ),
        (
            "  rm: /etc/: Permission denied".to_string(),
            LineStyle::Normal,
        ),
        (
            "  rm: /home/: Permission denied".to_string(),
            LineStyle::Normal,
        ),
        (String::new(), LineStyle::Normal),
        (
            "  Just kidding. This is a website, not your actual terminal.".to_string(),
            LineStyle::Accent,
        ),
        (
            "  Your files are safe. Probably.".to_string(),
            LineStyle::Muted,
        ),
    ];

    CommandOutput {
        lines,
        clear_screen: false,
        typewriter: true,
        start_game: None,
    }
}

pub fn neofetch() -> CommandOutput {
    let ascii = vec![
        ("        _______        ", LineStyle::Accent),
        ("       /       \\       ", LineStyle::Accent),
        ("      / PRUTHVI \\      ", LineStyle::Accent),
        ("     /  PARNE    \\     ", LineStyle::Accent),
        ("    /  ________   \\    ", LineStyle::Accent),
        ("   /  /        \\   \\   ", LineStyle::Accent),
        ("  /  /  SENIOR  \\   \\  ", LineStyle::Accent),
        (" /  /  SOFTWARE  \\   \\ ", LineStyle::Accent),
        ("/  /  ENGINEER    \\   \\", LineStyle::Accent),
        ("\\  \\              /   /", LineStyle::Accent),
        (" \\  \\____________/   / ", LineStyle::Accent),
        ("  \\________________/  ", LineStyle::Accent),
    ];

    let info = vec![
        ("", LineStyle::Normal),
        ("  visitor@portfolio", LineStyle::Accent),
        ("  -------------------", LineStyle::Muted),
        ("  OS:       WebAssembly/Browser", LineStyle::Normal),
        ("  Host:     GitHub Pages", LineStyle::Normal),
        ("  Shell:    terminal-portfolio 0.1.0", LineStyle::Normal),
        ("  Runtime:  Rust + Leptos + WASM", LineStyle::Normal),
        ("  Theme:    Claude Orange on Dark", LineStyle::Normal),
        ("  CPU:      Your Browser's JS Engine", LineStyle::Normal),
        ("  Memory:   A few MB of WASM", LineStyle::Normal),
        ("", LineStyle::Normal),
    ];

    let mut lines: Vec<(String, LineStyle)> = Vec::new();
    lines.push((String::new(), LineStyle::Normal));

    // Interleave ASCII art and info
    let max_len = ascii.len().max(info.len());
    for i in 0..max_len {
        let art_part = if i < ascii.len() {
            ascii[i].0
        } else {
            "                        "
        };
        let info_part = if i < info.len() { info[i].0 } else { "" };
        let style = if i < info.len() {
            info[i].1.clone()
        } else {
            LineStyle::Normal
        };

        lines.push((format!("  {} {}", art_part, info_part), style));
    }

    lines.push((String::new(), LineStyle::Normal));

    CommandOutput {
        lines,
        clear_screen: false,
        typewriter: true,
        start_game: None,
    }
}

pub fn cowsay(args: &[String]) -> CommandOutput {
    let message = if args.is_empty() {
        "Moo! Type 'help' for commands.".to_string()
    } else {
        args.join(" ")
    };

    let border_len = message.len() + 2;
    let top_border = format!("   {}", "_".repeat(border_len));
    let bottom_border = format!("   {}", "-".repeat(border_len));

    CommandOutput {
        lines: vec![
            (top_border, LineStyle::Normal),
            (format!("  < {} >", message), LineStyle::Accent),
            (bottom_border, LineStyle::Normal),
            ("          \\   ^__^".to_string(), LineStyle::Normal),
            (
                "           \\  (oo)\\_______".to_string(),
                LineStyle::Normal,
            ),
            (
                "              (__)\\       )\\/\\".to_string(),
                LineStyle::Normal,
            ),
            ("                  ||----w |".to_string(), LineStyle::Normal),
            ("                  ||     ||".to_string(), LineStyle::Normal),
        ],
        clear_screen: false,
        typewriter: true,
        start_game: None,
    }
}

pub fn whoami() -> CommandOutput {
    CommandOutput {
        lines: vec![("  visitor".to_string(), LineStyle::Normal)],
        clear_screen: false,
        typewriter: false,
        start_game: None,
    }
}

pub fn date() -> CommandOutput {
    // Get current date/time from JS
    let now = js_sys::Date::new_0();
    let date_str = now.to_locale_string("en-US", &wasm_bindgen::JsValue::UNDEFINED);

    CommandOutput {
        lines: vec![(format!("  {}", String::from(date_str)), LineStyle::Normal)],
        clear_screen: false,
        typewriter: false,
        start_game: None,
    }
}

pub fn echo(args: &[String]) -> CommandOutput {
    CommandOutput {
        lines: vec![(format!("  {}", args.join(" ")), LineStyle::Normal)],
        clear_screen: false,
        typewriter: false,
        start_game: None,
    }
}

pub fn exit() -> CommandOutput {
    CommandOutput {
        lines: vec![
            (
                "  Nice try! But there's no escape from my portfolio.".to_string(),
                LineStyle::Accent,
            ),
            (
                "  You're stuck here. Might as well type 'about'.".to_string(),
                LineStyle::Muted,
            ),
        ],
        clear_screen: false,
        typewriter: true,
        start_game: None,
    }
}

pub fn history_cmd(history: &[String]) -> CommandOutput {
    let mut lines = Vec::new();
    lines.push((String::new(), LineStyle::Normal));

    for (i, cmd) in history.iter().enumerate() {
        lines.push((format!("  {:>4}  {}", i + 1, cmd), LineStyle::Normal));
    }

    lines.push((String::new(), LineStyle::Normal));

    CommandOutput {
        lines,
        clear_screen: false,
        typewriter: false,
        start_game: None,
    }
}
