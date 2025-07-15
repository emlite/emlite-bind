use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProximitySensor {
    inner: Sensor,
}
impl FromVal for ProximitySensor {
    fn from_val(v: &emlite::Val) -> Self {
        ProximitySensor {
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
impl core::ops::Deref for ProximitySensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProximitySensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ProximitySensor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ProximitySensor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ProximitySensor> for emlite::Val {
    fn from(s: ProximitySensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ProximitySensor);

impl ProximitySensor {
    pub fn new0() -> ProximitySensor {
        Self {
            inner: emlite::Val::global("ProximitySensor")
                .new(&[])
                .as_::<Sensor>(),
        }
    }

    pub fn new1(sensor_options: Any) -> ProximitySensor {
        Self {
            inner: emlite::Val::global("ProximitySensor")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl ProximitySensor {
    pub fn distance(&self) -> f64 {
        self.inner.get("distance").as_::<f64>()
    }
}
impl ProximitySensor {
    pub fn max(&self) -> f64 {
        self.inner.get("max").as_::<f64>()
    }
}
impl ProximitySensor {
    pub fn near(&self) -> bool {
        self.inner.get("near").as_::<bool>()
    }
}
