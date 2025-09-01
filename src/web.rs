#[cfg(target_arch = "wasm32")]
mod web_support {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    pub fn enable_pointer_lock(canvas: &HtmlCanvasElement) {
        let closure = Closure::<dyn FnMut(_)>::new({
            let canvas = canvas.clone();
            move |_event: web_sys::MouseEvent| {
                let _ = canvas.request_pointer_lock();
            }
        });

        canvas
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();

        // Prevent the closure from being dropped
        closure.forget();
    }
}

pub fn setup_pointer_lock() {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::HtmlCanvasElement;
        use wasm_bindgen::JsCast;

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("bevy-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        web_support::enable_pointer_lock(&canvas);
    }
}