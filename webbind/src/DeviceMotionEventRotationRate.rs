use super::*;

/// The DeviceMotionEventRotationRate class.
/// [`DeviceMotionEventRotationRate`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventRotationRate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMotionEventRotationRate {
    inner: Any,
}

impl FromVal for DeviceMotionEventRotationRate {
    fn from_val(v: &Any) -> Self {
        DeviceMotionEventRotationRate {
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

impl core::ops::Deref for DeviceMotionEventRotationRate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeviceMotionEventRotationRate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeviceMotionEventRotationRate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeviceMotionEventRotationRate {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DeviceMotionEventRotationRate> for Any {
    fn from(s: DeviceMotionEventRotationRate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeviceMotionEventRotationRate> for Any {
    fn from(s: &DeviceMotionEventRotationRate) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DeviceMotionEventRotationRate);

impl DeviceMotionEventRotationRate {
    /// Getter of the `alpha` attribute.
    /// [`DeviceMotionEventRotationRate.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventRotationRate/alpha)
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }
}
impl DeviceMotionEventRotationRate {
    /// Getter of the `beta` attribute.
    /// [`DeviceMotionEventRotationRate.beta`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventRotationRate/beta)
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }
}
impl DeviceMotionEventRotationRate {
    /// Getter of the `gamma` attribute.
    /// [`DeviceMotionEventRotationRate.gamma`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventRotationRate/gamma)
    pub fn gamma(&self) -> f64 {
        self.inner.get("gamma").as_::<f64>()
    }
}
