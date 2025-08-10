use super::*;

/// The AbsoluteOrientationSensor class.
/// [`AbsoluteOrientationSensor`](https://developer.mozilla.org/en-US/docs/Web/API/AbsoluteOrientationSensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbsoluteOrientationSensor {
    inner: OrientationSensor,
}

impl FromVal for AbsoluteOrientationSensor {
    fn from_val(v: &Any) -> Self {
        AbsoluteOrientationSensor {
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

impl core::ops::Deref for AbsoluteOrientationSensor {
    type Target = OrientationSensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AbsoluteOrientationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AbsoluteOrientationSensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AbsoluteOrientationSensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AbsoluteOrientationSensor> for Any {
    fn from(s: AbsoluteOrientationSensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AbsoluteOrientationSensor> for Any {
    fn from(s: &AbsoluteOrientationSensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AbsoluteOrientationSensor);

impl AbsoluteOrientationSensor {
    /// The `new AbsoluteOrientationSensor(..)` constructor, creating a new AbsoluteOrientationSensor instance
    pub fn new0() -> AbsoluteOrientationSensor {
        Self {
            inner: Any::global("AbsoluteOrientationSensor")
                .new(&[])
                .as_::<OrientationSensor>(),
        }
    }

    /// The `new AbsoluteOrientationSensor(..)` constructor, creating a new AbsoluteOrientationSensor instance
    pub fn new1(sensor_options: &OrientationSensorOptions) -> AbsoluteOrientationSensor {
        Self {
            inner: Any::global("AbsoluteOrientationSensor")
                .new(&[sensor_options.into()])
                .as_::<OrientationSensor>(),
        }
    }
}
