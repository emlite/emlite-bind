use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AllowedBluetoothDevice {
    inner: Any,
}
impl FromVal for AllowedBluetoothDevice {
    fn from_val(v: &Any) -> Self {
        AllowedBluetoothDevice { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AllowedBluetoothDevice {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AllowedBluetoothDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AllowedBluetoothDevice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AllowedBluetoothDevice {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AllowedBluetoothDevice> for Any {
    fn from(s: AllowedBluetoothDevice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AllowedBluetoothDevice> for Any {
    fn from(s: &AllowedBluetoothDevice) -> Any {
        s.inner.clone()
    }
}

impl AllowedBluetoothDevice {
    pub fn device_id(&self) -> JsString {
        self.inner.get("deviceId").as_::<JsString>()
    }

    pub fn set_device_id(&mut self, value: &JsString) {
        self.inner.set("deviceId", value);
    }
}
impl AllowedBluetoothDevice {
    pub fn may_use_gatt(&self) -> bool {
        self.inner.get("mayUseGATT").as_::<bool>()
    }

    pub fn set_may_use_gatt(&mut self, value: bool) {
        self.inner.set("mayUseGATT", value);
    }
}
impl AllowedBluetoothDevice {
    pub fn allowed_services(&self) -> Any {
        self.inner.get("allowedServices").as_::<Any>()
    }

    pub fn set_allowed_services(&mut self, value: &Any) {
        self.inner.set("allowedServices", value);
    }
}
impl AllowedBluetoothDevice {
    pub fn allowed_manufacturer_data(&self) -> TypedArray<u16> {
        self.inner
            .get("allowedManufacturerData")
            .as_::<TypedArray<u16>>()
    }

    pub fn set_allowed_manufacturer_data(&mut self, value: TypedArray<u16>) {
        self.inner.set("allowedManufacturerData", value);
    }
}
