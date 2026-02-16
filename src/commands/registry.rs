pub struct CommandInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub hidden: bool,
}

pub fn all_commands() -> Vec<CommandInfo> {
    vec![
        CommandInfo {
            name: "help",
            description: "Show available commands",
            usage: "help [command]",
            hidden: false,
        },
        CommandInfo {
            name: "about",
            description: "Learn about me",
            usage: "about",
            hidden: false,
        },
        CommandInfo {
            name: "skills",
            description: "View my technical skills",
            usage: "skills",
            hidden: false,
        },
        CommandInfo {
            name: "projects",
            description: "Browse my projects",
            usage: "projects",
            hidden: false,
        },
        CommandInfo {
            name: "contact",
            description: "Get my contact information",
            usage: "contact",
            hidden: false,
        },
        CommandInfo {
            name: "resume",
            description: "View my resume",
            usage: "resume",
            hidden: false,
        },
        CommandInfo {
            name: "clear",
            description: "Clear the terminal",
            usage: "clear",
            hidden: false,
        },
        CommandInfo {
            name: "pwd",
            description: "Print current directory",
            usage: "pwd",
            hidden: false,
        },
        CommandInfo {
            name: "cd",
            description: "Change directory",
            usage: "cd <path>",
            hidden: false,
        },
        CommandInfo {
            name: "ls",
            description: "List directory contents",
            usage: "ls [path]",
            hidden: false,
        },
        CommandInfo {
            name: "cat",
            description: "Display file contents",
            usage: "cat <file>",
            hidden: false,
        },
        CommandInfo {
            name: "tree",
            description: "Show directory tree",
            usage: "tree <path>",
            hidden: false,
        },
        CommandInfo {
            name: "theme",
            description: "Change terminal theme/effects",
            usage: "theme [claude|green|amber|crt|matrix]",
            hidden: false,
        },
    ]
}
