use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelativeOrientationSensor {
    inner: OrientationSensor,
}
impl FromVal for RelativeOrientationSensor {
    fn from_val(v: &emlite::Val) -> Self {
        RelativeOrientationSensor {
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
impl core::ops::Deref for RelativeOrientationSensor {
    type Target = OrientationSensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RelativeOrientationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RelativeOrientationSensor> for emlite::Val {
    fn from(s: RelativeOrientationSensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RelativeOrientationSensor {
    pub fn new0() -> RelativeOrientationSensor {
        Self {
            inner: emlite::Val::global("RelativeOrientationSensor")
                .new(&[])
                .as_::<OrientationSensor>(),
        }
    }

    pub fn new1(sensor_options: jsbind::Any) -> RelativeOrientationSensor {
        Self {
            inner: emlite::Val::global("RelativeOrientationSensor")
                .new(&[sensor_options.into()])
                .as_::<OrientationSensor>(),
        }
    }
}
