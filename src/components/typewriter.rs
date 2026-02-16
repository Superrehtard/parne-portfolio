use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn TypewriterLine(
    text: String,
    #[prop(default = true)] animate: bool,
    #[prop(default = 0)] delay_ms: i32,
) -> impl IntoView {
    if !animate {
        return view! { <span>{text}</span> }.into_any();
    }

    let (displayed, set_displayed) = signal(String::new());
    let text_clone = text.clone();

    Effect::new(move || {
        let chars: Vec<char> = text_clone.chars().collect();
        let total = chars.len();
        let window = web_sys::window().unwrap();

        for i in 0..total {
            let chars = chars.clone();

            let cb = Closure::once_into_js(move || {
                set_displayed.set(chars[..=i].iter().collect());
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                delay_ms + (i as i32) * 12,
            );
        }
    });

    view! { <span>{move || displayed.get()}</span> }.into_any()
}
