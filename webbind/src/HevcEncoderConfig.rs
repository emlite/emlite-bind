use super::*;




/// The HevcEncoderConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HevcEncoderConfig {
    inner: Any,
}

impl FromVal for HevcEncoderConfig {
    fn from_val(v: &Any) -> Self {
        HevcEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HevcEncoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HevcEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HevcEncoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HevcEncoderConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HevcEncoderConfig> for Any {
    fn from(s: HevcEncoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HevcEncoderConfig> for Any {
    fn from(s: &HevcEncoderConfig) -> Any {
        s.inner.clone()
    }
}

impl HevcEncoderConfig {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> HevcBitstreamFormat {
        self.inner.get("format").as_::<HevcBitstreamFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &HevcBitstreamFormat) {
        self.inner.set("format", value);
    }
}
