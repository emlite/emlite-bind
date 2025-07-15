use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GravitySensor {
    inner: Accelerometer,
}
impl FromVal for GravitySensor {
    fn from_val(v: &emlite::Val) -> Self {
        GravitySensor {
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
impl core::ops::Deref for GravitySensor {
    type Target = Accelerometer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GravitySensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GravitySensor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GravitySensor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GravitySensor> for emlite::Val {
    fn from(s: GravitySensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GravitySensor> for emlite::Val {
    fn from(s: &GravitySensor) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GravitySensor);

impl GravitySensor {
    pub fn new0() -> GravitySensor {
        Self {
            inner: emlite::Val::global("GravitySensor")
                .new(&[])
                .as_::<Accelerometer>(),
        }
    }

    pub fn new1(options: Any) -> GravitySensor {
        Self {
            inner: emlite::Val::global("GravitySensor")
                .new(&[options.into()])
                .as_::<Accelerometer>(),
        }
    }
}
