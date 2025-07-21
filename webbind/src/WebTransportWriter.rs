use super::*;

/// The WebTransportWriter class.
/// [`WebTransportWriter`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportWriter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportWriter {
    inner: WritableStreamDefaultWriter,
}
impl FromVal for WebTransportWriter {
    fn from_val(v: &Any) -> Self {
        WebTransportWriter {
            inner: WritableStreamDefaultWriter::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportWriter {
    type Target = WritableStreamDefaultWriter;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransportWriter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportWriter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportWriter> for Any {
    fn from(s: WebTransportWriter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportWriter> for Any {
    fn from(s: &WebTransportWriter) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebTransportWriter);

impl WebTransportWriter {
    /// The atomicWrite method.
    /// [`WebTransportWriter.atomicWrite`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportWriter/atomicWrite)
    pub fn atomic_write0(&self) -> Promise<Undefined> {
        self.inner
            .call("atomicWrite", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The atomicWrite method.
    /// [`WebTransportWriter.atomicWrite`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportWriter/atomicWrite)
    pub fn atomic_write1(&self, chunk: &Any) -> Promise<Undefined> {
        self.inner
            .call("atomicWrite", &[chunk.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl WebTransportWriter {
    /// The commit method.
    /// [`WebTransportWriter.commit`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportWriter/commit)
    pub fn commit(&self) -> Undefined {
        self.inner.call("commit", &[]).as_::<Undefined>()
    }
}
