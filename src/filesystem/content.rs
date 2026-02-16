pub const ABOUT_TEXT: &str = "\
Hey! I'm Pruthvi.
Senior Software Engineer with experience in full-stack and mobile development.

I love building things that live on the web and in your pocket.
Currently exploring secOps and Platform Engineering professionally, Rust and WebAssembly as a hobby.

Type 'ls' to explore my files, or 'help' for all commands.";

pub const CONTACT_TEXT: &str = "\
Email:    iamparne@email.com
GitHub:   github.com/Superrehtard
LinkedIn: linkedin.com/in/iamparne
Personal: iamparne.dev";

pub const RESUME_TEXT: &str = "\
[Resume content coming soon]

For now, check out 'about', 'skills', and 'projects'.";

pub const SKILLS_LANGUAGES: &str = "\
Programming Languages
=====================
Swift       ████████████████████░░  Advanced
TypeScript  ████████████████████░░  Advanced
Java        ██████████████████░░░░  Proficient
Python      ████████████████░░░░░░  Proficient
JavaScript  ████████████████░░░░░░  Proficient
Rust        ██████████████░░░░░░░░  Intermediate";

pub const SKILLS_FRAMEWORKS: &str = "\
Frameworks & Libraries
======================
React               Frontend
Leptos              Frontend (Rust/WASM)
SwiftUI / UIKit     iOS
Actix / Axum        Backend (Rust)";

pub const SKILLS_TOOLS: &str = "\
Tools & Infrastructure
======================
Git / GitHub        Version Control
Docker / K8s        Containers
AWS / GCP           Cloud
PostgreSQL / Redis  Databases
CI/CD               GitHub Actions, Jenkins
Figma               Design Collaboration";

pub const PROJECT1_README: &str = "\
# Terminal Portfolio

A terminal-style portfolio website built with Rust, Leptos, and WebAssembly.

Tech: Rust, Leptos, WASM, CSS
Link: iamparne.dev

Features:
- Command-line interface for portfolio navigation
- Virtual filesystem simulation
- Easter eggs and mini-games";

pub const PROJECT2_README: &str = "\
# Project Two

Description placeholder.

Tech: [tech stack]
Status: [status]
Link: [url]";

pub const PROJECT3_README: &str = "\
# Project Three

Description placeholder.

Tech: [tech stack]
Status: [status]
Link: [url]";

pub const CMD_ABOUT: &[(&str, bool)] = &[
    ("", false),
    ("Hey! I'm Pruthvi Parne, a Senior Software Engineer.", true),
    ("Full-stack developer with mobile experience.", false),
    (
        "Currently exploring secOps and Platform Engineering professionally,",
        false,
    ),
    ("Rust and WebAssembly as a hobby.", false),
    ("", false),
    (
        "Type 'skills' to see what I work with, or 'projects' to see what I've built.",
        false,
    ),
    ("", false),
];

pub const CMD_SKILLS: &[(&str, bool)] = &[
    ("", false),
    (
        "Languages:   Swift, TypeScript, Java, Python, JavaScript, Rust",
        false,
    ),
    ("Frontend:    React, Leptos", false),
    ("Mobile:      SwiftUI, UIKit", false),
    ("Backend:     Actix, Axum", false),
    (
        "Tools:       Git, Docker, K8s, AWS/GCP, PostgreSQL, Redis",
        false,
    ),
    ("", false),
    (
        "Run 'cat ~/skills/languages.txt' for detailed breakdown.",
        false,
    ),
    ("", false),
];

pub const CMD_PROJECTS: &[(&str, bool)] = &[
    ("", false),
    ("1. Terminal Portfolio", true),
    (
        "A terminal-style portfolio built with Rust + Leptos + WASM",
        false,
    ),
    ("", false),
    ("2. CNH Mobile App", true),
    (
        "Native iOS application for ag-tech company Case New Holland",
        false,
    ),
    ("", false),
    ("3. Corteva Fungicide Timing", true),
    ("Frontend application for Corteva/Granular", false),
    ("", false),
    (
        "Run 'cat ~/projects/project1/README.md' for details.",
        false,
    ),
    ("", false),
];

pub const CMD_CONTACT: &[(&str, bool)] = &[
    ("", false),
    ("  Email:    iamparne@email.com", false),
    ("  GitHub:   github.com/Superrehtard", true),
    ("  LinkedIn: linkedin.com/in/iamparne", true),
    ("  Website:  iamparne.dev", true),
    ("", false),
];

pub const CMD_RESUME: &[(&str, bool)] = &[
    ("", false),
    ("  Resume download coming soon!", false),
    ("  For now, type 'about', 'skills', or 'projects'.", false),
    ("", false),
];
