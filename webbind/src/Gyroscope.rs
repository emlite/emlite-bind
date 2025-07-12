use super::*;

#[derive(Clone, Debug)]
pub struct Gyroscope {
    inner: Sensor,
}
impl FromVal for Gyroscope {
    fn from_val(v: &emlite::Val) -> Self {
        Gyroscope {
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
impl std::ops::Deref for Gyroscope {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Gyroscope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Gyroscope> for emlite::Val {
    fn from(s: Gyroscope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Gyroscope {
    pub fn new0() -> Gyroscope {
        Self {
            inner: emlite::Val::global("Gyroscope").new(&[]).as_::<Sensor>(),
        }
    }

    pub fn new1(sensor_options: jsbind::Any) -> Gyroscope {
        Self {
            inner: emlite::Val::global("Gyroscope")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl Gyroscope {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl Gyroscope {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl Gyroscope {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
