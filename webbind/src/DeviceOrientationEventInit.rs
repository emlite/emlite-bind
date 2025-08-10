use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceOrientationEventInit {
    inner: Any,
}
impl FromVal for DeviceOrientationEventInit {
    fn from_val(v: &Any) -> Self {
        DeviceOrientationEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DeviceOrientationEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DeviceOrientationEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DeviceOrientationEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DeviceOrientationEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DeviceOrientationEventInit> for Any {
    fn from(s: DeviceOrientationEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DeviceOrientationEventInit> for Any {
    fn from(s: &DeviceOrientationEventInit) -> Any {
        s.inner.clone()
    }
}

impl DeviceOrientationEventInit {
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl DeviceOrientationEventInit {
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
impl DeviceOrientationEventInit {
    pub fn gamma(&self) -> f64 {
        self.inner.get("gamma").as_::<f64>()
    }

    pub fn set_gamma(&mut self, value: f64) {
        self.inner.set("gamma", value);
    }
}
impl DeviceOrientationEventInit {
    pub fn absolute(&self) -> bool {
        self.inner.get("absolute").as_::<bool>()
    }

    pub fn set_absolute(&mut self, value: bool) {
        self.inner.set("absolute", value);
    }
}
