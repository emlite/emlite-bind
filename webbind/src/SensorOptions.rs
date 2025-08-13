use super::*;




/// The SensorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SensorOptions {
    inner: Any,
}

impl FromVal for SensorOptions {
    fn from_val(v: &Any) -> Self {
        SensorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SensorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SensorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SensorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SensorOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SensorOptions> for Any {
    fn from(s: SensorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SensorOptions> for Any {
    fn from(s: &SensorOptions) -> Any {
        s.inner.clone()
    }
}

impl SensorOptions {
    /// Getter of the `frequency` attribute.
    pub fn frequency(&self) -> f64 {
        self.inner.get("frequency").as_::<f64>()
    }

    /// Setter of the `frequency` attribute.
    pub fn set_frequency(&mut self, value: f64) {
        self.inner.set("frequency", value);
    }
}
