mod app;
mod commands;
mod components;
mod filesystem;
mod parser;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
