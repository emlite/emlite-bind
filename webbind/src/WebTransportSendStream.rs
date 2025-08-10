use super::*;

/// The WebTransportSendStream class.
/// [`WebTransportSendStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendStream {
    inner: WritableStream,
}

impl FromVal for WebTransportSendStream {
    fn from_val(v: &Any) -> Self {
        WebTransportSendStream {
            inner: WritableStream::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportSendStream {
    type Target = WritableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportSendStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportSendStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportSendStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebTransportSendStream> for Any {
    fn from(s: WebTransportSendStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportSendStream> for Any {
    fn from(s: &WebTransportSendStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebTransportSendStream);

impl WebTransportSendStream {
    /// Getter of the `sendGroup` attribute.
    /// [`WebTransportSendStream.sendGroup`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendGroup)
    pub fn send_group(&self) -> WebTransportSendGroup {
        self.inner.get("sendGroup").as_::<WebTransportSendGroup>()
    }

    /// Setter of the `sendGroup` attribute.
    /// [`WebTransportSendStream.sendGroup`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendGroup)
    pub fn set_send_group(&mut self, value: &WebTransportSendGroup) {
        self.inner.set("sendGroup", value);
    }
}
impl WebTransportSendStream {
    /// Getter of the `sendOrder` attribute.
    /// [`WebTransportSendStream.sendOrder`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendOrder)
    pub fn send_order(&self) -> i64 {
        self.inner.get("sendOrder").as_::<i64>()
    }

    /// Setter of the `sendOrder` attribute.
    /// [`WebTransportSendStream.sendOrder`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendOrder)
    pub fn set_send_order(&mut self, value: i64) {
        self.inner.set("sendOrder", value);
    }
}
impl WebTransportSendStream {
    /// The getStats method.
    /// [`WebTransportSendStream.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/getStats)
    pub fn get_stats(&self) -> Promise<WebTransportSendStreamStats> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<WebTransportSendStreamStats>>()
    }
}
impl WebTransportSendStream {
    /// The getWriter method.
    /// [`WebTransportSendStream.getWriter`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/getWriter)
    pub fn get_writer(&self) -> WebTransportWriter {
        self.inner
            .call("getWriter", &[])
            .as_::<WebTransportWriter>()
    }
}
