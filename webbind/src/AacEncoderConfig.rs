use super::*;

/// The AacEncoderConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AacEncoderConfig {
    inner: Any,
}

impl FromVal for AacEncoderConfig {
    fn from_val(v: &Any) -> Self {
        AacEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AacEncoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AacEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AacEncoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AacEncoderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AacEncoderConfig> for Any {
    fn from(s: AacEncoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AacEncoderConfig> for Any {
    fn from(s: &AacEncoderConfig) -> Any {
        s.inner.clone()
    }
}

impl AacEncoderConfig {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> AacBitstreamFormat {
        self.inner.get("format").as_::<AacBitstreamFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &AacBitstreamFormat) {
        self.inner.set("format", value);
    }
}
