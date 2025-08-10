use super::*;

/// The RTCEncodedVideoFrameOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedVideoFrameOptions {
    inner: Any,
}

impl FromVal for RTCEncodedVideoFrameOptions {
    fn from_val(v: &Any) -> Self {
        RTCEncodedVideoFrameOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCEncodedVideoFrameOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCEncodedVideoFrameOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCEncodedVideoFrameOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCEncodedVideoFrameOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCEncodedVideoFrameOptions> for Any {
    fn from(s: RTCEncodedVideoFrameOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCEncodedVideoFrameOptions> for Any {
    fn from(s: &RTCEncodedVideoFrameOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCEncodedVideoFrameOptions {
    /// Getter of the `metadata` attribute.
    pub fn metadata(&self) -> RTCEncodedVideoFrameMetadata {
        self.inner
            .get("metadata")
            .as_::<RTCEncodedVideoFrameMetadata>()
    }

    /// Setter of the `metadata` attribute.
    pub fn set_metadata(&mut self, value: &RTCEncodedVideoFrameMetadata) {
        self.inner.set("metadata", value);
    }
}
