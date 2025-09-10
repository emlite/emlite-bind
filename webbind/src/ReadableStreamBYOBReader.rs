use super::*;

/// The ReadableStreamBYOBReader class.
/// [`ReadableStreamBYOBReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamBYOBReader {
    inner: Any,
}

impl FromVal for ReadableStreamBYOBReader {
    fn from_val(v: &Any) -> Self {
        ReadableStreamBYOBReader {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReadableStreamBYOBReader {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadableStreamBYOBReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadableStreamBYOBReader {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadableStreamBYOBReader {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ReadableStreamBYOBReader> for Any {
    fn from(s: ReadableStreamBYOBReader) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadableStreamBYOBReader> for Any {
    fn from(s: &ReadableStreamBYOBReader) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ReadableStreamBYOBReader);

impl ReadableStreamBYOBReader {
    /// The `new ReadableStreamBYOBReader(..)` constructor, creating a new ReadableStreamBYOBReader instance
    pub fn new(stream: &ReadableStream) -> ReadableStreamBYOBReader {
        Self {
            inner: Any::global("ReadableStreamBYOBReader")
                .new(&[stream.into()])
                .as_::<Any>(),
        }
    }
}
impl ReadableStreamBYOBReader {
    /// The read method.
    /// [`ReadableStreamBYOBReader.read`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)
    pub fn read0(&self, view: &Any) -> Promise<ReadableStreamReadResult> {
        self.inner
            .call("read", &[view.into()])
            .as_::<Promise<ReadableStreamReadResult>>()
    }
    /// The read method.
    /// [`ReadableStreamBYOBReader.read`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)
    pub fn read1(
        &self,
        view: &Any,
        options: &ReadableStreamBYOBReaderReadOptions,
    ) -> Promise<ReadableStreamReadResult> {
        self.inner
            .call("read", &[view.into(), options.into()])
            .as_::<Promise<ReadableStreamReadResult>>()
    }
}
impl ReadableStreamBYOBReader {
    /// The releaseLock method.
    /// [`ReadableStreamBYOBReader.releaseLock`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/releaseLock)
    pub fn release_lock(&self) -> Undefined {
        self.inner.call("releaseLock", &[]).as_::<Undefined>()
    }
}
impl ReadableStreamBYOBReader {
    /// Getter of the `closed` attribute.
    /// [`ReadableStreamBYOBReader.closed`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/closed)
    pub fn closed(&self) -> Promise<Undefined> {
        self.inner.get("closed").as_::<Promise<Undefined>>()
    }
}
impl ReadableStreamBYOBReader {
    /// The cancel method.
    /// [`ReadableStreamBYOBReader.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)
    pub fn cancel0(&self) -> Promise<Undefined> {
        self.inner.call("cancel", &[]).as_::<Promise<Undefined>>()
    }
    /// The cancel method.
    /// [`ReadableStreamBYOBReader.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)
    pub fn cancel1(&self, reason: &Any) -> Promise<Undefined> {
        self.inner
            .call("cancel", &[reason.into()])
            .as_::<Promise<Undefined>>()
    }
}
