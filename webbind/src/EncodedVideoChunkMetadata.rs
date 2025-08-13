use super::*;




/// The EncodedVideoChunkMetadata dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncodedVideoChunkMetadata {
    inner: Any,
}

impl FromVal for EncodedVideoChunkMetadata {
    fn from_val(v: &Any) -> Self {
        EncodedVideoChunkMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EncodedVideoChunkMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncodedVideoChunkMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncodedVideoChunkMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncodedVideoChunkMetadata {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EncodedVideoChunkMetadata> for Any {
    fn from(s: EncodedVideoChunkMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncodedVideoChunkMetadata> for Any {
    fn from(s: &EncodedVideoChunkMetadata) -> Any {
        s.inner.clone()
    }
}

impl EncodedVideoChunkMetadata {
    /// Getter of the `decoderConfig` attribute.
    pub fn decoder_config(&self) -> VideoDecoderConfig {
        self.inner.get("decoderConfig").as_::<VideoDecoderConfig>()
    }

    /// Setter of the `decoderConfig` attribute.
    pub fn set_decoder_config(&mut self, value: &VideoDecoderConfig) {
        self.inner.set("decoderConfig", value);
    }
}
impl EncodedVideoChunkMetadata {
    /// Getter of the `svc` attribute.
    pub fn svc(&self) -> SvcOutputMetadata {
        self.inner.get("svc").as_::<SvcOutputMetadata>()
    }

    /// Setter of the `svc` attribute.
    pub fn set_svc(&mut self, value: &SvcOutputMetadata) {
        self.inner.set("svc", value);
    }
}
impl EncodedVideoChunkMetadata {
    /// Getter of the `alphaSideData` attribute.
    pub fn alpha_side_data(&self) -> Any {
        self.inner.get("alphaSideData").as_::<Any>()
    }

    /// Setter of the `alphaSideData` attribute.
    pub fn set_alpha_side_data(&mut self, value: &Any) {
        self.inner.set("alphaSideData", value);
    }
}
