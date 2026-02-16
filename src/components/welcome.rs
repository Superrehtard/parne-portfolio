use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const NAME_ART: &[&str] = &[                                                                                                                                                                                                              
    "██████╗ ██████╗ ██╗   ██╗████████╗██╗  ██╗██╗   ██╗██╗    ██████╗  █████╗ ██████╗ ███╗   ██╗███████╗",
    "██╔══██╗██╔══██╗██║   ██║╚══██╔══╝██║  ██║██║   ██║██║    ██╔══██╗██╔══██╗██╔══██╗████╗  ██║██╔════╝",                                                                                                                               
    "██████╔╝██████╔╝██║   ██║   ██║   ███████║██║   ██║██║    ██████╔╝███████║██████╔╝██╔██╗ ██║█████╗  ",                                                                                                                               
    "██╔═══╝ ██╔══██╗██║   ██║   ██║   ██╔══██║╚██╗ ██╔╝██║    ██╔═══╝ ██╔══██║██╔══██╗██║╚██╗██║██╔══╝  ",
    "██║     ██║  ██║╚██████╔╝   ██║   ██║  ██║ ╚████╔╝ ██║    ██║     ██║  ██║██║  ██║██║ ╚████║███████╗",
    "╚═╝     ╚═╝  ╚═╝ ╚═════╝    ╚═╝   ╚═╝  ╚═╝  ╚═══╝  ╚═╝    ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝",
];

const VIEWPORT_WIDTH: usize = 70;
const SPACER: usize = 20; // gap between repeated text

#[component]
pub fn WelcomeBanner() -> impl IntoView {
    let (display, set_display) = signal(String::new());

    let offset = Rc::new(RefCell::new(0usize));

    // Figure out the full width of the art (longest line)
    let full_width = NAME_ART
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);
    let total_width = full_width + SPACER;

    // Build initial frame
    set_display.set(render_frame(0, total_width));

    let offset_clone = offset.clone();
    let callback = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let callback_clone = callback.clone();

    *callback.borrow_mut() = Some(Closure::new(move || {
        let mut off = offset_clone.borrow_mut();
        *off = (*off + 1) % total_width;
        set_display.set(render_frame(*off, total_width));

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback_clone
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
                100,
            )
            .unwrap();
    }));

    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            100,
        )
        .unwrap();

    view! {
        <div class="welcome-banner">
            <pre class="banner-ascii">{move || display.get()}</pre>
            <div class="banner-message">
                "Welcome! Type 'help' to get started, or just explore."
            </div>
        </div>
    }
}

fn render_frame(offset: usize, total_width: usize) -> String {
    let mut result = String::new();

    for line in NAME_ART {
        let chars: Vec<char> = line.chars().collect();
        let line_len = chars.len();

        result.push_str("  ");
        for i in 0..VIEWPORT_WIDTH {
            let idx = (offset + i) % total_width;
            if idx < line_len {
                result.push(chars[idx]);
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    result
}
