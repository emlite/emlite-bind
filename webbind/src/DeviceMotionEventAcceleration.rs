use super::*;




/// The DeviceMotionEventAcceleration class.
/// [`DeviceMotionEventAcceleration`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventAcceleration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMotionEventAcceleration {
    inner: Any,
}

impl FromVal for DeviceMotionEventAcceleration {
    fn from_val(v: &Any) -> Self {
        DeviceMotionEventAcceleration { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DeviceMotionEventAcceleration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeviceMotionEventAcceleration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeviceMotionEventAcceleration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeviceMotionEventAcceleration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DeviceMotionEventAcceleration> for Any {
    fn from(s: DeviceMotionEventAcceleration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeviceMotionEventAcceleration> for Any {
    fn from(s: &DeviceMotionEventAcceleration) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DeviceMotionEventAcceleration);


impl DeviceMotionEventAcceleration {
    /// Getter of the `x` attribute.
    /// [`DeviceMotionEventAcceleration.x`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventAcceleration/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

}
impl DeviceMotionEventAcceleration {
    /// Getter of the `y` attribute.
    /// [`DeviceMotionEventAcceleration.y`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventAcceleration/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

}
impl DeviceMotionEventAcceleration {
    /// Getter of the `z` attribute.
    /// [`DeviceMotionEventAcceleration.z`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEventAcceleration/z)
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

}
