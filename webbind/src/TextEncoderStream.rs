use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEncoderStream {
    inner: emlite::Val,
}
impl FromVal for TextEncoderStream {
    fn from_val(v: &emlite::Val) -> Self {
        TextEncoderStream {
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
impl core::ops::Deref for TextEncoderStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextEncoderStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextEncoderStream> for emlite::Val {
    fn from(s: TextEncoderStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextEncoderStream {
    pub fn new() -> TextEncoderStream {
        Self {
            inner: emlite::Val::global("TextEncoderStream")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl TextEncoderStream {
    pub fn encoding(&self) -> jsbind::DOMString {
        self.inner.get("encoding").as_::<jsbind::DOMString>()
    }
}
impl TextEncoderStream {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl TextEncoderStream {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
