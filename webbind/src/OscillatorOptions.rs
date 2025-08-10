use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OscillatorOptions {
    inner: Any,
}
impl FromVal for OscillatorOptions {
    fn from_val(v: &Any) -> Self {
        OscillatorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OscillatorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OscillatorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OscillatorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OscillatorOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OscillatorOptions> for Any {
    fn from(s: OscillatorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OscillatorOptions> for Any {
    fn from(s: &OscillatorOptions) -> Any {
        s.inner.clone()
    }
}

impl OscillatorOptions {
    pub fn type_(&self) -> OscillatorType {
        self.inner.get("type").as_::<OscillatorType>()
    }

    pub fn set_type_(&mut self, value: &OscillatorType) {
        self.inner.set("type", value);
    }
}
impl OscillatorOptions {
    pub fn frequency(&self) -> f32 {
        self.inner.get("frequency").as_::<f32>()
    }

    pub fn set_frequency(&mut self, value: f32) {
        self.inner.set("frequency", value);
    }
}
impl OscillatorOptions {
    pub fn detune(&self) -> f32 {
        self.inner.get("detune").as_::<f32>()
    }

    pub fn set_detune(&mut self, value: f32) {
        self.inner.set("detune", value);
    }
}
impl OscillatorOptions {
    pub fn periodic_wave(&self) -> PeriodicWave {
        self.inner.get("periodicWave").as_::<PeriodicWave>()
    }

    pub fn set_periodic_wave(&mut self, value: &PeriodicWave) {
        self.inner.set("periodicWave", value);
    }
}
