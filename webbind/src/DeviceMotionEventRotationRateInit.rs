use super::*;




/// The DeviceMotionEventRotationRateInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMotionEventRotationRateInit {
    inner: Any,
}

impl FromVal for DeviceMotionEventRotationRateInit {
    fn from_val(v: &Any) -> Self {
        DeviceMotionEventRotationRateInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DeviceMotionEventRotationRateInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeviceMotionEventRotationRateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeviceMotionEventRotationRateInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeviceMotionEventRotationRateInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DeviceMotionEventRotationRateInit> for Any {
    fn from(s: DeviceMotionEventRotationRateInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeviceMotionEventRotationRateInit> for Any {
    fn from(s: &DeviceMotionEventRotationRateInit) -> Any {
        s.inner.clone()
    }
}

impl DeviceMotionEventRotationRateInit {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl DeviceMotionEventRotationRateInit {
    /// Getter of the `beta` attribute.
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    /// Setter of the `beta` attribute.
    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
impl DeviceMotionEventRotationRateInit {
    /// Getter of the `gamma` attribute.
    pub fn gamma(&self) -> f64 {
        self.inner.get("gamma").as_::<f64>()
    }

    /// Setter of the `gamma` attribute.
    pub fn set_gamma(&mut self, value: f64) {
        self.inner.set("gamma", value);
    }
}
