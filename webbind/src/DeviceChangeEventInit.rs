use super::*;

/// The DeviceChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceChangeEventInit {
    inner: Any,
}

impl FromVal for DeviceChangeEventInit {
    fn from_val(v: &Any) -> Self {
        DeviceChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DeviceChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeviceChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeviceChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeviceChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DeviceChangeEventInit> for Any {
    fn from(s: DeviceChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeviceChangeEventInit> for Any {
    fn from(s: &DeviceChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl DeviceChangeEventInit {
    /// Getter of the `devices` attribute.
    pub fn devices(&self) -> TypedArray<MediaDeviceInfo> {
        self.inner
            .get("devices")
            .as_::<TypedArray<MediaDeviceInfo>>()
    }

    /// Setter of the `devices` attribute.
    pub fn set_devices(&mut self, value: &TypedArray<MediaDeviceInfo>) {
        self.inner.set("devices", value);
    }
}
