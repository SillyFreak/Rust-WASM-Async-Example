use wasm_bindgen::prelude::*;

pub async fn sleep(millis: i32) -> Result<(), JsValue> {
    use crate::compat::promise_to_future;

    let promise = js_sys::Promise::new(&mut move |resolve, _| {
        let window = web_sys::window().expect("should have a Window");
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &resolve, millis
        ).expect("don't expect error on setTimeout()");
    });

    promise_to_future(promise).await?;
    Ok(())
}
