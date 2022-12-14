mod bindings;

use std::time::Duration;

use wasm_bindgen_futures::JsFuture;

/// Sleep for `ms` milliseconds.
pub async fn sleep(duration: Duration) {
    let msec = duration.as_millis().try_into().unwrap_or(u32::MAX);

    JsFuture::from(bindings::sleep(msec)).await.unwrap();
}
