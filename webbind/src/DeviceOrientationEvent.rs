use super::*;

/// The DeviceOrientationEvent class.
/// [`DeviceOrientationEvent`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceOrientationEvent {
    inner: Event,
}
impl FromVal for DeviceOrientationEvent {
    fn from_val(v: &Any) -> Self {
        DeviceOrientationEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DeviceOrientationEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DeviceOrientationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DeviceOrientationEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DeviceOrientationEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DeviceOrientationEvent> for Any {
    fn from(s: DeviceOrientationEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DeviceOrientationEvent> for Any {
    fn from(s: &DeviceOrientationEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DeviceOrientationEvent);

impl DeviceOrientationEvent {
    /// The `new DeviceOrientationEvent(..)` constructor, creating a new DeviceOrientationEvent instance
    pub fn new0(type_: &str) -> DeviceOrientationEvent {
        Self {
            inner: Any::global("DeviceOrientationEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new DeviceOrientationEvent(..)` constructor, creating a new DeviceOrientationEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> DeviceOrientationEvent {
        Self {
            inner: Any::global("DeviceOrientationEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl DeviceOrientationEvent {
    /// Getter of the `alpha` attribute.
    /// [`DeviceOrientationEvent.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/alpha)
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }
}
impl DeviceOrientationEvent {
    /// Getter of the `beta` attribute.
    /// [`DeviceOrientationEvent.beta`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/beta)
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }
}
impl DeviceOrientationEvent {
    /// Getter of the `gamma` attribute.
    /// [`DeviceOrientationEvent.gamma`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/gamma)
    pub fn gamma(&self) -> f64 {
        self.inner.get("gamma").as_::<f64>()
    }
}
impl DeviceOrientationEvent {
    /// Getter of the `absolute` attribute.
    /// [`DeviceOrientationEvent.absolute`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/absolute)
    pub fn absolute(&self) -> bool {
        self.inner.get("absolute").as_::<bool>()
    }
}
impl DeviceOrientationEvent {
    /// The requestPermission method.
    /// [`DeviceOrientationEvent.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/requestPermission)
    pub fn request_permission0() -> Promise<PermissionState> {
        Any::global("DeviceOrientationEvent")
            .call("requestPermission", &[])
            .as_::<Promise<PermissionState>>()
    }
    /// The requestPermission method.
    /// [`DeviceOrientationEvent.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/requestPermission)
    pub fn request_permission1(absolute: bool) -> Promise<PermissionState> {
        Any::global("DeviceOrientationEvent")
            .call("requestPermission", &[absolute.into()])
            .as_::<Promise<PermissionState>>()
    }
}
