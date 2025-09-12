use super::*;

/// The AmbientLightSensor class.
/// [`AmbientLightSensor`](https://developer.mozilla.org/en-US/docs/Web/API/AmbientLightSensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AmbientLightSensor {
    inner: Sensor,
}

impl FromVal for AmbientLightSensor {
    fn from_val(v: &Any) -> Self {
        AmbientLightSensor {
            inner: Sensor::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AmbientLightSensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AmbientLightSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AmbientLightSensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AmbientLightSensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AmbientLightSensor> for Any {
    fn from(s: AmbientLightSensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AmbientLightSensor> for Any {
    fn from(s: &AmbientLightSensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AmbientLightSensor);

impl AmbientLightSensor {
    /// Getter of the `illuminance` attribute.
    /// [`AmbientLightSensor.illuminance`](https://developer.mozilla.org/en-US/docs/Web/API/AmbientLightSensor/illuminance)
    pub fn illuminance(&self) -> f64 {
        self.inner.get("illuminance").as_::<f64>()
    }
}

impl AmbientLightSensor {
    /// The `new AmbientLightSensor(..)` constructor, creating a new AmbientLightSensor instance
    pub fn new() -> AmbientLightSensor {
        Self {
            inner: Any::global("AmbientLightSensor").new(&[]).as_::<Sensor>(),
        }
    }
}

impl AmbientLightSensor {
    /// The `new AmbientLightSensor(..)` constructor, creating a new AmbientLightSensor instance
    pub fn new_with_sensor_options(sensor_options: &SensorOptions) -> AmbientLightSensor {
        Self {
            inner: Any::global("AmbientLightSensor")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
