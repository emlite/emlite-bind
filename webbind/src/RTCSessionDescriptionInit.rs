use super::*;




/// The RTCSessionDescriptionInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSessionDescriptionInit {
    inner: Any,
}

impl FromVal for RTCSessionDescriptionInit {
    fn from_val(v: &Any) -> Self {
        RTCSessionDescriptionInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCSessionDescriptionInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCSessionDescriptionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCSessionDescriptionInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCSessionDescriptionInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCSessionDescriptionInit> for Any {
    fn from(s: RTCSessionDescriptionInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCSessionDescriptionInit> for Any {
    fn from(s: &RTCSessionDescriptionInit) -> Any {
        s.inner.clone()
    }
}

impl RTCSessionDescriptionInit {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> RTCSdpType {
        self.inner.get("type").as_::<RTCSdpType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &RTCSdpType) {
        self.inner.set("type", value);
    }
}
impl RTCSessionDescriptionInit {
    /// Getter of the `sdp` attribute.
    pub fn sdp(&self) -> JsString {
        self.inner.get("sdp").as_::<JsString>()
    }

    /// Setter of the `sdp` attribute.
    pub fn set_sdp(&mut self, value: &JsString) {
        self.inner.set("sdp", value);
    }
}
