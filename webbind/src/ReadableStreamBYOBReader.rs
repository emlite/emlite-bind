use super::*;

#[derive(Clone, Debug)]
pub struct ReadableStreamBYOBReaderReadOptions {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamBYOBReaderReadOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamBYOBReaderReadOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ReadableStreamBYOBReaderReadOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadableStreamBYOBReaderReadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadableStreamBYOBReaderReadOptions> for emlite::Val {
    fn from(s: ReadableStreamBYOBReaderReadOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadableStreamBYOBReaderReadOptions {
    pub fn min(&self) -> u64 {
        self.inner.get("min").as_::<u64>()
    }

    pub fn set_min(&mut self, value: u64) {
        self.inner.set("min", value);
    }
}
#[derive(Clone, Debug)]
pub struct ReadableStreamBYOBReader {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamBYOBReader {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamBYOBReader {
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
impl std::ops::Deref for ReadableStreamBYOBReader {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadableStreamBYOBReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadableStreamBYOBReader> for emlite::Val {
    fn from(s: ReadableStreamBYOBReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadableStreamBYOBReader {
    pub fn new(stream: ReadableStream) -> ReadableStreamBYOBReader {
        Self {
            inner: emlite::Val::global("ReadableStreamBYOBReader")
                .new(&[stream.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ReadableStreamBYOBReader {
    pub fn read0(&self, view: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("read", &[view.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn read1(
        &self,
        view: jsbind::Any,
        options: ReadableStreamBYOBReaderReadOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("read", &[view.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl ReadableStreamBYOBReader {
    pub fn release_lock(&self) -> jsbind::Undefined {
        self.inner
            .call("releaseLock", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl ReadableStreamBYOBReader {
    pub fn closed(&self) -> jsbind::Promise {
        self.inner.get("closed").as_::<jsbind::Promise>()
    }
}
impl ReadableStreamBYOBReader {
    pub fn cancel0(&self) -> jsbind::Promise {
        self.inner.call("cancel", &[]).as_::<jsbind::Promise>()
    }

    pub fn cancel1(&self, reason: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("cancel", &[reason.into()])
            .as_::<jsbind::Promise>()
    }
}
