use super::*;

/// The AudioWorkletGlobalScope class.
/// [`AudioWorkletGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for AudioWorkletGlobalScope {
    fn from_val(v: &Any) -> Self {
        AudioWorkletGlobalScope {
            inner: WorkletGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioWorkletGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioWorkletGlobalScope> for Any {
    fn from(s: AudioWorkletGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioWorkletGlobalScope> for Any {
    fn from(s: &AudioWorkletGlobalScope) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioWorkletGlobalScope);

impl AudioWorkletGlobalScope {
    /// The registerProcessor method.
    /// [`AudioWorkletGlobalScope.registerProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/registerProcessor)
    pub fn register_processor(&self, name: &DOMString, processor_ctor: &Function) -> Undefined {
        self.inner
            .call("registerProcessor", &[name.into(), processor_ctor.into()])
            .as_::<Undefined>()
    }
}
impl AudioWorkletGlobalScope {
    /// Getter of the `currentFrame` attribute.
    /// [`AudioWorkletGlobalScope.currentFrame`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentFrame)
    pub fn current_frame(&self) -> u64 {
        self.inner.get("currentFrame").as_::<u64>()
    }
}
impl AudioWorkletGlobalScope {
    /// Getter of the `currentTime` attribute.
    /// [`AudioWorkletGlobalScope.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentTime)
    pub fn current_time(&self) -> f64 {
        self.inner.get("currentTime").as_::<f64>()
    }
}
impl AudioWorkletGlobalScope {
    /// Getter of the `sampleRate` attribute.
    /// [`AudioWorkletGlobalScope.sampleRate`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/sampleRate)
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl AudioWorkletGlobalScope {
    /// Getter of the `renderQuantumSize` attribute.
    /// [`AudioWorkletGlobalScope.renderQuantumSize`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/renderQuantumSize)
    pub fn render_quantum_size(&self) -> u32 {
        self.inner.get("renderQuantumSize").as_::<u32>()
    }
}
impl AudioWorkletGlobalScope {
    /// Getter of the `port` attribute.
    /// [`AudioWorkletGlobalScope.port`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/port)
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }
}
