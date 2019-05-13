use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/sleep.js")]
extern "C" {
    pub fn sleep_impl(millis: i32)-> js_sys::Promise;
}

pub async fn sleep(millis: i32) {
    use crate::compat::PromiseExt;

    sleep_impl(millis).to_future().await.expect("sleep() can't fail");
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: i32) -> js_sys::Promise {
    sleep_impl(millis)
}
