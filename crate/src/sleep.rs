use wasm_bindgen::prelude::*;

pub async fn sleep(millis: i32) -> Result<(), JsValue> {
    use crate::compat::PromiseExt;

    sleep_js(millis).to_future().await?;
    Ok(())
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: i32) -> js_sys::Promise {
    js_sys::Promise::new(&mut move |resolve, _| {
        let window = web_sys::window().expect("should have a Window");
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &resolve, millis
        ).expect("don't expect error on setTimeout()");
    })
}
