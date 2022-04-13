use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen(module = "/js/util.js")]
extern "C" {
    pub(super) fn sleep(ms: u32) -> Promise;
}
