use super::*;




/// The SerialPortInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialPortInfo {
    inner: Any,
}

impl FromVal for SerialPortInfo {
    fn from_val(v: &Any) -> Self {
        SerialPortInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SerialPortInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SerialPortInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SerialPortInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SerialPortInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SerialPortInfo> for Any {
    fn from(s: SerialPortInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SerialPortInfo> for Any {
    fn from(s: &SerialPortInfo) -> Any {
        s.inner.clone()
    }
}

impl SerialPortInfo {
    /// Getter of the `usbVendorId` attribute.
    pub fn usb_vendor_id(&self) -> u16 {
        self.inner.get("usbVendorId").as_::<u16>()
    }

    /// Setter of the `usbVendorId` attribute.
    pub fn set_usb_vendor_id(&mut self, value: u16) {
        self.inner.set("usbVendorId", value);
    }
}
impl SerialPortInfo {
    /// Getter of the `usbProductId` attribute.
    pub fn usb_product_id(&self) -> u16 {
        self.inner.get("usbProductId").as_::<u16>()
    }

    /// Setter of the `usbProductId` attribute.
    pub fn set_usb_product_id(&mut self, value: u16) {
        self.inner.set("usbProductId", value);
    }
}
impl SerialPortInfo {
    /// Getter of the `bluetoothServiceClassId` attribute.
    pub fn bluetooth_service_class_id(&self) -> Any {
        self.inner.get("bluetoothServiceClassId").as_::<Any>()
    }

    /// Setter of the `bluetoothServiceClassId` attribute.
    pub fn set_bluetooth_service_class_id(&mut self, value: &Any) {
        self.inner.set("bluetoothServiceClassId", value);
    }
}
