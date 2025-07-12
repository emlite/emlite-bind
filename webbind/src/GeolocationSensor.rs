use super::*;

#[derive(Clone, Debug)]
pub struct GeolocationSensorReading {
    inner: emlite::Val,
}
impl FromVal for GeolocationSensorReading {
    fn from_val(v: &emlite::Val) -> Self {
        GeolocationSensorReading { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GeolocationSensorReading {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GeolocationSensorReading {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GeolocationSensorReading> for emlite::Val {
    fn from(s: GeolocationSensorReading) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GeolocationSensorReading {
    pub fn timestamp(&self) -> jsbind::Any {
        self.inner.get("timestamp").as_::<jsbind::Any>()
    }

    pub fn set_timestamp(&mut self, value: jsbind::Any) {
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
#[derive(Clone, Debug)]
pub struct ReadOptions {
    inner: emlite::Val,
}
impl FromVal for ReadOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ReadOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ReadOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadOptions> for emlite::Val {
    fn from(s: ReadOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug)]
pub struct GeolocationSensor {
    inner: Sensor,
}
impl FromVal for GeolocationSensor {
    fn from_val(v: &emlite::Val) -> Self {
        GeolocationSensor {
            inner: Sensor::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GeolocationSensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GeolocationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GeolocationSensor> for emlite::Val {
    fn from(s: GeolocationSensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GeolocationSensor {
    pub fn new0() -> GeolocationSensor {
        Self {
            inner: emlite::Val::global("GeolocationSensor")
                .new(&[])
                .as_::<Sensor>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> GeolocationSensor {
        Self {
            inner: emlite::Val::global("GeolocationSensor")
                .new(&[options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl GeolocationSensor {
    pub fn read0() -> jsbind::Promise {
        emlite::Val::global("geolocationsensor")
            .call("read", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn read1(read_options: ReadOptions) -> jsbind::Promise {
        emlite::Val::global("geolocationsensor")
            .call("read", &[read_options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl GeolocationSensor {
    pub fn latitude(&self) -> f64 {
        self.inner.get("latitude").as_::<f64>()
    }
}
impl GeolocationSensor {
    pub fn longitude(&self) -> f64 {
        self.inner.get("longitude").as_::<f64>()
    }
}
impl GeolocationSensor {
    pub fn altitude(&self) -> f64 {
        self.inner.get("altitude").as_::<f64>()
    }
}
impl GeolocationSensor {
    pub fn accuracy(&self) -> f64 {
        self.inner.get("accuracy").as_::<f64>()
    }
}
impl GeolocationSensor {
    pub fn altitude_accuracy(&self) -> f64 {
        self.inner.get("altitudeAccuracy").as_::<f64>()
    }
}
impl GeolocationSensor {
    pub fn heading(&self) -> f64 {
        self.inner.get("heading").as_::<f64>()
    }
}
impl GeolocationSensor {
    pub fn speed(&self) -> f64 {
        self.inner.get("speed").as_::<f64>()
    }
}
