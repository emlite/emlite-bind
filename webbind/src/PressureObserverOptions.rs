use super::*;

/// The PressureObserverOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PressureObserverOptions {
    inner: Any,
}

impl FromVal for PressureObserverOptions {
    fn from_val(v: &Any) -> Self {
        PressureObserverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PressureObserverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PressureObserverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PressureObserverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PressureObserverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PressureObserverOptions> for Any {
    fn from(s: PressureObserverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PressureObserverOptions> for Any {
    fn from(s: &PressureObserverOptions) -> Any {
        s.inner.clone()
    }
}

impl PressureObserverOptions {
    /// Getter of the `sampleInterval` attribute.
    pub fn sample_interval(&self) -> u32 {
        self.inner.get("sampleInterval").as_::<u32>()
    }

    /// Setter of the `sampleInterval` attribute.
    pub fn set_sample_interval(&mut self, value: u32) {
        self.inner.set("sampleInterval", value);
    }
}
