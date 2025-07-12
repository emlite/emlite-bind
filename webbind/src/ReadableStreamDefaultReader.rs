use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for ReadableStreamReadResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadableStreamReadResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadableStreamReadResult> for emlite::Val {
    fn from(s: ReadableStreamReadResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for ReadableStreamDefaultReader {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadableStreamDefaultReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadableStreamDefaultReader> for emlite::Val {
    fn from(s: ReadableStreamDefaultReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
