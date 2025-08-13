use super::*;




/// The RTCRtpCodingParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpCodingParameters {
    inner: Any,
}

impl FromVal for RTCRtpCodingParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpCodingParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpCodingParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpCodingParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpCodingParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpCodingParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpCodingParameters> for Any {
    fn from(s: RTCRtpCodingParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpCodingParameters> for Any {
    fn from(s: &RTCRtpCodingParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpCodingParameters {
    /// Getter of the `rid` attribute.
    pub fn rid(&self) -> JsString {
        self.inner.get("rid").as_::<JsString>()
    }

    /// Setter of the `rid` attribute.
    pub fn set_rid(&mut self, value: &JsString) {
        self.inner.set("rid", value);
    }
}
