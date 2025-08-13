use super::*;




/// The DeviceMotionEvent class.
/// [`DeviceMotionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceMotionEvent {
    inner: Event,
}

impl FromVal for DeviceMotionEvent {
    fn from_val(v: &Any) -> Self {
        DeviceMotionEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DeviceMotionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeviceMotionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeviceMotionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeviceMotionEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DeviceMotionEvent> for Any {
    fn from(s: DeviceMotionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeviceMotionEvent> for Any {
    fn from(s: &DeviceMotionEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DeviceMotionEvent);



impl DeviceMotionEvent {
    /// The `new DeviceMotionEvent(..)` constructor, creating a new DeviceMotionEvent instance
    pub fn new0(type_: &JsString) -> DeviceMotionEvent {
        Self {
            inner: Any::global("DeviceMotionEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    /// The `new DeviceMotionEvent(..)` constructor, creating a new DeviceMotionEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &DeviceMotionEventInit) -> DeviceMotionEvent {
        Self {
            inner: Any::global("DeviceMotionEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl DeviceMotionEvent {
    /// Getter of the `acceleration` attribute.
    /// [`DeviceMotionEvent.acceleration`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/acceleration)
    pub fn acceleration(&self) -> DeviceMotionEventAcceleration {
        self.inner.get("acceleration").as_::<DeviceMotionEventAcceleration>()
    }

}
impl DeviceMotionEvent {
    /// Getter of the `accelerationIncludingGravity` attribute.
    /// [`DeviceMotionEvent.accelerationIncludingGravity`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/accelerationIncludingGravity)
    pub fn acceleration_including_gravity(&self) -> DeviceMotionEventAcceleration {
        self.inner.get("accelerationIncludingGravity").as_::<DeviceMotionEventAcceleration>()
    }

}
impl DeviceMotionEvent {
    /// Getter of the `rotationRate` attribute.
    /// [`DeviceMotionEvent.rotationRate`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/rotationRate)
    pub fn rotation_rate(&self) -> DeviceMotionEventRotationRate {
        self.inner.get("rotationRate").as_::<DeviceMotionEventRotationRate>()
    }

}
impl DeviceMotionEvent {
    /// Getter of the `interval` attribute.
    /// [`DeviceMotionEvent.interval`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/interval)
    pub fn interval(&self) -> f64 {
        self.inner.get("interval").as_::<f64>()
    }

}
impl DeviceMotionEvent {
    /// The requestPermission method.
    /// [`DeviceMotionEvent.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/requestPermission)
    pub fn request_permission() -> Promise<PermissionState> {
        Any::global("DeviceMotionEvent").call("requestPermission", &[]).as_::<Promise<PermissionState>>()
    }
}
