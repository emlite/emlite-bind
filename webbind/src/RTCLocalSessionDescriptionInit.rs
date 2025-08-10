use super::*;

/// The RTCLocalSessionDescriptionInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCLocalSessionDescriptionInit {
    inner: Any,
}

impl FromVal for RTCLocalSessionDescriptionInit {
    fn from_val(v: &Any) -> Self {
        RTCLocalSessionDescriptionInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCLocalSessionDescriptionInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCLocalSessionDescriptionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCLocalSessionDescriptionInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCLocalSessionDescriptionInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCLocalSessionDescriptionInit> for Any {
    fn from(s: RTCLocalSessionDescriptionInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCLocalSessionDescriptionInit> for Any {
    fn from(s: &RTCLocalSessionDescriptionInit) -> Any {
        s.inner.clone()
    }
}

impl RTCLocalSessionDescriptionInit {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> RTCSdpType {
        self.inner.get("type").as_::<RTCSdpType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &RTCSdpType) {
        self.inner.set("type", value);
    }
}
impl RTCLocalSessionDescriptionInit {
    /// Getter of the `sdp` attribute.
    pub fn sdp(&self) -> JsString {
        self.inner.get("sdp").as_::<JsString>()
    }

    /// Setter of the `sdp` attribute.
    pub fn set_sdp(&mut self, value: &JsString) {
        self.inner.set("sdp", value);
    }
}
