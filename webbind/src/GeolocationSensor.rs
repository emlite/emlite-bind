use super::*;

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
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    pub fn set_timestamp(&mut self, value: &Any) {
        self.inner.set("timestamp", value);
    }
}
impl GeolocationSensorReading {
    pub fn latitude(&self) -> f64 {
        self.inner.get("latitude").as_::<f64>()
    }

    pub fn set_latitude(&mut self, value: f64) {
        self.inner.set("latitude", value);
    }
}
impl GeolocationSensorReading {
    pub fn longitude(&self) -> f64 {
        self.inner.get("longitude").as_::<f64>()
    }

    pub fn set_longitude(&mut self, value: f64) {
        self.inner.set("longitude", value);
    }
}
impl GeolocationSensorReading {
    pub fn altitude(&self) -> f64 {
        self.inner.get("altitude").as_::<f64>()
    }

    pub fn set_altitude(&mut self, value: f64) {
        self.inner.set("altitude", value);
    }
}
impl GeolocationSensorReading {
    pub fn accuracy(&self) -> f64 {
        self.inner.get("accuracy").as_::<f64>()
    }

    pub fn set_accuracy(&mut self, value: f64) {
        self.inner.set("accuracy", value);
    }
}
impl GeolocationSensorReading {
    pub fn altitude_accuracy(&self) -> f64 {
        self.inner.get("altitudeAccuracy").as_::<f64>()
    }

    pub fn set_altitude_accuracy(&mut self, value: f64) {
        self.inner.set("altitudeAccuracy", value);
    }
}
impl GeolocationSensorReading {
    pub fn heading(&self) -> f64 {
        self.inner.get("heading").as_::<f64>()
    }

    pub fn set_heading(&mut self, value: f64) {
        self.inner.set("heading", value);
    }
}
impl GeolocationSensorReading {
    pub fn speed(&self) -> f64 {
        self.inner.get("speed").as_::<f64>()
    }

    pub fn set_speed(&mut self, value: f64) {
        self.inner.set("speed", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadOptions {
    inner: Any,
}
impl FromVal for ReadOptions {
    fn from_val(v: &Any) -> Self {
        ReadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReadOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReadOptions> for Any {
    fn from(s: ReadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReadOptions> for Any {
    fn from(s: &ReadOptions) -> Any {
        s.inner.clone()
    }
}

impl ReadOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The GeolocationSensor class.
/// [`GeolocationSensor`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GeolocationSensor {
    inner: Sensor,
}
impl FromVal for GeolocationSensor {
    fn from_val(v: &Any) -> Self {
        GeolocationSensor {
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
    pub fn new1(options: &Any) -> GeolocationSensor {
        Self {
            inner: Any::global("GeolocationSensor")
                .new(&[options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl GeolocationSensor {
    /// The read method.
    /// [`GeolocationSensor.read`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/read)
    pub fn read0() -> Promise<GeolocationSensorReading> {
        Any::global("GeolocationSensor")
            .call("read", &[])
            .as_::<Promise<GeolocationSensorReading>>()
    }
    /// The read method.
    /// [`GeolocationSensor.read`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationSensor/read)
    pub fn read1(read_options: &ReadOptions) -> Promise<GeolocationSensorReading> {
        Any::global("GeolocationSensor")
            .call("read", &[read_options.into()])
            .as_::<Promise<GeolocationSensorReading>>()
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
