use super::*;

/// The RTCRtpReceiveParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpReceiveParameters {
    inner: Any,
}

impl FromVal for RTCRtpReceiveParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpReceiveParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpReceiveParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpReceiveParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpReceiveParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpReceiveParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCRtpReceiveParameters> for Any {
    fn from(s: RTCRtpReceiveParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpReceiveParameters> for Any {
    fn from(s: &RTCRtpReceiveParameters) -> Any {
        s.inner.clone()
    }
}
