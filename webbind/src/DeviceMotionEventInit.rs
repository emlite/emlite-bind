use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMotionEventInit {
    inner: Any,
}
impl FromVal for DeviceMotionEventInit {
    fn from_val(v: &Any) -> Self {
        DeviceMotionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DeviceMotionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DeviceMotionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DeviceMotionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DeviceMotionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DeviceMotionEventInit> for Any {
    fn from(s: DeviceMotionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DeviceMotionEventInit> for Any {
    fn from(s: &DeviceMotionEventInit) -> Any {
        s.inner.clone()
    }
}

impl DeviceMotionEventInit {
    pub fn acceleration(&self) -> DeviceMotionEventAccelerationInit {
        self.inner
            .get("acceleration")
            .as_::<DeviceMotionEventAccelerationInit>()
    }

    pub fn set_acceleration(&mut self, value: &DeviceMotionEventAccelerationInit) {
        self.inner.set("acceleration", value);
    }
}
impl DeviceMotionEventInit {
    pub fn acceleration_including_gravity(&self) -> DeviceMotionEventAccelerationInit {
        self.inner
            .get("accelerationIncludingGravity")
            .as_::<DeviceMotionEventAccelerationInit>()
    }

    pub fn set_acceleration_including_gravity(
        &mut self,
        value: &DeviceMotionEventAccelerationInit,
    ) {
        self.inner.set("accelerationIncludingGravity", value);
    }
}
impl DeviceMotionEventInit {
    pub fn rotation_rate(&self) -> DeviceMotionEventRotationRateInit {
        self.inner
            .get("rotationRate")
            .as_::<DeviceMotionEventRotationRateInit>()
    }

    pub fn set_rotation_rate(&mut self, value: &DeviceMotionEventRotationRateInit) {
        self.inner.set("rotationRate", value);
    }
}
impl DeviceMotionEventInit {
    pub fn interval(&self) -> f64 {
        self.inner.get("interval").as_::<f64>()
    }

    pub fn set_interval(&mut self, value: f64) {
        self.inner.set("interval", value);
    }
}
