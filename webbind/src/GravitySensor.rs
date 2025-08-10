use super::*;

/// The GravitySensor class.
/// [`GravitySensor`](https://developer.mozilla.org/en-US/docs/Web/API/GravitySensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GravitySensor {
    inner: Accelerometer,
}

impl FromVal for GravitySensor {
    fn from_val(v: &Any) -> Self {
        GravitySensor {
            inner: Accelerometer::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GravitySensor {
    type Target = Accelerometer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GravitySensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GravitySensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GravitySensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GravitySensor> for Any {
    fn from(s: GravitySensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GravitySensor> for Any {
    fn from(s: &GravitySensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GravitySensor);

impl GravitySensor {
    /// The `new GravitySensor(..)` constructor, creating a new GravitySensor instance
    pub fn new0() -> GravitySensor {
        Self {
            inner: Any::global("GravitySensor").new(&[]).as_::<Accelerometer>(),
        }
    }

    /// The `new GravitySensor(..)` constructor, creating a new GravitySensor instance
    pub fn new1(options: &AccelerometerSensorOptions) -> GravitySensor {
        Self {
            inner: Any::global("GravitySensor")
                .new(&[options.into()])
                .as_::<Accelerometer>(),
        }
    }
}
