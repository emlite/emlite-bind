use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBPermissionStorage {
    inner: Any,
}
impl FromVal for USBPermissionStorage {
    fn from_val(v: &Any) -> Self {
        USBPermissionStorage { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for USBPermissionStorage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBPermissionStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for USBPermissionStorage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USBPermissionStorage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USBPermissionStorage> for Any {
    fn from(s: USBPermissionStorage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USBPermissionStorage> for Any {
    fn from(s: &USBPermissionStorage) -> Any {
        s.inner.clone()
    }
}

impl USBPermissionStorage {
    pub fn allowed_devices(&self) -> TypedArray<AllowedUSBDevice> {
        self.inner
            .get("allowedDevices")
            .as_::<TypedArray<AllowedUSBDevice>>()
    }

    pub fn set_allowed_devices(&mut self, value: &TypedArray<AllowedUSBDevice>) {
        self.inner.set("allowedDevices", value);
    }
}
