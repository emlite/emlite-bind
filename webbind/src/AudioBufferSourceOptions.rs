use super::*;

/// The AudioBufferSourceOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioBufferSourceOptions {
    inner: Any,
}

impl FromVal for AudioBufferSourceOptions {
    fn from_val(v: &Any) -> Self {
        AudioBufferSourceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioBufferSourceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioBufferSourceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioBufferSourceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioBufferSourceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioBufferSourceOptions> for Any {
    fn from(s: AudioBufferSourceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioBufferSourceOptions> for Any {
    fn from(s: &AudioBufferSourceOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioBufferSourceOptions {
    /// Getter of the `buffer` attribute.
    pub fn buffer(&self) -> AudioBuffer {
        self.inner.get("buffer").as_::<AudioBuffer>()
    }

    /// Setter of the `buffer` attribute.
    pub fn set_buffer(&mut self, value: &AudioBuffer) {
        self.inner.set("buffer", value);
    }
}
impl AudioBufferSourceOptions {
    /// Getter of the `detune` attribute.
    pub fn detune(&self) -> f32 {
        self.inner.get("detune").as_::<f32>()
    }

    /// Setter of the `detune` attribute.
    pub fn set_detune(&mut self, value: f32) {
        self.inner.set("detune", value);
    }
}
impl AudioBufferSourceOptions {
    /// Getter of the `loop` attribute.
    pub fn loop_(&self) -> bool {
        self.inner.get("loop").as_::<bool>()
    }

    /// Setter of the `loop` attribute.
    pub fn set_loop_(&mut self, value: bool) {
        self.inner.set("loop", value);
    }
}
impl AudioBufferSourceOptions {
    /// Getter of the `loopEnd` attribute.
    pub fn loop_end(&self) -> f64 {
        self.inner.get("loopEnd").as_::<f64>()
    }

    /// Setter of the `loopEnd` attribute.
    pub fn set_loop_end(&mut self, value: f64) {
        self.inner.set("loopEnd", value);
    }
}
impl AudioBufferSourceOptions {
    /// Getter of the `loopStart` attribute.
    pub fn loop_start(&self) -> f64 {
        self.inner.get("loopStart").as_::<f64>()
    }

    /// Setter of the `loopStart` attribute.
    pub fn set_loop_start(&mut self, value: f64) {
        self.inner.set("loopStart", value);
    }
}
impl AudioBufferSourceOptions {
    /// Getter of the `playbackRate` attribute.
    pub fn playback_rate(&self) -> f32 {
        self.inner.get("playbackRate").as_::<f32>()
    }

    /// Setter of the `playbackRate` attribute.
    pub fn set_playback_rate(&mut self, value: f32) {
        self.inner.set("playbackRate", value);
    }
}
