use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/sleep.js")]
extern "C" {
    pub fn sleep_impl(millis: i32)-> js_sys::Promise;
}

pub async fn sleep(millis: i32) {
    use wasm_bindgen_futures::futures_0_3::JsFuture;

    let promise = sleep_impl(millis);
    JsFuture::from(promise).await.expect("sleep() can't fail");
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: i32) -> js_sys::Promise {
    sleep_impl(millis)
}
