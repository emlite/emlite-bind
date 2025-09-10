use super::*;

/// The RelativeOrientationSensor class.
/// [`RelativeOrientationSensor`](https://developer.mozilla.org/en-US/docs/Web/API/RelativeOrientationSensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RelativeOrientationSensor {
    inner: OrientationSensor,
}

impl FromVal for RelativeOrientationSensor {
    fn from_val(v: &Any) -> Self {
        RelativeOrientationSensor {
            inner: OrientationSensor::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RelativeOrientationSensor {
    type Target = OrientationSensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RelativeOrientationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RelativeOrientationSensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RelativeOrientationSensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RelativeOrientationSensor> for Any {
    fn from(s: RelativeOrientationSensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RelativeOrientationSensor> for Any {
    fn from(s: &RelativeOrientationSensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RelativeOrientationSensor);

impl RelativeOrientationSensor {
    /// The `new RelativeOrientationSensor(..)` constructor, creating a new RelativeOrientationSensor instance
    pub fn new0() -> RelativeOrientationSensor {
        Self {
            inner: Any::global("RelativeOrientationSensor")
                .new(&[])
                .as_::<OrientationSensor>(),
        }
    }

    /// The `new RelativeOrientationSensor(..)` constructor, creating a new RelativeOrientationSensor instance
    pub fn new1(sensor_options: &OrientationSensorOptions) -> RelativeOrientationSensor {
        Self {
            inner: Any::global("RelativeOrientationSensor")
                .new(&[sensor_options.into()])
                .as_::<OrientationSensor>(),
        }
    }
}
