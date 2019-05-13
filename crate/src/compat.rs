use std::future::Future;
use futures::compat::{
    Compat,
    Compat01As03,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    JsFuture as JsFuture01,
    future_to_promise as future01_to_promise,
};

type JsFuture = Compat01As03<JsFuture01>;

pub trait FutureExt {
    fn to_promise(self) -> Promise;
}

impl<F> FutureExt for F
        where F: Future<Output=Result<JsValue, JsValue>> + 'static {
    fn to_promise(self) -> Promise {
        // pin 0.3
        let future = Box::pin(self);
        // 0.3 to 0.1
        let future = Compat::new(future);
        // 0.1 to promise
        future01_to_promise(future)
    }
}

pub trait PromiseExt<F>
        where F: Future<Output=Result<JsValue, JsValue>> {
    fn to_future(self) -> F;
}

impl PromiseExt<JsFuture> for Promise {
    fn to_future(self) -> JsFuture {
        // promise to 0.1
        let future01 = JsFuture01::from(self);
        // 0.1 to 0.3
        Compat01As03::new(future01)
    }
}
