use crate::utils::bind;
use crate::{Any, Uint8Array};
use alloc::string::String;

/// JavaScript `TextEncoder` (`new TextEncoder()`).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEncoder {
    inner: emlite::Val,
}
bind!(TextEncoder);

impl TextEncoder {
    /// `new TextEncoder()` (always UTF-8 in browsers/Node).
    pub fn new() -> Self {
        emlite::Val::global("TextEncoder").new(&[]).as_::<Self>()
    }

    /// `encoder.encode(str)` → `Uint8Array`
    pub fn encode(&self, s: &str) -> Uint8Array {
        self.inner.call("encode", &[s.into()]).as_::<Uint8Array>()
    }

    /// `encoder.encodeInto(src, dst)` → `{ read, written }`
    pub fn encode_into(&self, src: &str, dst: &mut Uint8Array) -> (usize, usize) {
        let res = self
            .inner
            .call("encodeInto", &[src.into(), dst.clone().into()]);
        let read = res.get("read").as_::<u32>() as usize;
        let written = res.get("written").as_::<u32>() as usize;
        (read, written)
    }
}

/// JavaScript `TextDecoder`
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextDecoder {
    inner: emlite::Val,
}
bind!(TextDecoder);

impl TextDecoder {
    /// `new TextDecoder(label?, options?)`
    pub fn new(label: Option<&str>, opts: Option<&Any>) -> Self {
        let ctor = emlite::Val::global("TextDecoder");
        match (label, opts) {
            (Some(l), Some(o)) => ctor.new(&[l.into(), o.clone()]).as_::<Self>(),
            (Some(l), None) => ctor.new(&[l.into()]).as_::<Self>(),
            (None, Some(o)) => ctor.new(&[o.clone()]).as_::<Self>(),
            (None, None) => ctor.new(&[]).as_::<Self>(),
        }
    }

    /// `decoder.decode(input)` – UTF-8 → `String`.
    pub fn decode(&self, bytes: &Uint8Array) -> String {
        self.inner.call("decode", &[bytes.clone().into()]).as_()
    }
}
