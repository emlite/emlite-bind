use super::*;




/// The WebTransportSendOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendOptions {
    inner: Any,
}

impl FromVal for WebTransportSendOptions {
    fn from_val(v: &Any) -> Self {
        WebTransportSendOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportSendOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportSendOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportSendOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportSendOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebTransportSendOptions> for Any {
    fn from(s: WebTransportSendOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportSendOptions> for Any {
    fn from(s: &WebTransportSendOptions) -> Any {
        s.inner.clone()
    }
}

impl WebTransportSendOptions {
    /// Getter of the `sendGroup` attribute.
    pub fn send_group(&self) -> WebTransportSendGroup {
        self.inner.get("sendGroup").as_::<WebTransportSendGroup>()
    }

    /// Setter of the `sendGroup` attribute.
    pub fn set_send_group(&mut self, value: &WebTransportSendGroup) {
        self.inner.set("sendGroup", value);
    }
}
impl WebTransportSendOptions {
    /// Getter of the `sendOrder` attribute.
    pub fn send_order(&self) -> i64 {
        self.inner.get("sendOrder").as_::<i64>()
    }

    /// Setter of the `sendOrder` attribute.
    pub fn set_send_order(&mut self, value: i64) {
        self.inner.set("sendOrder", value);
    }
}
