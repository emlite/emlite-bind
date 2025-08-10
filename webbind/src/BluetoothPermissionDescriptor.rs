use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothPermissionDescriptor {
    inner: Any,
}
impl FromVal for BluetoothPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        BluetoothPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BluetoothPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BluetoothPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BluetoothPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BluetoothPermissionDescriptor> for Any {
    fn from(s: BluetoothPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BluetoothPermissionDescriptor> for Any {
    fn from(s: &BluetoothPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl BluetoothPermissionDescriptor {
    pub fn device_id(&self) -> JsString {
        self.inner.get("deviceId").as_::<JsString>()
    }

    pub fn set_device_id(&mut self, value: &JsString) {
        self.inner.set("deviceId", value);
    }
}
impl BluetoothPermissionDescriptor {
    pub fn filters(&self) -> TypedArray<BluetoothLEScanFilterInit> {
        self.inner
            .get("filters")
            .as_::<TypedArray<BluetoothLEScanFilterInit>>()
    }

    pub fn set_filters(&mut self, value: &TypedArray<BluetoothLEScanFilterInit>) {
        self.inner.set("filters", value);
    }
}
impl BluetoothPermissionDescriptor {
    pub fn optional_services(&self) -> TypedArray<Any> {
        self.inner.get("optionalServices").as_::<TypedArray<Any>>()
    }

    pub fn set_optional_services(&mut self, value: &TypedArray<Any>) {
        self.inner.set("optionalServices", value);
    }
}
impl BluetoothPermissionDescriptor {
    pub fn optional_manufacturer_data(&self) -> TypedArray<u16> {
        self.inner
            .get("optionalManufacturerData")
            .as_::<TypedArray<u16>>()
    }

    pub fn set_optional_manufacturer_data(&mut self, value: TypedArray<u16>) {
        self.inner.set("optionalManufacturerData", value);
    }
}
impl BluetoothPermissionDescriptor {
    pub fn accept_all_devices(&self) -> bool {
        self.inner.get("acceptAllDevices").as_::<bool>()
    }

    pub fn set_accept_all_devices(&mut self, value: bool) {
        self.inner.set("acceptAllDevices", value);
    }
}
