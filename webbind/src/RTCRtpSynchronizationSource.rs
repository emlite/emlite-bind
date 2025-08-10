use super::*;

/// The RTCRtpSynchronizationSource dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpSynchronizationSource {
    inner: Any,
}

impl FromVal for RTCRtpSynchronizationSource {
    fn from_val(v: &Any) -> Self {
        RTCRtpSynchronizationSource { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpSynchronizationSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpSynchronizationSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpSynchronizationSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpSynchronizationSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCRtpSynchronizationSource> for Any {
    fn from(s: RTCRtpSynchronizationSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpSynchronizationSource> for Any {
    fn from(s: &RTCRtpSynchronizationSource) -> Any {
        s.inner.clone()
    }
}
