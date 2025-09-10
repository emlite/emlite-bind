use super::*;

/// The DeviceMotionEventAccelerationInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMotionEventAccelerationInit {
    inner: Any,
}

impl FromVal for DeviceMotionEventAccelerationInit {
    fn from_val(v: &Any) -> Self {
        DeviceMotionEventAccelerationInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DeviceMotionEventAccelerationInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeviceMotionEventAccelerationInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeviceMotionEventAccelerationInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeviceMotionEventAccelerationInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DeviceMotionEventAccelerationInit> for Any {
    fn from(s: DeviceMotionEventAccelerationInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeviceMotionEventAccelerationInit> for Any {
    fn from(s: &DeviceMotionEventAccelerationInit) -> Any {
        s.inner.clone()
    }
}

impl DeviceMotionEventAccelerationInit {
    /// Getter of the `x` attribute.
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    /// Setter of the `x` attribute.
    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DeviceMotionEventAccelerationInit {
    /// Getter of the `y` attribute.
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    /// Setter of the `y` attribute.
    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DeviceMotionEventAccelerationInit {
    /// Getter of the `z` attribute.
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

    /// Setter of the `z` attribute.
    pub fn set_z(&mut self, value: f64) {
        self.inner.set("z", value);
    }
}
