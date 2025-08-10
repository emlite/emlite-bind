use super::*;

/// The AudioBufferOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioBufferOptions {
    inner: Any,
}

impl FromVal for AudioBufferOptions {
    fn from_val(v: &Any) -> Self {
        AudioBufferOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioBufferOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioBufferOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioBufferOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioBufferOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioBufferOptions> for Any {
    fn from(s: AudioBufferOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioBufferOptions> for Any {
    fn from(s: &AudioBufferOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioBufferOptions {
    /// Getter of the `numberOfChannels` attribute.
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }

    /// Setter of the `numberOfChannels` attribute.
    pub fn set_number_of_channels(&mut self, value: u32) {
        self.inner.set("numberOfChannels", value);
    }
}
impl AudioBufferOptions {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
impl AudioBufferOptions {
    /// Getter of the `sampleRate` attribute.
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }

    /// Setter of the `sampleRate` attribute.
    pub fn set_sample_rate(&mut self, value: f32) {
        self.inner.set("sampleRate", value);
    }
}
