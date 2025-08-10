use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioProcessingEventInit {
    inner: Any,
}
impl FromVal for AudioProcessingEventInit {
    fn from_val(v: &Any) -> Self {
        AudioProcessingEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioProcessingEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioProcessingEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioProcessingEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioProcessingEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioProcessingEventInit> for Any {
    fn from(s: AudioProcessingEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioProcessingEventInit> for Any {
    fn from(s: &AudioProcessingEventInit) -> Any {
        s.inner.clone()
    }
}

impl AudioProcessingEventInit {
    pub fn playback_time(&self) -> f64 {
        self.inner.get("playbackTime").as_::<f64>()
    }

    pub fn set_playback_time(&mut self, value: f64) {
        self.inner.set("playbackTime", value);
    }
}
impl AudioProcessingEventInit {
    pub fn input_buffer(&self) -> AudioBuffer {
        self.inner.get("inputBuffer").as_::<AudioBuffer>()
    }

    pub fn set_input_buffer(&mut self, value: &AudioBuffer) {
        self.inner.set("inputBuffer", value);
    }
}
impl AudioProcessingEventInit {
    pub fn output_buffer(&self) -> AudioBuffer {
        self.inner.get("outputBuffer").as_::<AudioBuffer>()
    }

    pub fn set_output_buffer(&mut self, value: &AudioBuffer) {
        self.inner.set("outputBuffer", value);
    }
}
