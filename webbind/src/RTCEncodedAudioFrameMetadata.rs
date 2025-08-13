use super::*;




/// The RTCEncodedAudioFrameMetadata dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedAudioFrameMetadata {
    inner: Any,
}

impl FromVal for RTCEncodedAudioFrameMetadata {
    fn from_val(v: &Any) -> Self {
        RTCEncodedAudioFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCEncodedAudioFrameMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCEncodedAudioFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCEncodedAudioFrameMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCEncodedAudioFrameMetadata {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCEncodedAudioFrameMetadata> for Any {
    fn from(s: RTCEncodedAudioFrameMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCEncodedAudioFrameMetadata> for Any {
    fn from(s: &RTCEncodedAudioFrameMetadata) -> Any {
        s.inner.clone()
    }
}

impl RTCEncodedAudioFrameMetadata {
    /// Getter of the `sequenceNumber` attribute.
    pub fn sequence_number(&self) -> i16 {
        self.inner.get("sequenceNumber").as_::<i16>()
    }

    /// Setter of the `sequenceNumber` attribute.
    pub fn set_sequence_number(&mut self, value: i16) {
        self.inner.set("sequenceNumber", value);
    }
}
impl RTCEncodedAudioFrameMetadata {
    /// Getter of the `audioLevel` attribute.
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    /// Setter of the `audioLevel` attribute.
    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
