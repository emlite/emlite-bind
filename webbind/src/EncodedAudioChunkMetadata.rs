use super::*;




/// The EncodedAudioChunkMetadata dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncodedAudioChunkMetadata {
    inner: Any,
}

impl FromVal for EncodedAudioChunkMetadata {
    fn from_val(v: &Any) -> Self {
        EncodedAudioChunkMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EncodedAudioChunkMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncodedAudioChunkMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncodedAudioChunkMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncodedAudioChunkMetadata {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EncodedAudioChunkMetadata> for Any {
    fn from(s: EncodedAudioChunkMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncodedAudioChunkMetadata> for Any {
    fn from(s: &EncodedAudioChunkMetadata) -> Any {
        s.inner.clone()
    }
}

impl EncodedAudioChunkMetadata {
    /// Getter of the `decoderConfig` attribute.
    pub fn decoder_config(&self) -> AudioDecoderConfig {
        self.inner.get("decoderConfig").as_::<AudioDecoderConfig>()
    }

    /// Setter of the `decoderConfig` attribute.
    pub fn set_decoder_config(&mut self, value: &AudioDecoderConfig) {
        self.inner.set("decoderConfig", value);
    }
}
