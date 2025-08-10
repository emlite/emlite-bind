use super::*;

/// The AudioProcessingEvent class.
/// [`AudioProcessingEvent`](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioProcessingEvent {
    inner: Event,
}
impl FromVal for AudioProcessingEvent {
    fn from_val(v: &Any) -> Self {
        AudioProcessingEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for AudioProcessingEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioProcessingEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioProcessingEvent> for Any {
    fn from(s: AudioProcessingEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioProcessingEvent> for Any {
    fn from(s: &AudioProcessingEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioProcessingEvent);

impl AudioProcessingEvent {
    /// The `new AudioProcessingEvent(..)` constructor, creating a new AudioProcessingEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &AudioProcessingEventInit,
    ) -> AudioProcessingEvent {
        Self {
            inner: Any::global("AudioProcessingEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl AudioProcessingEvent {
    /// Getter of the `playbackTime` attribute.
    /// [`AudioProcessingEvent.playbackTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/playbackTime)
    pub fn playback_time(&self) -> f64 {
        self.inner.get("playbackTime").as_::<f64>()
    }
}
impl AudioProcessingEvent {
    /// Getter of the `inputBuffer` attribute.
    /// [`AudioProcessingEvent.inputBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/inputBuffer)
    pub fn input_buffer(&self) -> AudioBuffer {
        self.inner.get("inputBuffer").as_::<AudioBuffer>()
    }
}
impl AudioProcessingEvent {
    /// Getter of the `outputBuffer` attribute.
    /// [`AudioProcessingEvent.outputBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/outputBuffer)
    pub fn output_buffer(&self) -> AudioBuffer {
        self.inner.get("outputBuffer").as_::<AudioBuffer>()
    }
}
