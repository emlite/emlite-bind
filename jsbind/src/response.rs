use crate::utils::*;
use crate::{any::Any, array::ArrayBuffer, error::*, promise::Promise, string::JsString};

/// JavaScript `Response` object returned by `fetch`. There is also a Response object in webbind.
/// This requires a javascript runtime which supports Response and fetch!
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Response {
    inner: emlite::Val,
}

bind!(Response);
impl_dyn_cast!(Response);

impl Response {
    /// `response.ok`
    pub fn ok(&self) -> bool {
        self.inner.get("ok").as_::<bool>()
    }
    /// `response.status`
    pub fn status(&self) -> u16 {
        self.inner.get("status").as_::<u32>() as u16
    }
    /// `response.headers` – you can wrap later if you need it
    pub fn headers_raw(&self) -> Any {
        self.inner.get("headers")
    }

    /// `response.text()` – consumes the body and resolves to JsString.
    pub fn text(&self) -> Promise<Result<JsString, JsError>> {
        self.inner.call("text", &[]).as_::<Promise<_>>()
    }
    /// `response.json()` – resolves to any JS value (`Any`).
    pub fn json(&self) -> Promise<Result<Any, JsError>> {
        self.inner.call("json", &[]).as_::<Promise<_>>()
    }
    /// `response.arrayBuffer()` – resolves to `ArrayBuffer`; we map to
    /// `Uint8Array` for easy use in Rust.
    pub fn array_buffer(&self) -> Promise<Result<ArrayBuffer, JsError>> {
        self.inner.call("arrayBuffer", &[]).as_::<Promise<_>>()
    }
}

/// JavaScript `fetch(input, init?)` – returns a `Promise<Response>`.
///
/// ```rust
/// use jsbind::{fetch, Promise, Response};
///
/// let p: Promise = fetch("https://example.com", None);
/// let resp: Response = p.await_();
/// if resp.ok() {
///     let body_p = resp.text();
///     let txt: String = body_p.await_();
///     println!("{}", txt);
/// }
/// ```
pub fn fetch(input: &str, init: Option<&Any>) -> Promise<Result<Response, JsError>> {
    let fetch_fn = emlite::Val::global("fetch");
    if !fetch_fn.is_function() {
        let err = JsError::new("globalThis.fetch is not available");
        return Promise::reject(err);
    }
    let mut args: alloc::vec::Vec<emlite::Val> = alloc::vec::Vec::with_capacity(2);
    args.push(input.into());
    if let Some(i) = init {
        args.push(i.clone());
    }
    fetch_fn.invoke(&args).as_::<Promise<_>>()
}

/* Convenience: fetch with Request object or other Any */
pub fn fetch_val(input: &Any, init: Option<&Any>) -> Promise<Result<Response, JsError>> {
    let fetch_fn = emlite::Val::global("fetch");
    if !fetch_fn.is_function() {
        let err = JsError::new("globalThis.fetch is not available");
        return Promise::reject(err);
    }
    let mut args: alloc::vec::Vec<emlite::Val> = alloc::vec::Vec::with_capacity(2);
    args.push(input.clone());
    if let Some(i) = init {
        args.push(i.clone());
    }
    fetch_fn.invoke(&args).as_::<Promise<_>>()
}
