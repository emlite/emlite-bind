use super::*;




/// The GeolocationSensor class.
/// [`GeolocationSensor`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GeolocationSensor {
    inner: Sensor,
}

impl FromVal for GeolocationSensor {
    fn from_val(v: &Any) -> Self {
        GeolocationSensor { inner: Sensor::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GeolocationSensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GeolocationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GeolocationSensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GeolocationSensor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GeolocationSensor> for Any {
    fn from(s: GeolocationSensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GeolocationSensor> for Any {
    fn from(s: &GeolocationSensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GeolocationSensor);



impl GeolocationSensor {
    /// The `new GeolocationSensor(..)` constructor, creating a new GeolocationSensor instance
    pub fn new0() -> GeolocationSensor {
        Self {
            inner: Any::global("GeolocationSensor").new(&[]).as_::<Sensor>(),
        }
    }

    /// The `new GeolocationSensor(..)` constructor, creating a new GeolocationSensor instance
    pub fn new1(options: &GeolocationSensorOptions) -> GeolocationSensor {
        Self {
            inner: Any::global("GeolocationSensor").new(&[options.into()]).as_::<Sensor>(),
        }
    }

}
impl GeolocationSensor {
    /// The read method.
    /// [`GeolocationSensor.read`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/read)
    pub fn read0() -> Promise<GeolocationSensorReading> {
        Any::global("GeolocationSensor").call("read", &[]).as_::<Promise<GeolocationSensorReading>>()
    }
    /// The read method.
    /// [`GeolocationSensor.read`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/read)
    pub fn read1(read_options: &ReadOptions) -> Promise<GeolocationSensorReading> {
        Any::global("GeolocationSensor").call("read", &[read_options.into(), ]).as_::<Promise<GeolocationSensorReading>>()
    }
}
impl GeolocationSensor {
    /// Getter of the `latitude` attribute.
    /// [`GeolocationSensor.latitude`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/latitude)
    pub fn latitude(&self) -> f64 {
        self.inner.get("latitude").as_::<f64>()
    }

}
impl GeolocationSensor {
    /// Getter of the `longitude` attribute.
    /// [`GeolocationSensor.longitude`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/longitude)
    pub fn longitude(&self) -> f64 {
        self.inner.get("longitude").as_::<f64>()
    }

}
impl GeolocationSensor {
    /// Getter of the `altitude` attribute.
    /// [`GeolocationSensor.altitude`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/altitude)
    pub fn altitude(&self) -> f64 {
        self.inner.get("altitude").as_::<f64>()
    }

}
impl GeolocationSensor {
    /// Getter of the `accuracy` attribute.
    /// [`GeolocationSensor.accuracy`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/accuracy)
    pub fn accuracy(&self) -> f64 {
        self.inner.get("accuracy").as_::<f64>()
    }

}
impl GeolocationSensor {
    /// Getter of the `altitudeAccuracy` attribute.
    /// [`GeolocationSensor.altitudeAccuracy`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/altitudeAccuracy)
    pub fn altitude_accuracy(&self) -> f64 {
        self.inner.get("altitudeAccuracy").as_::<f64>()
    }

}
impl GeolocationSensor {
    /// Getter of the `heading` attribute.
    /// [`GeolocationSensor.heading`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/heading)
    pub fn heading(&self) -> f64 {
        self.inner.get("heading").as_::<f64>()
    }

}
impl GeolocationSensor {
    /// Getter of the `speed` attribute.
    /// [`GeolocationSensor.speed`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/speed)
    pub fn speed(&self) -> f64 {
        self.inner.get("speed").as_::<f64>()
    }

}
