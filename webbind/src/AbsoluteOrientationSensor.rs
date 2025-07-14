use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbsoluteOrientationSensor {
    inner: OrientationSensor,
}
impl FromVal for AbsoluteOrientationSensor {
    fn from_val(v: &emlite::Val) -> Self {
        AbsoluteOrientationSensor {
            inner: OrientationSensor::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AbsoluteOrientationSensor {
    type Target = OrientationSensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AbsoluteOrientationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AbsoluteOrientationSensor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AbsoluteOrientationSensor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AbsoluteOrientationSensor> for emlite::Val {
    fn from(s: AbsoluteOrientationSensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AbsoluteOrientationSensor);

impl AbsoluteOrientationSensor {
    pub fn new0() -> AbsoluteOrientationSensor {
        Self {
            inner: emlite::Val::global("AbsoluteOrientationSensor")
                .new(&[])
                .as_::<OrientationSensor>(),
        }
    }

    pub fn new1(sensor_options: jsbind::Any) -> AbsoluteOrientationSensor {
        Self {
            inner: emlite::Val::global("AbsoluteOrientationSensor")
                .new(&[sensor_options.into()])
                .as_::<OrientationSensor>(),
        }
    }
}
