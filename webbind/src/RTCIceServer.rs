use super::*;




/// The RTCIceServer dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceServer {
    inner: Any,
}

impl FromVal for RTCIceServer {
    fn from_val(v: &Any) -> Self {
        RTCIceServer { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIceServer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIceServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIceServer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIceServer {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIceServer> for Any {
    fn from(s: RTCIceServer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIceServer> for Any {
    fn from(s: &RTCIceServer) -> Any {
        s.inner.clone()
    }
}

impl RTCIceServer {
    /// Getter of the `urls` attribute.
    pub fn urls(&self) -> Any {
        self.inner.get("urls").as_::<Any>()
    }

    /// Setter of the `urls` attribute.
    pub fn set_urls(&mut self, value: &Any) {
        self.inner.set("urls", value);
    }
}
impl RTCIceServer {
    /// Getter of the `username` attribute.
    pub fn username(&self) -> JsString {
        self.inner.get("username").as_::<JsString>()
    }

    /// Setter of the `username` attribute.
    pub fn set_username(&mut self, value: &JsString) {
        self.inner.set("username", value);
    }
}
impl RTCIceServer {
    /// Getter of the `credential` attribute.
    pub fn credential(&self) -> JsString {
        self.inner.get("credential").as_::<JsString>()
    }

    /// Setter of the `credential` attribute.
    pub fn set_credential(&mut self, value: &JsString) {
        self.inner.set("credential", value);
    }
}
