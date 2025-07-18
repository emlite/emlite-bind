use crate::utils::*;
use crate::{any::Any, promise::Promise};

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

    /// `response.text()` – consumes the body and resolves to DOMString.
    pub fn text(&self) -> Promise {
        self.inner.call("text", &[]).as_::<Promise>()
    }
    /// `response.json()` – resolves to any JS value (`Any`).
    pub fn json(&self) -> Promise {
        self.inner.call("json", &[]).as_::<Promise>()
    }
    /// `response.arrayBuffer()` – resolves to `ArrayBuffer`; we map to
    /// `Uint8Array` for easy use in Rust.
    pub fn array_buffer(&self) -> Promise {
        self.inner.call("arrayBuffer", &[]).as_::<Promise>()
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
pub fn fetch(input: &str, init: Option<&Any>) -> Promise {
    let fetch_fn = emlite::Val::global("fetch");
    match init {
        Some(i) => fetch_fn.invoke(&[input.into(), i.clone()]),
        None => fetch_fn.invoke(&[input.into()]),
    }
    .as_::<Promise>()
}

/* Convenience: fetch with Request object or other Any */
pub fn fetch_val(input: &Any, init: Option<&Any>) -> Promise {
    let fetch_fn = emlite::Val::global("fetch");
    match init {
        Some(i) => fetch_fn.invoke(&[input.clone(), i.clone()]),
        None => fetch_fn.invoke(&[input.clone()]),
    }
    .as_::<Promise>()
}
