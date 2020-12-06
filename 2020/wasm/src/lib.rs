mod page;
mod util;

use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn greet_async(name: JsValue) {
    log!("Hello...");
    util::sleep(5_000).await;
    log!("{}!", name.as_string().unwrap_or(String::from("world")));
}

#[wasm_bindgen]
pub async fn draw_circle() {
    let ctx = page::canvas_context_2d();

    ctx.begin_path();
    ctx.set_stroke_style(&JsValue::from_str("#cccccc"));
    ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0).unwrap();
    ctx.stroke();
}
