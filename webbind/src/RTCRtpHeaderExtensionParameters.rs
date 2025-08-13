use super::*;




/// The RTCRtpHeaderExtensionParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpHeaderExtensionParameters {
    inner: Any,
}

impl FromVal for RTCRtpHeaderExtensionParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpHeaderExtensionParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpHeaderExtensionParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpHeaderExtensionParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpHeaderExtensionParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpHeaderExtensionParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpHeaderExtensionParameters> for Any {
    fn from(s: RTCRtpHeaderExtensionParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpHeaderExtensionParameters> for Any {
    fn from(s: &RTCRtpHeaderExtensionParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpHeaderExtensionParameters {
    /// Getter of the `uri` attribute.
    pub fn uri(&self) -> JsString {
        self.inner.get("uri").as_::<JsString>()
    }

    /// Setter of the `uri` attribute.
    pub fn set_uri(&mut self, value: &JsString) {
        self.inner.set("uri", value);
    }
}
impl RTCRtpHeaderExtensionParameters {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> u16 {
        self.inner.get("id").as_::<u16>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: u16) {
        self.inner.set("id", value);
    }
}
impl RTCRtpHeaderExtensionParameters {
    /// Getter of the `encrypted` attribute.
    pub fn encrypted(&self) -> bool {
        self.inner.get("encrypted").as_::<bool>()
    }

    /// Setter of the `encrypted` attribute.
    pub fn set_encrypted(&mut self, value: bool) {
        self.inner.set("encrypted", value);
    }
}
