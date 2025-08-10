use super::*;

/// The WebTransportSendStreamOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendStreamOptions {
    inner: Any,
}

impl FromVal for WebTransportSendStreamOptions {
    fn from_val(v: &Any) -> Self {
        WebTransportSendStreamOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportSendStreamOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportSendStreamOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportSendStreamOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportSendStreamOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebTransportSendStreamOptions> for Any {
    fn from(s: WebTransportSendStreamOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportSendStreamOptions> for Any {
    fn from(s: &WebTransportSendStreamOptions) -> Any {
        s.inner.clone()
    }
}

impl WebTransportSendStreamOptions {
    /// Getter of the `waitUntilAvailable` attribute.
    pub fn wait_until_available(&self) -> bool {
        self.inner.get("waitUntilAvailable").as_::<bool>()
    }

    /// Setter of the `waitUntilAvailable` attribute.
    pub fn set_wait_until_available(&mut self, value: bool) {
        self.inner.set("waitUntilAvailable", value);
    }
}
