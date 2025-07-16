use super::*;

/// The WritableStreamDefaultWriter class.
/// [`WritableStreamDefaultWriter`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WritableStreamDefaultWriter {
    inner: Any,
}
impl FromVal for WritableStreamDefaultWriter {
    fn from_val(v: &Any) -> Self {
        WritableStreamDefaultWriter {
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
impl core::ops::Deref for WritableStreamDefaultWriter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WritableStreamDefaultWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WritableStreamDefaultWriter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WritableStreamDefaultWriter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WritableStreamDefaultWriter> for Any {
    fn from(s: WritableStreamDefaultWriter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WritableStreamDefaultWriter> for Any {
    fn from(s: &WritableStreamDefaultWriter) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WritableStreamDefaultWriter);

impl WritableStreamDefaultWriter {
    /// The `new WritableStreamDefaultWriter(..)` constructor, creating a new WritableStreamDefaultWriter instance
    pub fn new(stream: &WritableStream) -> WritableStreamDefaultWriter {
        Self {
            inner: Any::global("WritableStreamDefaultWriter")
                .new(&[stream.into()])
                .as_::<Any>(),
        }
    }
}
impl WritableStreamDefaultWriter {
    /// Getter of the `closed` attribute.
    /// [`WritableStreamDefaultWriter.closed`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/closed)
    pub fn closed(&self) -> Promise {
        self.inner.get("closed").as_::<Promise>()
    }
}
impl WritableStreamDefaultWriter {
    /// Getter of the `desiredSize` attribute.
    /// [`WritableStreamDefaultWriter.desiredSize`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/desiredSize)
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl WritableStreamDefaultWriter {
    /// Getter of the `ready` attribute.
    /// [`WritableStreamDefaultWriter.ready`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/ready)
    pub fn ready(&self) -> Promise {
        self.inner.get("ready").as_::<Promise>()
    }
}
impl WritableStreamDefaultWriter {
    /// The abort method.
    /// [`WritableStreamDefaultWriter.abort`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)
    pub fn abort0(&self) -> Promise {
        self.inner.call("abort", &[]).as_::<Promise>()
    }
    /// The abort method.
    /// [`WritableStreamDefaultWriter.abort`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)
    pub fn abort1(&self, reason: &Any) -> Promise {
        self.inner.call("abort", &[reason.into()]).as_::<Promise>()
    }
}
impl WritableStreamDefaultWriter {
    /// The close method.
    /// [`WritableStreamDefaultWriter.close`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/close)
    pub fn close(&self) -> Promise {
        self.inner.call("close", &[]).as_::<Promise>()
    }
}
impl WritableStreamDefaultWriter {
    /// The releaseLock method.
    /// [`WritableStreamDefaultWriter.releaseLock`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/releaseLock)
    pub fn release_lock(&self) -> Undefined {
        self.inner.call("releaseLock", &[]).as_::<Undefined>()
    }
}
impl WritableStreamDefaultWriter {
    /// The write method.
    /// [`WritableStreamDefaultWriter.write`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/write)
    pub fn write0(&self) -> Promise {
        self.inner.call("write", &[]).as_::<Promise>()
    }
    /// The write method.
    /// [`WritableStreamDefaultWriter.write`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/write)
    pub fn write1(&self, chunk: &Any) -> Promise {
        self.inner.call("write", &[chunk.into()]).as_::<Promise>()
    }
}
