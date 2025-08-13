use super::*;




/// The BluetoothLEScanPermissionDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanPermissionDescriptor {
    inner: Any,
}

impl FromVal for BluetoothLEScanPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        BluetoothLEScanPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothLEScanPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothLEScanPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothLEScanPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothLEScanPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BluetoothLEScanPermissionDescriptor> for Any {
    fn from(s: BluetoothLEScanPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothLEScanPermissionDescriptor> for Any {
    fn from(s: &BluetoothLEScanPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl BluetoothLEScanPermissionDescriptor {
    /// Getter of the `filters` attribute.
    pub fn filters(&self) -> TypedArray<BluetoothLEScanFilterInit> {
        self.inner.get("filters").as_::<TypedArray<BluetoothLEScanFilterInit>>()
    }

    /// Setter of the `filters` attribute.
    pub fn set_filters(&mut self, value: &TypedArray<BluetoothLEScanFilterInit>) {
        self.inner.set("filters", value);
    }
}
impl BluetoothLEScanPermissionDescriptor {
    /// Getter of the `keepRepeatedDevices` attribute.
    pub fn keep_repeated_devices(&self) -> bool {
        self.inner.get("keepRepeatedDevices").as_::<bool>()
    }

    /// Setter of the `keepRepeatedDevices` attribute.
    pub fn set_keep_repeated_devices(&mut self, value: bool) {
        self.inner.set("keepRepeatedDevices", value);
    }
}
impl BluetoothLEScanPermissionDescriptor {
    /// Getter of the `acceptAllAdvertisements` attribute.
    pub fn accept_all_advertisements(&self) -> bool {
        self.inner.get("acceptAllAdvertisements").as_::<bool>()
    }

    /// Setter of the `acceptAllAdvertisements` attribute.
    pub fn set_accept_all_advertisements(&mut self, value: bool) {
        self.inner.set("acceptAllAdvertisements", value);
    }
}
