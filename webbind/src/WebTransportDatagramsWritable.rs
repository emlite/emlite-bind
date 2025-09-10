use super::*;

/// The WebTransportDatagramsWritable class.
/// [`WebTransportDatagramsWritable`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramsWritable)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportDatagramsWritable {
    inner: WritableStream,
}

impl FromVal for WebTransportDatagramsWritable {
    fn from_val(v: &Any) -> Self {
        WebTransportDatagramsWritable {
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

impl core::ops::Deref for WebTransportDatagramsWritable {
    type Target = WritableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportDatagramsWritable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportDatagramsWritable {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportDatagramsWritable {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebTransportDatagramsWritable> for Any {
    fn from(s: WebTransportDatagramsWritable) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportDatagramsWritable> for Any {
    fn from(s: &WebTransportDatagramsWritable) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebTransportDatagramsWritable);

impl WebTransportDatagramsWritable {
    /// Getter of the `sendGroup` attribute.
    /// [`WebTransportDatagramsWritable.sendGroup`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramsWritable/sendGroup)
    pub fn send_group(&self) -> WebTransportSendGroup {
        self.inner.get("sendGroup").as_::<WebTransportSendGroup>()
    }

    /// Setter of the `sendGroup` attribute.
    /// [`WebTransportDatagramsWritable.sendGroup`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramsWritable/sendGroup)
    pub fn set_send_group(&mut self, value: &WebTransportSendGroup) {
        self.inner.set("sendGroup", value);
    }
}
impl WebTransportDatagramsWritable {
    /// Getter of the `sendOrder` attribute.
    /// [`WebTransportDatagramsWritable.sendOrder`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramsWritable/sendOrder)
    pub fn send_order(&self) -> i64 {
        self.inner.get("sendOrder").as_::<i64>()
    }

    /// Setter of the `sendOrder` attribute.
    /// [`WebTransportDatagramsWritable.sendOrder`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramsWritable/sendOrder)
    pub fn set_send_order(&mut self, value: i64) {
        self.inner.set("sendOrder", value);
    }
}
