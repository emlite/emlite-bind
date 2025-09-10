use super::*;

/// The WritableStream class.
/// [`WritableStream`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WritableStream {
    inner: Any,
}

impl FromVal for WritableStream {
    fn from_val(v: &Any) -> Self {
        WritableStream {
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

impl core::ops::Deref for WritableStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WritableStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WritableStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WritableStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WritableStream> for Any {
    fn from(s: WritableStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WritableStream> for Any {
    fn from(s: &WritableStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WritableStream);

impl WritableStream {
    /// The `new WritableStream(..)` constructor, creating a new WritableStream instance
    pub fn new0() -> WritableStream {
        Self {
            inner: Any::global("WritableStream").new(&[]).as_::<Any>(),
        }
    }

    /// The `new WritableStream(..)` constructor, creating a new WritableStream instance
    pub fn new1(underlying_sink: &Object) -> WritableStream {
        Self {
            inner: Any::global("WritableStream")
                .new(&[underlying_sink.into()])
                .as_::<Any>(),
        }
    }

    /// The `new WritableStream(..)` constructor, creating a new WritableStream instance
    pub fn new2(underlying_sink: &Object, strategy: &QueuingStrategy) -> WritableStream {
        Self {
            inner: Any::global("WritableStream")
                .new(&[underlying_sink.into(), strategy.into()])
                .as_::<Any>(),
        }
    }
}
impl WritableStream {
    /// Getter of the `locked` attribute.
    /// [`WritableStream.locked`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/locked)
    pub fn locked(&self) -> bool {
        self.inner.get("locked").as_::<bool>()
    }
}
impl WritableStream {
    /// The abort method.
    /// [`WritableStream.abort`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)
    pub fn abort0(&self) -> Promise<Undefined> {
        self.inner.call("abort", &[]).as_::<Promise<Undefined>>()
    }
    /// The abort method.
    /// [`WritableStream.abort`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)
    pub fn abort1(&self, reason: &Any) -> Promise<Undefined> {
        self.inner
            .call("abort", &[reason.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl WritableStream {
    /// The close method.
    /// [`WritableStream.close`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/close)
    pub fn close(&self) -> Promise<Undefined> {
        self.inner.call("close", &[]).as_::<Promise<Undefined>>()
    }
}
impl WritableStream {
    /// The getWriter method.
    /// [`WritableStream.getWriter`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/getWriter)
    pub fn get_writer(&self) -> WritableStreamDefaultWriter {
        self.inner
            .call("getWriter", &[])
            .as_::<WritableStreamDefaultWriter>()
    }
}
