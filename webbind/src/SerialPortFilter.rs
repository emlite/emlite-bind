use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialPortFilter {
    inner: Any,
}
impl FromVal for SerialPortFilter {
    fn from_val(v: &Any) -> Self {
        SerialPortFilter { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SerialPortFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SerialPortFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SerialPortFilter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SerialPortFilter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SerialPortFilter> for Any {
    fn from(s: SerialPortFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SerialPortFilter> for Any {
    fn from(s: &SerialPortFilter) -> Any {
        s.inner.clone()
    }
}

impl SerialPortFilter {
    pub fn usb_vendor_id(&self) -> u16 {
        self.inner.get("usbVendorId").as_::<u16>()
    }

    pub fn set_usb_vendor_id(&mut self, value: u16) {
        self.inner.set("usbVendorId", value);
    }
}
impl SerialPortFilter {
    pub fn usb_product_id(&self) -> u16 {
        self.inner.get("usbProductId").as_::<u16>()
    }

    pub fn set_usb_product_id(&mut self, value: u16) {
        self.inner.set("usbProductId", value);
    }
}
impl SerialPortFilter {
    pub fn bluetooth_service_class_id(&self) -> Any {
        self.inner.get("bluetoothServiceClassId").as_::<Any>()
    }

    pub fn set_bluetooth_service_class_id(&mut self, value: &Any) {
        self.inner.set("bluetoothServiceClassId", value);
    }
}
