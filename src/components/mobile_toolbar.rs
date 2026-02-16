use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn MobileToolbar() -> impl IntoView {
    let send_key = move |key: &str| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        if let Some(input) = document.query_selector(".terminal-input").unwrap() {
            let _ = input.dyn_ref::<web_sys::HtmlElement>().unwrap().focus();
            let keyboard_event = web_sys::KeyboardEventInit::new();
            keyboard_event.set_key(key);
            keyboard_event.set_bubbles(true);
            keyboard_event.set_cancelable(true);

            let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
                "keydown",
                &keyboard_event,
            )
            .unwrap();

            let _ = input.dispatch_event(&event);
        }
    };

    view! {
        <div class="mobile-toolbar">
            <button on:click=move |_| send_key("Tab")>"Tab"</button>
            <button on:click=move |_| send_key("ArrowUp")>"↑"</button>
            <button on:click=move |_| send_key("ArrowDown")>"↓"</button>
            <button on:click=move |_| send_key("Enter")>"Enter"</button>
        </div>
    }
}
