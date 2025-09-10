use super::*;

/// The BluetoothPermissionResult class.
/// [`BluetoothPermissionResult`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothPermissionResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothPermissionResult {
    inner: PermissionStatus,
}

impl FromVal for BluetoothPermissionResult {
    fn from_val(v: &Any) -> Self {
        BluetoothPermissionResult {
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

impl core::ops::Deref for BluetoothPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothPermissionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothPermissionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothPermissionResult> for Any {
    fn from(s: BluetoothPermissionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothPermissionResult> for Any {
    fn from(s: &BluetoothPermissionResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothPermissionResult);

impl BluetoothPermissionResult {
    /// Getter of the `devices` attribute.
    /// [`BluetoothPermissionResult.devices`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothPermissionResult/devices)
    pub fn devices(&self) -> TypedArray<BluetoothDevice> {
        self.inner
            .get("devices")
            .as_::<TypedArray<BluetoothDevice>>()
    }

    /// Setter of the `devices` attribute.
    /// [`BluetoothPermissionResult.devices`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothPermissionResult/devices)
    pub fn set_devices(&mut self, value: &TypedArray<BluetoothDevice>) {
        self.inner.set("devices", value);
    }
}
