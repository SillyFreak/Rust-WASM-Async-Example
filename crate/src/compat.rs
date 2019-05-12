use std::future::Future;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise as _future_to_promise;
use futures::compat::Compat;

pub fn future_to_promise<F>(future: F) -> Promise
        where F: Future<Output=Result<JsValue, JsValue>> + Unpin + 'static {
    // 0.3 to 0.1
    let future = Compat::new(future);
    // 0.1 to promise
    _future_to_promise(future)
}
