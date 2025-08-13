use super::*;




/// The PeriodicWaveConstraints dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicWaveConstraints {
    inner: Any,
}

impl FromVal for PeriodicWaveConstraints {
    fn from_val(v: &Any) -> Self {
        PeriodicWaveConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PeriodicWaveConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PeriodicWaveConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PeriodicWaveConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PeriodicWaveConstraints {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PeriodicWaveConstraints> for Any {
    fn from(s: PeriodicWaveConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PeriodicWaveConstraints> for Any {
    fn from(s: &PeriodicWaveConstraints) -> Any {
        s.inner.clone()
    }
}

impl PeriodicWaveConstraints {
    /// Getter of the `disableNormalization` attribute.
    pub fn disable_normalization(&self) -> bool {
        self.inner.get("disableNormalization").as_::<bool>()
    }

    /// Setter of the `disableNormalization` attribute.
    pub fn set_disable_normalization(&mut self, value: bool) {
        self.inner.set("disableNormalization", value);
    }
}
