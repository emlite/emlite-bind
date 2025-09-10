use super::*;

/// The USBPermissionResult class.
/// [`USBPermissionResult`](https://developer.mozilla.org/en-US/docs/Web/API/USBPermissionResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBPermissionResult {
    inner: PermissionStatus,
}

impl FromVal for USBPermissionResult {
    fn from_val(v: &Any) -> Self {
        USBPermissionResult {
            inner: PermissionStatus::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBPermissionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBPermissionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBPermissionResult> for Any {
    fn from(s: USBPermissionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBPermissionResult> for Any {
    fn from(s: &USBPermissionResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBPermissionResult);

impl USBPermissionResult {
    /// Getter of the `devices` attribute.
    /// [`USBPermissionResult.devices`](https://developer.mozilla.org/en-US/docs/Web/API/USBPermissionResult/devices)
    pub fn devices(&self) -> TypedArray<USBDevice> {
        self.inner.get("devices").as_::<TypedArray<USBDevice>>()
    }

    /// Setter of the `devices` attribute.
    /// [`USBPermissionResult.devices`](https://developer.mozilla.org/en-US/docs/Web/API/USBPermissionResult/devices)
    pub fn set_devices(&mut self, value: &TypedArray<USBDevice>) {
        self.inner.set("devices", value);
    }
}
