use super::*;

/// The WebTransportCloseInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportCloseInfo {
    inner: Any,
}

impl FromVal for WebTransportCloseInfo {
    fn from_val(v: &Any) -> Self {
        WebTransportCloseInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportCloseInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportCloseInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportCloseInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportCloseInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebTransportCloseInfo> for Any {
    fn from(s: WebTransportCloseInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportCloseInfo> for Any {
    fn from(s: &WebTransportCloseInfo) -> Any {
        s.inner.clone()
    }
}

impl WebTransportCloseInfo {
    /// Getter of the `closeCode` attribute.
    pub fn close_code(&self) -> u32 {
        self.inner.get("closeCode").as_::<u32>()
    }

    /// Setter of the `closeCode` attribute.
    pub fn set_close_code(&mut self, value: u32) {
        self.inner.set("closeCode", value);
    }
}
impl WebTransportCloseInfo {
    /// Getter of the `reason` attribute.
    pub fn reason(&self) -> JsString {
        self.inner.get("reason").as_::<JsString>()
    }

    /// Setter of the `reason` attribute.
    pub fn set_reason(&mut self, value: &JsString) {
        self.inner.set("reason", value);
    }
}
