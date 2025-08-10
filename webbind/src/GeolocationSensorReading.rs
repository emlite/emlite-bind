use super::*;

/// The GeolocationSensorReading dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GeolocationSensorReading {
    inner: Any,
}

impl FromVal for GeolocationSensorReading {
    fn from_val(v: &Any) -> Self {
        GeolocationSensorReading { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GeolocationSensorReading {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GeolocationSensorReading {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GeolocationSensorReading {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GeolocationSensorReading {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GeolocationSensorReading> for Any {
    fn from(s: GeolocationSensorReading) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GeolocationSensorReading> for Any {
    fn from(s: &GeolocationSensorReading) -> Any {
        s.inner.clone()
    }
}

impl GeolocationSensorReading {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: &Any) {
        self.inner.set("timestamp", value);
    }
}
impl GeolocationSensorReading {
    /// Getter of the `latitude` attribute.
    pub fn latitude(&self) -> f64 {
        self.inner.get("latitude").as_::<f64>()
    }

    /// Setter of the `latitude` attribute.
    pub fn set_latitude(&mut self, value: f64) {
        self.inner.set("latitude", value);
    }
}
impl GeolocationSensorReading {
    /// Getter of the `longitude` attribute.
    pub fn longitude(&self) -> f64 {
        self.inner.get("longitude").as_::<f64>()
    }

    /// Setter of the `longitude` attribute.
    pub fn set_longitude(&mut self, value: f64) {
        self.inner.set("longitude", value);
    }
}
impl GeolocationSensorReading {
    /// Getter of the `altitude` attribute.
    pub fn altitude(&self) -> f64 {
        self.inner.get("altitude").as_::<f64>()
    }

    /// Setter of the `altitude` attribute.
    pub fn set_altitude(&mut self, value: f64) {
        self.inner.set("altitude", value);
    }
}
impl GeolocationSensorReading {
    /// Getter of the `accuracy` attribute.
    pub fn accuracy(&self) -> f64 {
        self.inner.get("accuracy").as_::<f64>()
    }

    /// Setter of the `accuracy` attribute.
    pub fn set_accuracy(&mut self, value: f64) {
        self.inner.set("accuracy", value);
    }
}
impl GeolocationSensorReading {
    /// Getter of the `altitudeAccuracy` attribute.
    pub fn altitude_accuracy(&self) -> f64 {
        self.inner.get("altitudeAccuracy").as_::<f64>()
    }

    /// Setter of the `altitudeAccuracy` attribute.
    pub fn set_altitude_accuracy(&mut self, value: f64) {
        self.inner.set("altitudeAccuracy", value);
    }
}
impl GeolocationSensorReading {
    /// Getter of the `heading` attribute.
    pub fn heading(&self) -> f64 {
        self.inner.get("heading").as_::<f64>()
    }

    /// Setter of the `heading` attribute.
    pub fn set_heading(&mut self, value: f64) {
        self.inner.set("heading", value);
    }
}
impl GeolocationSensorReading {
    /// Getter of the `speed` attribute.
    pub fn speed(&self) -> f64 {
        self.inner.get("speed").as_::<f64>()
    }

    /// Setter of the `speed` attribute.
    pub fn set_speed(&mut self, value: f64) {
        self.inner.set("speed", value);
    }
}
