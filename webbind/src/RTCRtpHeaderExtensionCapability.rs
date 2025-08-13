use super::*;




/// The RTCRtpHeaderExtensionCapability dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpHeaderExtensionCapability {
    inner: Any,
}

impl FromVal for RTCRtpHeaderExtensionCapability {
    fn from_val(v: &Any) -> Self {
        RTCRtpHeaderExtensionCapability { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpHeaderExtensionCapability {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpHeaderExtensionCapability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpHeaderExtensionCapability {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpHeaderExtensionCapability {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpHeaderExtensionCapability> for Any {
    fn from(s: RTCRtpHeaderExtensionCapability) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpHeaderExtensionCapability> for Any {
    fn from(s: &RTCRtpHeaderExtensionCapability) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpHeaderExtensionCapability {
    /// Getter of the `uri` attribute.
    pub fn uri(&self) -> JsString {
        self.inner.get("uri").as_::<JsString>()
    }

    /// Setter of the `uri` attribute.
    pub fn set_uri(&mut self, value: &JsString) {
        self.inner.set("uri", value);
    }
}
