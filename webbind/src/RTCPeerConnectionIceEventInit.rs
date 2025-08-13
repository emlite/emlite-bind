use super::*;




/// The RTCPeerConnectionIceEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnectionIceEventInit {
    inner: Any,
}

impl FromVal for RTCPeerConnectionIceEventInit {
    fn from_val(v: &Any) -> Self {
        RTCPeerConnectionIceEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCPeerConnectionIceEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCPeerConnectionIceEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCPeerConnectionIceEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCPeerConnectionIceEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCPeerConnectionIceEventInit> for Any {
    fn from(s: RTCPeerConnectionIceEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCPeerConnectionIceEventInit> for Any {
    fn from(s: &RTCPeerConnectionIceEventInit) -> Any {
        s.inner.clone()
    }
}

impl RTCPeerConnectionIceEventInit {
    /// Getter of the `candidate` attribute.
    pub fn candidate(&self) -> RTCIceCandidate {
        self.inner.get("candidate").as_::<RTCIceCandidate>()
    }

    /// Setter of the `candidate` attribute.
    pub fn set_candidate(&mut self, value: &RTCIceCandidate) {
        self.inner.set("candidate", value);
    }
}
impl RTCPeerConnectionIceEventInit {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
