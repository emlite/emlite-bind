use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEncoderEncodeIntoResult {
    inner: emlite::Val,
}
impl FromVal for TextEncoderEncodeIntoResult {
    fn from_val(v: &emlite::Val) -> Self {
        TextEncoderEncodeIntoResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextEncoderEncodeIntoResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextEncoderEncodeIntoResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextEncoderEncodeIntoResult> for emlite::Val {
    fn from(s: TextEncoderEncodeIntoResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextEncoderEncodeIntoResult {
    pub fn read(&self) -> u64 {
        self.inner.get("read").as_::<u64>()
    }

    pub fn set_read(&mut self, value: u64) {
        self.inner.set("read", value);
    }
}
impl TextEncoderEncodeIntoResult {
    pub fn written(&self) -> u64 {
        self.inner.get("written").as_::<u64>()
    }

    pub fn set_written(&mut self, value: u64) {
        self.inner.set("written", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEncoder {
    inner: emlite::Val,
}
impl FromVal for TextEncoder {
    fn from_val(v: &emlite::Val) -> Self {
        TextEncoder {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextEncoder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextEncoder> for emlite::Val {
    fn from(s: TextEncoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextEncoder {
    pub fn new() -> TextEncoder {
        Self {
            inner: emlite::Val::global("TextEncoder")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl TextEncoder {
    pub fn encode0(&self) -> jsbind::Uint8Array {
        self.inner.call("encode", &[]).as_::<jsbind::Uint8Array>()
    }

    pub fn encode1(&self, input: jsbind::USVString) -> jsbind::Uint8Array {
        self.inner
            .call("encode", &[input.into()])
            .as_::<jsbind::Uint8Array>()
    }
}
impl TextEncoder {
    pub fn encode_into(
        &self,
        source: jsbind::USVString,
        destination: jsbind::Uint8Array,
    ) -> TextEncoderEncodeIntoResult {
        self.inner
            .call("encodeInto", &[source.into(), destination.into()])
            .as_::<TextEncoderEncodeIntoResult>()
    }
}
impl TextEncoder {
    pub fn encoding(&self) -> jsbind::DOMString {
        self.inner.get("encoding").as_::<jsbind::DOMString>()
    }
}
