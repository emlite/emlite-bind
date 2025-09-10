use super::*;

/// The AudioEncoderConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioEncoderConfig {
    inner: Any,
}

impl FromVal for AudioEncoderConfig {
    fn from_val(v: &Any) -> Self {
        AudioEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioEncoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioEncoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioEncoderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioEncoderConfig> for Any {
    fn from(s: AudioEncoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioEncoderConfig> for Any {
    fn from(s: &AudioEncoderConfig) -> Any {
        s.inner.clone()
    }
}

impl AudioEncoderConfig {
    /// Getter of the `codec` attribute.
    pub fn codec(&self) -> JsString {
        self.inner.get("codec").as_::<JsString>()
    }

    /// Setter of the `codec` attribute.
    pub fn set_codec(&mut self, value: &JsString) {
        self.inner.set("codec", value);
    }
}
impl AudioEncoderConfig {
    /// Getter of the `sampleRate` attribute.
    pub fn sample_rate(&self) -> u32 {
        self.inner.get("sampleRate").as_::<u32>()
    }

    /// Setter of the `sampleRate` attribute.
    pub fn set_sample_rate(&mut self, value: u32) {
        self.inner.set("sampleRate", value);
    }
}
impl AudioEncoderConfig {
    /// Getter of the `numberOfChannels` attribute.
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }

    /// Setter of the `numberOfChannels` attribute.
    pub fn set_number_of_channels(&mut self, value: u32) {
        self.inner.set("numberOfChannels", value);
    }
}
impl AudioEncoderConfig {
    /// Getter of the `bitrate` attribute.
    pub fn bitrate(&self) -> u64 {
        self.inner.get("bitrate").as_::<u64>()
    }

    /// Setter of the `bitrate` attribute.
    pub fn set_bitrate(&mut self, value: u64) {
        self.inner.set("bitrate", value);
    }
}
impl AudioEncoderConfig {
    /// Getter of the `bitrateMode` attribute.
    pub fn bitrate_mode(&self) -> BitrateMode {
        self.inner.get("bitrateMode").as_::<BitrateMode>()
    }

    /// Setter of the `bitrateMode` attribute.
    pub fn set_bitrate_mode(&mut self, value: &BitrateMode) {
        self.inner.set("bitrateMode", value);
    }
}
