use leptos::prelude::*;

const BANNER: &str = r#"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║   ██████╗  ██████╗ ██████╗ ████████╗███████╗ ██████╗         ║
    ║   ██╔══██╗██╔═══██╗██╔══██╗╚══██╔══╝██╔════╝██╔═══██╗        ║
    ║   ██████╔╝██║   ██║██████╔╝   ██║   █████╗  ██║   ██║        ║
    ║   ██╔═══╝ ██║   ██║██╔══██╗   ██║   ██╔══╝  ██║   ██║        ║
    ║   ██║     ╚██████╔╝██║  ██║   ██║   ██║     ╚██████╔╝        ║
    ║   ╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝      ╚═════╝         ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
"#;

const WELCOME_MSG: &str = "  Welcome! I'm Pruthvi, a Senior Software Engineer.
    Type 'help' to see available commands, or just start exploring.
";

#[component]
pub fn WelcomeBanner() -> impl IntoView {
    view! {
        <div class="welcome-banner">
            <pre class="banner-ascii">{BANNER}</pre>
            <div class="banner-message">{WELCOME_MSG}</div>
        </div>
    }
}
