use wasm_bindgen::JsCast;

/// Get 2D canvas context.
pub fn canvas_context_2d() -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

/// Log a message to the console.
pub fn log(s: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let console = document.get_element_by_id("console").unwrap()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .map_err(|_| ())
        .unwrap();

    let value = console.value() + s + "\n";
    console.set_value(&value);
    console.set_scroll_top(console.scroll_height());
}

#[macro_export]
macro_rules! log {
    ($($v: expr),*) => { crate::page::log(&format!($($v),*)) };
}
