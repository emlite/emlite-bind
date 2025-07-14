use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrientationSensor {
    inner: Sensor,
}
impl FromVal for OrientationSensor {
    fn from_val(v: &emlite::Val) -> Self {
        OrientationSensor {
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
impl core::ops::Deref for OrientationSensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OrientationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OrientationSensor> for emlite::Val {
    fn from(s: OrientationSensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OrientationSensor {
    pub fn quaternion(&self) -> jsbind::FrozenArray<f64> {
        self.inner
            .get("quaternion")
            .as_::<jsbind::FrozenArray<f64>>()
    }
}
impl OrientationSensor {
    pub fn populate_matrix(&self, target_matrix: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("populateMatrix", &[target_matrix.into()])
            .as_::<jsbind::Undefined>()
    }
}
