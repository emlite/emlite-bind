use super::*;




/// The AvcEncoderConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AvcEncoderConfig {
    inner: Any,
}

impl FromVal for AvcEncoderConfig {
    fn from_val(v: &Any) -> Self {
        AvcEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AvcEncoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AvcEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AvcEncoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AvcEncoderConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AvcEncoderConfig> for Any {
    fn from(s: AvcEncoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AvcEncoderConfig> for Any {
    fn from(s: &AvcEncoderConfig) -> Any {
        s.inner.clone()
    }
}

impl AvcEncoderConfig {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> AvcBitstreamFormat {
        self.inner.get("format").as_::<AvcBitstreamFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &AvcBitstreamFormat) {
        self.inner.set("format", value);
    }
}
