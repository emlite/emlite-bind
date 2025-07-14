use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamReadResult {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamReadResult {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamReadResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadableStreamReadResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamReadResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableStreamReadResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableStreamReadResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableStreamReadResult> for emlite::Val {
    fn from(s: ReadableStreamReadResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadableStreamReadResult {
    pub fn value(&self) -> jsbind::Any {
        self.inner.get("value").as_::<jsbind::Any>()
    }

    pub fn set_value(&mut self, value: jsbind::Any) {
        self.inner.set("value", value);
    }
}
impl ReadableStreamReadResult {
    pub fn done(&self) -> bool {
        self.inner.get("done").as_::<bool>()
    }

    pub fn set_done(&mut self, value: bool) {
        self.inner.set("done", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamDefaultReader {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamDefaultReader {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamDefaultReader {
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
impl core::ops::Deref for ReadableStreamDefaultReader {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamDefaultReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableStreamDefaultReader {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableStreamDefaultReader {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableStreamDefaultReader> for emlite::Val {
    fn from(s: ReadableStreamDefaultReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ReadableStreamDefaultReader);

impl ReadableStreamDefaultReader {
    pub fn new(stream: ReadableStream) -> ReadableStreamDefaultReader {
        Self {
            inner: emlite::Val::global("ReadableStreamDefaultReader")
                .new(&[stream.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ReadableStreamDefaultReader {
    pub fn read(&self) -> jsbind::Promise {
        self.inner.call("read", &[]).as_::<jsbind::Promise>()
    }
}
impl ReadableStreamDefaultReader {
    pub fn release_lock(&self) -> jsbind::Undefined {
        self.inner
            .call("releaseLock", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl ReadableStreamDefaultReader {
    pub fn closed(&self) -> jsbind::Promise {
        self.inner.get("closed").as_::<jsbind::Promise>()
    }
}
impl ReadableStreamDefaultReader {
    pub fn cancel0(&self) -> jsbind::Promise {
        self.inner.call("cancel", &[]).as_::<jsbind::Promise>()
    }

    pub fn cancel1(&self, reason: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("cancel", &[reason.into()])
            .as_::<jsbind::Promise>()
    }
}
