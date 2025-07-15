use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ReadableStreamBYOBReaderReadOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamBYOBReaderReadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableStreamBYOBReaderReadOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableStreamBYOBReaderReadOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableStreamBYOBReaderReadOptions> for emlite::Val {
    fn from(s: ReadableStreamBYOBReaderReadOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ReadableStreamBYOBReader {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamBYOBReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableStreamBYOBReader {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableStreamBYOBReader {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableStreamBYOBReader> for emlite::Val {
    fn from(s: ReadableStreamBYOBReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ReadableStreamBYOBReader);

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
    pub fn read0(&self, view: Any) -> Promise {
        self.inner.call("read", &[view.into()]).as_::<Promise>()
    }

    pub fn read1(&self, view: Any, options: ReadableStreamBYOBReaderReadOptions) -> Promise {
        self.inner
            .call("read", &[view.into(), options.into()])
            .as_::<Promise>()
    }
}
impl ReadableStreamBYOBReader {
    pub fn release_lock(&self) -> Undefined {
        self.inner.call("releaseLock", &[]).as_::<Undefined>()
    }
}
impl ReadableStreamBYOBReader {
    pub fn closed(&self) -> Promise {
        self.inner.get("closed").as_::<Promise>()
    }
}
impl ReadableStreamBYOBReader {
    pub fn cancel0(&self) -> Promise {
        self.inner.call("cancel", &[]).as_::<Promise>()
    }

    pub fn cancel1(&self, reason: Any) -> Promise {
        self.inner.call("cancel", &[reason.into()]).as_::<Promise>()
    }
}
