use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Accelerometer {
    inner: Sensor,
}
impl FromVal for Accelerometer {
    fn from_val(v: &emlite::Val) -> Self {
        Accelerometer {
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
impl core::ops::Deref for Accelerometer {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Accelerometer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Accelerometer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Accelerometer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Accelerometer> for emlite::Val {
    fn from(s: Accelerometer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Accelerometer);

impl Accelerometer {
    pub fn new0() -> Accelerometer {
        Self {
            inner: emlite::Val::global("Accelerometer")
                .new(&[])
                .as_::<Sensor>(),
        }
    }

    pub fn new1(options: Any) -> Accelerometer {
        Self {
            inner: emlite::Val::global("Accelerometer")
                .new(&[options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl Accelerometer {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl Accelerometer {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl Accelerometer {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
