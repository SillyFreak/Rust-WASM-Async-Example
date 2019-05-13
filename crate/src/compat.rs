use std::future::Future;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

pub trait FutureExt {
    fn to_promise(self) -> Promise;
}

impl<F> FutureExt for F
        where F: Future<Output=Result<JsValue, JsValue>> + 'static {
    fn to_promise(self) -> Promise {
        use wasm_bindgen_futures::future_to_promise;
        use futures::compat::Compat;

        // pin 0.3
        let future = Box::pin(self);
        // 0.3 to 0.1
        let future = Compat::new(future);
        // 0.1 to promise
        future_to_promise(future)
    }
}

pub fn promise_to_future(promise: Promise) -> impl Future<Output=Result<JsValue, JsValue>> {
    use wasm_bindgen_futures::JsFuture;
    use futures::compat::Compat01As03;

    // promise to 0.1
    let future01 = JsFuture::from(promise);
    // 0.1 to 0.3
    Compat01As03::new(future01)
}
