use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompressionStream {
    inner: emlite::Val,
}
impl FromVal for CompressionStream {
    fn from_val(v: &emlite::Val) -> Self {
        CompressionStream {
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
impl core::ops::Deref for CompressionStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CompressionStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CompressionStream> for emlite::Val {
    fn from(s: CompressionStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CompressionStream {
    pub fn new(format: CompressionFormat) -> CompressionStream {
        Self {
            inner: emlite::Val::global("CompressionStream")
                .new(&[format.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl CompressionStream {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl CompressionStream {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
