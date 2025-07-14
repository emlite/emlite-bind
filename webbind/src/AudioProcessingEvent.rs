use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioProcessingEvent {
    inner: Event,
}
impl FromVal for AudioProcessingEvent {
    fn from_val(v: &emlite::Val) -> Self {
        AudioProcessingEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioProcessingEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioProcessingEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioProcessingEvent> for emlite::Val {
    fn from(s: AudioProcessingEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioProcessingEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> AudioProcessingEvent {
        Self {
            inner: emlite::Val::global("AudioProcessingEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl AudioProcessingEvent {
    pub fn playback_time(&self) -> f64 {
        self.inner.get("playbackTime").as_::<f64>()
    }
}
impl AudioProcessingEvent {
    pub fn input_buffer(&self) -> AudioBuffer {
        self.inner.get("inputBuffer").as_::<AudioBuffer>()
    }
}
impl AudioProcessingEvent {
    pub fn output_buffer(&self) -> AudioBuffer {
        self.inner.get("outputBuffer").as_::<AudioBuffer>()
    }
}
