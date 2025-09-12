use super::*;

/// The DeviceChangeEvent class.
/// [`DeviceChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceChangeEvent {
    inner: Event,
}

impl FromVal for DeviceChangeEvent {
    fn from_val(v: &Any) -> Self {
        DeviceChangeEvent {
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

impl core::ops::Deref for DeviceChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeviceChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeviceChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeviceChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DeviceChangeEvent> for Any {
    fn from(s: DeviceChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeviceChangeEvent> for Any {
    fn from(s: &DeviceChangeEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DeviceChangeEvent);

impl DeviceChangeEvent {
    /// Getter of the `devices` attribute.
    /// [`DeviceChangeEvent.devices`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceChangeEvent/devices)
    pub fn devices(&self) -> TypedArray<MediaDeviceInfo> {
        self.inner
            .get("devices")
            .as_::<TypedArray<MediaDeviceInfo>>()
    }
}
impl DeviceChangeEvent {
    /// Getter of the `userInsertedDevices` attribute.
    /// [`DeviceChangeEvent.userInsertedDevices`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceChangeEvent/userInsertedDevices)
    pub fn user_inserted_devices(&self) -> TypedArray<MediaDeviceInfo> {
        self.inner
            .get("userInsertedDevices")
            .as_::<TypedArray<MediaDeviceInfo>>()
    }
}

impl DeviceChangeEvent {
    /// The `new DeviceChangeEvent(..)` constructor, creating a new DeviceChangeEvent instance
    pub fn new0(type_: &JsString) -> DeviceChangeEvent {
        Self {
            inner: Any::global("DeviceChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new DeviceChangeEvent(..)` constructor, creating a new DeviceChangeEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &DeviceChangeEventInit) -> DeviceChangeEvent {
        Self {
            inner: Any::global("DeviceChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
