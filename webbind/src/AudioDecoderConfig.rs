use super::*;

/// The AudioDecoderConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoderConfig {
    inner: Any,
}

impl FromVal for AudioDecoderConfig {
    fn from_val(v: &Any) -> Self {
        AudioDecoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioDecoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioDecoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioDecoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioDecoderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioDecoderConfig> for Any {
    fn from(s: AudioDecoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioDecoderConfig> for Any {
    fn from(s: &AudioDecoderConfig) -> Any {
        s.inner.clone()
    }
}

impl AudioDecoderConfig {
    /// Getter of the `codec` attribute.
    pub fn codec(&self) -> JsString {
        self.inner.get("codec").as_::<JsString>()
    }

    /// Setter of the `codec` attribute.
    pub fn set_codec(&mut self, value: &JsString) {
        self.inner.set("codec", value);
    }
}
impl AudioDecoderConfig {
    /// Getter of the `sampleRate` attribute.
    pub fn sample_rate(&self) -> u32 {
        self.inner.get("sampleRate").as_::<u32>()
    }

    /// Setter of the `sampleRate` attribute.
    pub fn set_sample_rate(&mut self, value: u32) {
        self.inner.set("sampleRate", value);
    }
}
impl AudioDecoderConfig {
    /// Getter of the `numberOfChannels` attribute.
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }

    /// Setter of the `numberOfChannels` attribute.
    pub fn set_number_of_channels(&mut self, value: u32) {
        self.inner.set("numberOfChannels", value);
    }
}
impl AudioDecoderConfig {
    /// Getter of the `description` attribute.
    pub fn description(&self) -> Any {
        self.inner.get("description").as_::<Any>()
    }

    /// Setter of the `description` attribute.
    pub fn set_description(&mut self, value: &Any) {
        self.inner.set("description", value);
    }
}
