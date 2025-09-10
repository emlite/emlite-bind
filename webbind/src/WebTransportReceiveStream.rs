use super::*;

/// The WebTransportReceiveStream class.
/// [`WebTransportReceiveStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportReceiveStream {
    inner: ReadableStream,
}

impl FromVal for WebTransportReceiveStream {
    fn from_val(v: &Any) -> Self {
        WebTransportReceiveStream {
            inner: ReadableStream::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportReceiveStream {
    type Target = ReadableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportReceiveStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportReceiveStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportReceiveStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebTransportReceiveStream> for Any {
    fn from(s: WebTransportReceiveStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportReceiveStream> for Any {
    fn from(s: &WebTransportReceiveStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebTransportReceiveStream);

impl WebTransportReceiveStream {
    /// The getStats method.
    /// [`WebTransportReceiveStream.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream/getStats)
    pub fn get_stats(&self) -> Promise<WebTransportReceiveStreamStats> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<WebTransportReceiveStreamStats>>()
    }
}
