use super::*;

/// The RTCEncodedAudioFrameOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedAudioFrameOptions {
    inner: Any,
}

impl FromVal for RTCEncodedAudioFrameOptions {
    fn from_val(v: &Any) -> Self {
        RTCEncodedAudioFrameOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCEncodedAudioFrameOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCEncodedAudioFrameOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCEncodedAudioFrameOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCEncodedAudioFrameOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCEncodedAudioFrameOptions> for Any {
    fn from(s: RTCEncodedAudioFrameOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCEncodedAudioFrameOptions> for Any {
    fn from(s: &RTCEncodedAudioFrameOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCEncodedAudioFrameOptions {
    /// Getter of the `metadata` attribute.
    pub fn metadata(&self) -> RTCEncodedAudioFrameMetadata {
        self.inner
            .get("metadata")
            .as_::<RTCEncodedAudioFrameMetadata>()
    }

    /// Setter of the `metadata` attribute.
    pub fn set_metadata(&mut self, value: &RTCEncodedAudioFrameMetadata) {
        self.inner.set("metadata", value);
    }
}
