use super::*;

/// The LinearAccelerationSensor class.
/// [`LinearAccelerationSensor`](https://developer.mozilla.org/en-US/docs/Web/API/LinearAccelerationSensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LinearAccelerationSensor {
    inner: Accelerometer,
}

impl FromVal for LinearAccelerationSensor {
    fn from_val(v: &Any) -> Self {
        LinearAccelerationSensor {
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

impl core::ops::Deref for LinearAccelerationSensor {
    type Target = Accelerometer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LinearAccelerationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LinearAccelerationSensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LinearAccelerationSensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<LinearAccelerationSensor> for Any {
    fn from(s: LinearAccelerationSensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LinearAccelerationSensor> for Any {
    fn from(s: &LinearAccelerationSensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(LinearAccelerationSensor);

impl LinearAccelerationSensor {
    /// The `new LinearAccelerationSensor(..)` constructor, creating a new LinearAccelerationSensor instance
    pub fn new() -> LinearAccelerationSensor {
        Self {
            inner: Any::global("LinearAccelerationSensor")
                .new(&[])
                .as_::<Accelerometer>(),
        }
    }
}

impl LinearAccelerationSensor {
    /// The `new LinearAccelerationSensor(..)` constructor, creating a new LinearAccelerationSensor instance
    pub fn new_with_options(options: &AccelerometerSensorOptions) -> LinearAccelerationSensor {
        Self {
            inner: Any::global("LinearAccelerationSensor")
                .new(&[options.into()])
                .as_::<Accelerometer>(),
        }
    }
}
