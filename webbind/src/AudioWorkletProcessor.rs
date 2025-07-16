use super::*;

/// The AudioWorkletProcessor class.
/// [`AudioWorkletProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioWorkletProcessor {
    inner: Any,
}
impl FromVal for AudioWorkletProcessor {
    fn from_val(v: &Any) -> Self {
        AudioWorkletProcessor {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioWorkletProcessor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioWorkletProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioWorkletProcessor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioWorkletProcessor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioWorkletProcessor> for Any {
    fn from(s: AudioWorkletProcessor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioWorkletProcessor> for Any {
    fn from(s: &AudioWorkletProcessor) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioWorkletProcessor);

impl AudioWorkletProcessor {
    /// The `new AudioWorkletProcessor(..)` constructor, creating a new AudioWorkletProcessor instance
    pub fn new() -> AudioWorkletProcessor {
        Self {
            inner: Any::global("AudioWorkletProcessor").new(&[]).as_::<Any>(),
        }
    }
}
impl AudioWorkletProcessor {
    /// Getter of the `port` attribute.
    /// [`AudioWorkletProcessor.port`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/port)
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }
}
