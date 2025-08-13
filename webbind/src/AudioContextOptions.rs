use super::*;




/// The AudioContextOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioContextOptions {
    inner: Any,
}

impl FromVal for AudioContextOptions {
    fn from_val(v: &Any) -> Self {
        AudioContextOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioContextOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioContextOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioContextOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioContextOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioContextOptions> for Any {
    fn from(s: AudioContextOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioContextOptions> for Any {
    fn from(s: &AudioContextOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioContextOptions {
    /// Getter of the `latencyHint` attribute.
    pub fn latency_hint(&self) -> Any {
        self.inner.get("latencyHint").as_::<Any>()
    }

    /// Setter of the `latencyHint` attribute.
    pub fn set_latency_hint(&mut self, value: &Any) {
        self.inner.set("latencyHint", value);
    }
}
impl AudioContextOptions {
    /// Getter of the `sampleRate` attribute.
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }

    /// Setter of the `sampleRate` attribute.
    pub fn set_sample_rate(&mut self, value: f32) {
        self.inner.set("sampleRate", value);
    }
}
impl AudioContextOptions {
    /// Getter of the `sinkId` attribute.
    pub fn sink_id(&self) -> Any {
        self.inner.get("sinkId").as_::<Any>()
    }

    /// Setter of the `sinkId` attribute.
    pub fn set_sink_id(&mut self, value: &Any) {
        self.inner.set("sinkId", value);
    }
}
impl AudioContextOptions {
    /// Getter of the `renderSizeHint` attribute.
    pub fn render_size_hint(&self) -> Any {
        self.inner.get("renderSizeHint").as_::<Any>()
    }

    /// Setter of the `renderSizeHint` attribute.
    pub fn set_render_size_hint(&mut self, value: &Any) {
        self.inner.set("renderSizeHint", value);
    }
}
