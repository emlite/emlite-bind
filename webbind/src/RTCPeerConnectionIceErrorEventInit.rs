use super::*;




/// The RTCPeerConnectionIceErrorEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnectionIceErrorEventInit {
    inner: Any,
}

impl FromVal for RTCPeerConnectionIceErrorEventInit {
    fn from_val(v: &Any) -> Self {
        RTCPeerConnectionIceErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCPeerConnectionIceErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCPeerConnectionIceErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCPeerConnectionIceErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCPeerConnectionIceErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCPeerConnectionIceErrorEventInit> for Any {
    fn from(s: RTCPeerConnectionIceErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCPeerConnectionIceErrorEventInit> for Any {
    fn from(s: &RTCPeerConnectionIceErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl RTCPeerConnectionIceErrorEventInit {
    /// Getter of the `address` attribute.
    pub fn address(&self) -> JsString {
        self.inner.get("address").as_::<JsString>()
    }

    /// Setter of the `address` attribute.
    pub fn set_address(&mut self, value: &JsString) {
        self.inner.set("address", value);
    }
}
impl RTCPeerConnectionIceErrorEventInit {
    /// Getter of the `port` attribute.
    pub fn port(&self) -> u16 {
        self.inner.get("port").as_::<u16>()
    }

    /// Setter of the `port` attribute.
    pub fn set_port(&mut self, value: u16) {
        self.inner.set("port", value);
    }
}
impl RTCPeerConnectionIceErrorEventInit {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl RTCPeerConnectionIceErrorEventInit {
    /// Getter of the `errorCode` attribute.
    pub fn error_code(&self) -> u16 {
        self.inner.get("errorCode").as_::<u16>()
    }

    /// Setter of the `errorCode` attribute.
    pub fn set_error_code(&mut self, value: u16) {
        self.inner.set("errorCode", value);
    }
}
impl RTCPeerConnectionIceErrorEventInit {
    /// Getter of the `errorText` attribute.
    pub fn error_text(&self) -> JsString {
        self.inner.get("errorText").as_::<JsString>()
    }

    /// Setter of the `errorText` attribute.
    pub fn set_error_text(&mut self, value: &JsString) {
        self.inner.set("errorText", value);
    }
}
