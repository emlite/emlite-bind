use super::*;




/// The ReadableStreamDefaultReader class.
/// [`ReadableStreamDefaultReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamDefaultReader {
    inner: Any,
}

impl FromVal for ReadableStreamDefaultReader {
    fn from_val(v: &Any) -> Self {
        ReadableStreamDefaultReader { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReadableStreamDefaultReader {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadableStreamDefaultReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadableStreamDefaultReader {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadableStreamDefaultReader {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ReadableStreamDefaultReader> for Any {
    fn from(s: ReadableStreamDefaultReader) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadableStreamDefaultReader> for Any {
    fn from(s: &ReadableStreamDefaultReader) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ReadableStreamDefaultReader);



impl ReadableStreamDefaultReader {
    /// The `new ReadableStreamDefaultReader(..)` constructor, creating a new ReadableStreamDefaultReader instance
    pub fn new(stream: &ReadableStream) -> ReadableStreamDefaultReader {
        Self {
            inner: Any::global("ReadableStreamDefaultReader").new(&[stream.into()]).as_::<Any>(),
        }
    }

}
impl ReadableStreamDefaultReader {
    /// The read method.
    /// [`ReadableStreamDefaultReader.read`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read)
    pub fn read(&self, ) -> Promise<ReadableStreamReadResult> {
        self.inner.call("read", &[]).as_::<Promise<ReadableStreamReadResult>>()
    }
}
impl ReadableStreamDefaultReader {
    /// The releaseLock method.
    /// [`ReadableStreamDefaultReader.releaseLock`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/releaseLock)
    pub fn release_lock(&self, ) -> Undefined {
        self.inner.call("releaseLock", &[]).as_::<Undefined>()
    }
}
impl ReadableStreamDefaultReader {
    /// Getter of the `closed` attribute.
    /// [`ReadableStreamDefaultReader.closed`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/closed)
    pub fn closed(&self) -> Promise<Undefined> {
        self.inner.get("closed").as_::<Promise<Undefined>>()
    }

}
impl ReadableStreamDefaultReader {
    /// The cancel method.
    /// [`ReadableStreamDefaultReader.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)
    pub fn cancel0(&self, ) -> Promise<Undefined> {
        self.inner.call("cancel", &[]).as_::<Promise<Undefined>>()
    }
    /// The cancel method.
    /// [`ReadableStreamDefaultReader.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)
    pub fn cancel1(&self, reason: &Any) -> Promise<Undefined> {
        self.inner.call("cancel", &[reason.into(), ]).as_::<Promise<Undefined>>()
    }
}
