use super::*;




/// The AudioEncoderSupport dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioEncoderSupport {
    inner: Any,
}

impl FromVal for AudioEncoderSupport {
    fn from_val(v: &Any) -> Self {
        AudioEncoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioEncoderSupport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioEncoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioEncoderSupport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioEncoderSupport {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioEncoderSupport> for Any {
    fn from(s: AudioEncoderSupport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioEncoderSupport> for Any {
    fn from(s: &AudioEncoderSupport) -> Any {
        s.inner.clone()
    }
}

impl AudioEncoderSupport {
    /// Getter of the `supported` attribute.
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    /// Setter of the `supported` attribute.
    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl AudioEncoderSupport {
    /// Getter of the `config` attribute.
    pub fn config(&self) -> AudioEncoderConfig {
        self.inner.get("config").as_::<AudioEncoderConfig>()
    }

    /// Setter of the `config` attribute.
    pub fn set_config(&mut self, value: &AudioEncoderConfig) {
        self.inner.set("config", value);
    }
}
