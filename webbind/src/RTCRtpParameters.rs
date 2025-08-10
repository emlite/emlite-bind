use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpParameters {
    inner: Any,
}
impl FromVal for RTCRtpParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpParameters> for Any {
    fn from(s: RTCRtpParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpParameters> for Any {
    fn from(s: &RTCRtpParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpParameters {
    pub fn header_extensions(&self) -> TypedArray<RTCRtpHeaderExtensionParameters> {
        self.inner
            .get("headerExtensions")
            .as_::<TypedArray<RTCRtpHeaderExtensionParameters>>()
    }

    pub fn set_header_extensions(&mut self, value: &TypedArray<RTCRtpHeaderExtensionParameters>) {
        self.inner.set("headerExtensions", value);
    }
}
impl RTCRtpParameters {
    pub fn rtcp(&self) -> RTCRtcpParameters {
        self.inner.get("rtcp").as_::<RTCRtcpParameters>()
    }

    pub fn set_rtcp(&mut self, value: &RTCRtcpParameters) {
        self.inner.set("rtcp", value);
    }
}
impl RTCRtpParameters {
    pub fn codecs(&self) -> TypedArray<RTCRtpCodecParameters> {
        self.inner
            .get("codecs")
            .as_::<TypedArray<RTCRtpCodecParameters>>()
    }

    pub fn set_codecs(&mut self, value: &TypedArray<RTCRtpCodecParameters>) {
        self.inner.set("codecs", value);
    }
}
