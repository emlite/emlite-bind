use super::*;

/// The AudioDataInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDataInit {
    inner: Any,
}

impl FromVal for AudioDataInit {
    fn from_val(v: &Any) -> Self {
        AudioDataInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioDataInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioDataInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioDataInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioDataInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioDataInit> for Any {
    fn from(s: AudioDataInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioDataInit> for Any {
    fn from(s: &AudioDataInit) -> Any {
        s.inner.clone()
    }
}

impl AudioDataInit {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> AudioSampleFormat {
        self.inner.get("format").as_::<AudioSampleFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &AudioSampleFormat) {
        self.inner.set("format", value);
    }
}
impl AudioDataInit {
    /// Getter of the `sampleRate` attribute.
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }

    /// Setter of the `sampleRate` attribute.
    pub fn set_sample_rate(&mut self, value: f32) {
        self.inner.set("sampleRate", value);
    }
}
impl AudioDataInit {
    /// Getter of the `numberOfFrames` attribute.
    pub fn number_of_frames(&self) -> u32 {
        self.inner.get("numberOfFrames").as_::<u32>()
    }

    /// Setter of the `numberOfFrames` attribute.
    pub fn set_number_of_frames(&mut self, value: u32) {
        self.inner.set("numberOfFrames", value);
    }
}
impl AudioDataInit {
    /// Getter of the `numberOfChannels` attribute.
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }

    /// Setter of the `numberOfChannels` attribute.
    pub fn set_number_of_channels(&mut self, value: u32) {
        self.inner.set("numberOfChannels", value);
    }
}
impl AudioDataInit {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
impl AudioDataInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl AudioDataInit {
    /// Getter of the `transfer` attribute.
    pub fn transfer(&self) -> TypedArray<ArrayBuffer> {
        self.inner.get("transfer").as_::<TypedArray<ArrayBuffer>>()
    }

    /// Setter of the `transfer` attribute.
    pub fn set_transfer(&mut self, value: &TypedArray<ArrayBuffer>) {
        self.inner.set("transfer", value);
    }
}
