use std::future::Future;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    JsFuture,
    future_to_promise as _future_to_promise,
};
use futures::compat::{
    Compat,
    Compat01As03,
};

pub fn future_to_promise<F>(future: F) -> Promise
        where F: Future<Output=Result<JsValue, JsValue>> + Unpin + 'static {
    // 0.3 to 0.1
    let future = Compat::new(future);
    // 0.1 to promise
    _future_to_promise(future)
}

pub fn promise_to_future(promise: Promise) -> impl Future<Output=Result<JsValue, JsValue>> {
    // promise to 0.1
    let future01 = JsFuture::from(promise);
    // 0.1 to 0.3
    Compat01As03::new(future01)
}
