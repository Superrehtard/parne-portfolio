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
        CommandInfo {
            name: "history",
            description: "Show command history",
            usage: "history",
            hidden: false,
        },
        CommandInfo {
            name: "echo",
            description: "Print text",
            usage: "echo <text>",
            hidden: false,
        },
        CommandInfo {
            name: "whoami",
            description: "Print current user",
            usage: "whoami",
            hidden: false,
        },
        CommandInfo {
            name: "date",
            description: "Print current date",
            usage: "date",
            hidden: false,
        },
        // Hidden easter eggs
        CommandInfo {
            name: "sudo",
            description: "Superuser do",
            usage: "sudo <command>",
            hidden: true,
        },
        CommandInfo {
            name: "rm",
            description: "Remove files",
            usage: "rm <file>",
            hidden: true,
        },
        CommandInfo {
            name: "neofetch",
            description: "System info",
            usage: "neofetch",
            hidden: true,
        },
        CommandInfo {
            name: "cowsay",
            description: "Cow says moo",
            usage: "cowsay [message]",
            hidden: true,
        },
        CommandInfo {
            name: "exit",
            description: "Exit terminal",
            usage: "exit",
            hidden: true,
        },
        CommandInfo {
            name: "ttt",
            description: "Play Tic-Tac-Toe",
            usage: "ttt",
            hidden: false,
        },
        CommandInfo {
            name: "typing",
            description: "Typing speed test",
            usage: "typing",
            hidden: false,
        },
    ]
}
