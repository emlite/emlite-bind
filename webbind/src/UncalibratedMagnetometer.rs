use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UncalibratedMagnetometer {
    inner: Sensor,
}
impl FromVal for UncalibratedMagnetometer {
    fn from_val(v: &emlite::Val) -> Self {
        UncalibratedMagnetometer {
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
impl core::ops::Deref for UncalibratedMagnetometer {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UncalibratedMagnetometer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for UncalibratedMagnetometer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for UncalibratedMagnetometer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<UncalibratedMagnetometer> for emlite::Val {
    fn from(s: UncalibratedMagnetometer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(UncalibratedMagnetometer);

impl UncalibratedMagnetometer {
    pub fn new0() -> UncalibratedMagnetometer {
        Self {
            inner: emlite::Val::global("UncalibratedMagnetometer")
                .new(&[])
                .as_::<Sensor>(),
        }
    }

    pub fn new1(sensor_options: jsbind::Any) -> UncalibratedMagnetometer {
        Self {
            inner: emlite::Val::global("UncalibratedMagnetometer")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl UncalibratedMagnetometer {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    pub fn x_bias(&self) -> f64 {
        self.inner.get("xBias").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    pub fn y_bias(&self) -> f64 {
        self.inner.get("yBias").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    pub fn z_bias(&self) -> f64 {
        self.inner.get("zBias").as_::<f64>()
    }
}
