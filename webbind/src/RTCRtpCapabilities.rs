use super::*;




/// The RTCRtpCapabilities dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpCapabilities {
    inner: Any,
}

impl FromVal for RTCRtpCapabilities {
    fn from_val(v: &Any) -> Self {
        RTCRtpCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpCapabilities {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpCapabilities> for Any {
    fn from(s: RTCRtpCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpCapabilities> for Any {
    fn from(s: &RTCRtpCapabilities) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpCapabilities {
    /// Getter of the `codecs` attribute.
    pub fn codecs(&self) -> TypedArray<RTCRtpCodec> {
        self.inner.get("codecs").as_::<TypedArray<RTCRtpCodec>>()
    }

    /// Setter of the `codecs` attribute.
    pub fn set_codecs(&mut self, value: &TypedArray<RTCRtpCodec>) {
        self.inner.set("codecs", value);
    }
}
impl RTCRtpCapabilities {
    /// Getter of the `headerExtensions` attribute.
    pub fn header_extensions(&self) -> TypedArray<RTCRtpHeaderExtensionCapability> {
        self.inner.get("headerExtensions").as_::<TypedArray<RTCRtpHeaderExtensionCapability>>()
    }

    /// Setter of the `headerExtensions` attribute.
    pub fn set_header_extensions(&mut self, value: &TypedArray<RTCRtpHeaderExtensionCapability>) {
        self.inner.set("headerExtensions", value);
    }
}
