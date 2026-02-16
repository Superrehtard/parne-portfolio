use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn MatrixRain() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move || {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        canvas.set_width(width);
        canvas.set_height(height);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let font_size = 14u32;
        let columns = width / font_size;

        let drops: std::rc::Rc<std::cell::RefCell<Vec<f64>>> =
            std::rc::Rc::new(std::cell::RefCell::new(vec![1.0; columns as usize]));

        let chars = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789ABCDEF";
        let char_vec: Vec<char> = chars.chars().collect();

        let drops_clone = drops.clone();
        let callback = std::rc::Rc::new(std::cell::RefCell::new(None::<Closure<dyn FnMut()>>));
        let callback_clone = callback.clone();

        *callback.borrow_mut() = Some(Closure::new(move || {
            // Semi-transparent black to create fade trail
            context.set_fill_style_str("rgba(0, 0, 0, 0.05)");
            context.fill_rect(0.0, 0.0, width as f64, height as f64);

            context.set_fill_style_str("#00ff41");
            context.set_font(&format!("{}px monospace", font_size));

            let mut drops = drops_clone.borrow_mut();
            for i in 0..drops.len() {
                // Pick a random character
                let idx = (js_sys::Math::random() * char_vec.len() as f64) as usize;
                let ch = char_vec[idx].to_string();

                let x = (i as u32 * font_size) as f64;
                let y = drops[i] * font_size as f64;

                context.fill_text(&ch, x, y).unwrap();

                // Reset drop to top randomly after it passes the bottom
                if y > height as f64 && js_sys::Math::random() > 0.975 {
                    drops[i] = 0.0;
                }

                drops[i] += 1.0;
            }

            // Request next frame
            web_sys::window()
                .unwrap()
                .request_animation_frame(
                    callback_clone
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap();
        }));

        // Start the animation loop
        window
            .request_animation_frame(callback.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    });

    view! {
        <canvas
            node_ref=canvas_ref
            style="position:fixed;top:0;left:0;z-index:0;opacity:0.15;pointer-events:none;"
        />
    }
}
