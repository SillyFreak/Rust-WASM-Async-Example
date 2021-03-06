#![feature(async_await)]
#![deny(warnings)]

mod sleep;

use wasm_bindgen::prelude::*;
use crate::sleep::sleep;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub async fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));
    body.append_child(&p)?;

    sleep(1000).await;

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("...asynchronously!"));
    body.append_child(&p)?;

    Ok(())
}

// Called by our JS entry point to run the example.
#[wasm_bindgen(js_name = run)]
pub fn run_js() -> js_sys::Promise {
    use wasm_bindgen_futures::futures_0_3::future_to_promise;

    future_to_promise(async move {
        run().await?;
        Ok(JsValue::UNDEFINED)
    })
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
