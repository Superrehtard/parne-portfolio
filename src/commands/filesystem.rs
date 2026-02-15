use crate::commands::{CommandOutput, LineStyle};
use crate::filesystem::model::VirtualFs;

pub fn pwd(cwd: &str) -> CommandOutput {
    CommandOutput {
        lines: vec![(cwd.to_string(), LineStyle::Normal)],
        clear_screen: false,
        typewriter: true,
    }
}

pub fn cd(fs: &VirtualFs, cwd: &mut String, args: &[String]) -> CommandOutput {
    let target = match args.first() {
        Some(path) => path.as_str(),
        None => "~",
    };

    let resolved = fs.resolve_path(cwd, target);

    if fs.is_dir(&resolved) {
        *cwd = resolved;
        CommandOutput {
            lines: vec![],
            clear_screen: false,
            typewriter: true,
        }
    } else if fs.is_file(&resolved) {
        CommandOutput {
            lines: vec![(format!("cd: not a directory: {}", target), LineStyle::Error)],
            clear_screen: false,
            typewriter: true,
        }
    } else {
        CommandOutput {
            lines: vec![(
                format!("cd: no such directory: {}", target),
                LineStyle::Error,
            )],
            clear_screen: false,
            typewriter: true,
        }
    }
}

pub fn ls(fs: &VirtualFs, cwd: &str, args: &[String]) -> CommandOutput {
    let target = match args.first() {
        Some(path) => fs.resolve_path(cwd, path),
        None => cwd.to_string(),
    };

    match fs.ls(&target) {
        Some(entries) => {
            let lines = entries
                .iter()
                .map(|(name, is_dir)| {
                    if *is_dir {
                        (format!("  {}/", name), LineStyle::Accent)
                    } else {
                        (format!("  {}", name), LineStyle::Normal)
                    }
                })
                .collect();
            CommandOutput {
                lines,
                clear_screen: false,
                typewriter: true,
            }
        }
        None => CommandOutput {
            lines: vec![(
                format!(
                    "ls: cannot access '{}': No such directory",
                    args.first().unwrap_or(&target)
                ),
                LineStyle::Error,
            )],
            clear_screen: false,
            typewriter: true,
        },
    }
}

pub fn cat(fs: &VirtualFs, cwd: &str, args: &[String]) -> CommandOutput {
    let target = match args.first() {
        Some(path) => path.as_str(),
        None => {
            return CommandOutput {
                lines: vec![("cat: missing file argument".to_string(), LineStyle::Error)],
                clear_screen: false,
                typewriter: true,
            };
        }
    };

    let resolved = fs.resolve_path(cwd, target);

    if fs.is_dir(&resolved) {
        return CommandOutput {
            lines: vec![(format!("cat: {}: Is a directory", target), LineStyle::Error)],
            clear_screen: false,
            typewriter: true,
        };
    }

    match fs.cat(&resolved) {
        Some(content) => {
            let lines = content
                .lines()
                .map(|line| (line.to_string(), LineStyle::Normal))
                .collect();
            CommandOutput {
                lines,
                clear_screen: false,
                typewriter: true,
            }
        }
        None => CommandOutput {
            lines: vec![(format!("cat: {}: No such file", target), LineStyle::Error)],
            clear_screen: false,
            typewriter: true,
        },
    }
}

pub fn tree(fs: &VirtualFs, cwd: &str, args: &[String]) -> CommandOutput {
    let target = match args.first() {
        Some(path) => fs.resolve_path(cwd, path),
        None => cwd.to_string(),
    };

    match fs.tree(&target, "", true) {
        Some(lines) => CommandOutput {
            lines: lines
                .into_iter()
                .map(|line| (line, LineStyle::Normal))
                .collect(),
            clear_screen: false,
            typewriter: true,
        },
        None => CommandOutput {
            lines: vec![(
                format!("tree: '{}': No such directory", target),
                LineStyle::Error,
            )],
            clear_screen: false,
            typewriter: true,
        },
    }
}
