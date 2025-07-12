use super::*;

#[derive(Clone, Debug)]
pub struct Magnetometer {
    inner: Sensor,
}
impl FromVal for Magnetometer {
    fn from_val(v: &emlite::Val) -> Self {
        Magnetometer {
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
impl std::ops::Deref for Magnetometer {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Magnetometer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Magnetometer> for emlite::Val {
    fn from(s: Magnetometer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Magnetometer {
    pub fn new0() -> Magnetometer {
        Self {
            inner: emlite::Val::global("Magnetometer").new(&[]).as_::<Sensor>(),
        }
    }

    pub fn new1(sensor_options: jsbind::Any) -> Magnetometer {
        Self {
            inner: emlite::Val::global("Magnetometer")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl Magnetometer {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl Magnetometer {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl Magnetometer {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
