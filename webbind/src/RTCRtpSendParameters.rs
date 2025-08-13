use super::*;




/// The RTCRtpSendParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpSendParameters {
    inner: Any,
}

impl FromVal for RTCRtpSendParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpSendParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpSendParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpSendParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpSendParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpSendParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpSendParameters> for Any {
    fn from(s: RTCRtpSendParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpSendParameters> for Any {
    fn from(s: &RTCRtpSendParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpSendParameters {
    /// Getter of the `transactionId` attribute.
    pub fn transaction_id(&self) -> JsString {
        self.inner.get("transactionId").as_::<JsString>()
    }

    /// Setter of the `transactionId` attribute.
    pub fn set_transaction_id(&mut self, value: &JsString) {
        self.inner.set("transactionId", value);
    }
}
impl RTCRtpSendParameters {
    /// Getter of the `encodings` attribute.
    pub fn encodings(&self) -> TypedArray<RTCRtpEncodingParameters> {
        self.inner.get("encodings").as_::<TypedArray<RTCRtpEncodingParameters>>()
    }

    /// Setter of the `encodings` attribute.
    pub fn set_encodings(&mut self, value: &TypedArray<RTCRtpEncodingParameters>) {
        self.inner.set("encodings", value);
    }
}
