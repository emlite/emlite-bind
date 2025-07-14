use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMotionEventAcceleration {
    inner: emlite::Val,
}
impl FromVal for DeviceMotionEventAcceleration {
    fn from_val(v: &emlite::Val) -> Self {
        DeviceMotionEventAcceleration {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DeviceMotionEventAcceleration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DeviceMotionEventAcceleration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DeviceMotionEventAcceleration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DeviceMotionEventAcceleration {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DeviceMotionEventAcceleration> for emlite::Val {
    fn from(s: DeviceMotionEventAcceleration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DeviceMotionEventAcceleration);

impl DeviceMotionEventAcceleration {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl DeviceMotionEventAcceleration {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl DeviceMotionEventAcceleration {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
