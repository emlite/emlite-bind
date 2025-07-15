use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for AudioWorkletGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        AudioWorkletGlobalScope {
            inner: WorkletGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for AudioWorkletGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioWorkletGlobalScope> for emlite::Val {
    fn from(s: AudioWorkletGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AudioWorkletGlobalScope);

impl AudioWorkletGlobalScope {
    pub fn register_processor(&self, name: DOMString, processor_ctor: Function) -> Undefined {
        self.inner
            .call("registerProcessor", &[name.into(), processor_ctor.into()])
            .as_::<Undefined>()
    }
}
impl AudioWorkletGlobalScope {
    pub fn current_frame(&self) -> u64 {
        self.inner.get("currentFrame").as_::<u64>()
    }
}
impl AudioWorkletGlobalScope {
    pub fn current_time(&self) -> f64 {
        self.inner.get("currentTime").as_::<f64>()
    }
}
impl AudioWorkletGlobalScope {
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl AudioWorkletGlobalScope {
    pub fn render_quantum_size(&self) -> u32 {
        self.inner.get("renderQuantumSize").as_::<u32>()
    }
}
impl AudioWorkletGlobalScope {
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }
}
