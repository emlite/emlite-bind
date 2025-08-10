use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpCodecParameters {
    inner: Any,
}
impl FromVal for RTCRtpCodecParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpCodecParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpCodecParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpCodecParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpCodecParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpCodecParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpCodecParameters> for Any {
    fn from(s: RTCRtpCodecParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpCodecParameters> for Any {
    fn from(s: &RTCRtpCodecParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpCodecParameters {
    pub fn payload_type(&self) -> u8 {
        self.inner.get("payloadType").as_::<u8>()
    }

    pub fn set_payload_type(&mut self, value: u8) {
        self.inner.set("payloadType", value);
    }
}
