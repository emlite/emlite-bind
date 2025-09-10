use super::*;

/// The OscillatorOptions dictionary.
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
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> OscillatorType {
        self.inner.get("type").as_::<OscillatorType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &OscillatorType) {
        self.inner.set("type", value);
    }
}
impl OscillatorOptions {
    /// Getter of the `frequency` attribute.
    pub fn frequency(&self) -> f32 {
        self.inner.get("frequency").as_::<f32>()
    }

    /// Setter of the `frequency` attribute.
    pub fn set_frequency(&mut self, value: f32) {
        self.inner.set("frequency", value);
    }
}
impl OscillatorOptions {
    /// Getter of the `detune` attribute.
    pub fn detune(&self) -> f32 {
        self.inner.get("detune").as_::<f32>()
    }

    /// Setter of the `detune` attribute.
    pub fn set_detune(&mut self, value: f32) {
        self.inner.set("detune", value);
    }
}
impl OscillatorOptions {
    /// Getter of the `periodicWave` attribute.
    pub fn periodic_wave(&self) -> PeriodicWave {
        self.inner.get("periodicWave").as_::<PeriodicWave>()
    }

    /// Setter of the `periodicWave` attribute.
    pub fn set_periodic_wave(&mut self, value: &PeriodicWave) {
        self.inner.set("periodicWave", value);
    }
}
