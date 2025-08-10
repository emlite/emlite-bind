use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AllowedUSBDevice {
    inner: Any,
}
impl FromVal for AllowedUSBDevice {
    fn from_val(v: &Any) -> Self {
        AllowedUSBDevice { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AllowedUSBDevice {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AllowedUSBDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AllowedUSBDevice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AllowedUSBDevice {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AllowedUSBDevice> for Any {
    fn from(s: AllowedUSBDevice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AllowedUSBDevice> for Any {
    fn from(s: &AllowedUSBDevice) -> Any {
        s.inner.clone()
    }
}

impl AllowedUSBDevice {
    pub fn vendor_id(&self) -> u8 {
        self.inner.get("vendorId").as_::<u8>()
    }

    pub fn set_vendor_id(&mut self, value: u8) {
        self.inner.set("vendorId", value);
    }
}
impl AllowedUSBDevice {
    pub fn product_id(&self) -> u8 {
        self.inner.get("productId").as_::<u8>()
    }

    pub fn set_product_id(&mut self, value: u8) {
        self.inner.set("productId", value);
    }
}
impl AllowedUSBDevice {
    pub fn serial_number(&self) -> JsString {
        self.inner.get("serialNumber").as_::<JsString>()
    }

    pub fn set_serial_number(&mut self, value: &JsString) {
        self.inner.set("serialNumber", value);
    }
}
