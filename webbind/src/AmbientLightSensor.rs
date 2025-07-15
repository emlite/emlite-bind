use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AmbientLightSensor {
    inner: Sensor,
}
impl FromVal for AmbientLightSensor {
    fn from_val(v: &emlite::Val) -> Self {
        AmbientLightSensor {
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
impl core::ops::Deref for AmbientLightSensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AmbientLightSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AmbientLightSensor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AmbientLightSensor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AmbientLightSensor> for emlite::Val {
    fn from(s: AmbientLightSensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AmbientLightSensor> for emlite::Val {
    fn from(s: &AmbientLightSensor) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AmbientLightSensor);

impl AmbientLightSensor {
    pub fn new0() -> AmbientLightSensor {
        Self {
            inner: emlite::Val::global("AmbientLightSensor")
                .new(&[])
                .as_::<Sensor>(),
        }
    }

    pub fn new1(sensor_options: &Any) -> AmbientLightSensor {
        Self {
            inner: emlite::Val::global("AmbientLightSensor")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl AmbientLightSensor {
    pub fn illuminance(&self) -> f64 {
        self.inner.get("illuminance").as_::<f64>()
    }
}
