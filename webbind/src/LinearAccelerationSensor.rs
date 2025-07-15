use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LinearAccelerationSensor {
    inner: Accelerometer,
}
impl FromVal for LinearAccelerationSensor {
    fn from_val(v: &emlite::Val) -> Self {
        LinearAccelerationSensor {
            inner: Accelerometer::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LinearAccelerationSensor {
    type Target = Accelerometer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LinearAccelerationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LinearAccelerationSensor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LinearAccelerationSensor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LinearAccelerationSensor> for emlite::Val {
    fn from(s: LinearAccelerationSensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(LinearAccelerationSensor);

impl LinearAccelerationSensor {
    pub fn new0() -> LinearAccelerationSensor {
        Self {
            inner: emlite::Val::global("LinearAccelerationSensor")
                .new(&[])
                .as_::<Accelerometer>(),
        }
    }

    pub fn new1(options: Any) -> LinearAccelerationSensor {
        Self {
            inner: emlite::Val::global("LinearAccelerationSensor")
                .new(&[options.into()])
                .as_::<Accelerometer>(),
        }
    }
}
