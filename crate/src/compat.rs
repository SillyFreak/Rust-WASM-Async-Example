use std::future::Future;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::{
    JsFuture,
    future_to_promise,
};

impl<T: ?Sized> FutureExt for T where T: Future {}

pub trait FutureExt: Future {
    fn to_promise(self) -> Promise
            where Self: Future<Output=Result<JsValue, JsValue>> + Sized + 'static {
        future_to_promise(self)
    }
}

pub trait PromiseExt<F>
        where F: Future<Output=Result<JsValue, JsValue>> {
    fn to_future(self) -> F;
}

impl PromiseExt<JsFuture> for Promise {
    fn to_future(self) -> JsFuture {
        JsFuture::from(self)
    }
}
