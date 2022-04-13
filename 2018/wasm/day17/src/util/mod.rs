mod bindings;

use wasm_bindgen_futures::JsFuture;

/// Sleep for `ms` milliseconds.
pub async fn sleep(ms: u32) {
    JsFuture::from(bindings::sleep(ms)).await.unwrap();
}
