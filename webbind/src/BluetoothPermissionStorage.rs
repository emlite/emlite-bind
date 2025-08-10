use super::*;

/// The BluetoothPermissionStorage dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothPermissionStorage {
    inner: Any,
}

impl FromVal for BluetoothPermissionStorage {
    fn from_val(v: &Any) -> Self {
        BluetoothPermissionStorage { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothPermissionStorage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothPermissionStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothPermissionStorage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothPermissionStorage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothPermissionStorage> for Any {
    fn from(s: BluetoothPermissionStorage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothPermissionStorage> for Any {
    fn from(s: &BluetoothPermissionStorage) -> Any {
        s.inner.clone()
    }
}

impl BluetoothPermissionStorage {
    /// Getter of the `allowedDevices` attribute.
    pub fn allowed_devices(&self) -> TypedArray<AllowedBluetoothDevice> {
        self.inner
            .get("allowedDevices")
            .as_::<TypedArray<AllowedBluetoothDevice>>()
    }

    /// Setter of the `allowedDevices` attribute.
    pub fn set_allowed_devices(&mut self, value: &TypedArray<AllowedBluetoothDevice>) {
        self.inner.set("allowedDevices", value);
    }
}
