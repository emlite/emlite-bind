use super::*;

/// The USBDeviceFilter dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBDeviceFilter {
    inner: Any,
}

impl FromVal for USBDeviceFilter {
    fn from_val(v: &Any) -> Self {
        USBDeviceFilter { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBDeviceFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBDeviceFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBDeviceFilter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBDeviceFilter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBDeviceFilter> for Any {
    fn from(s: USBDeviceFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBDeviceFilter> for Any {
    fn from(s: &USBDeviceFilter) -> Any {
        s.inner.clone()
    }
}

impl USBDeviceFilter {
    /// Getter of the `vendorId` attribute.
    pub fn vendor_id(&self) -> u16 {
        self.inner.get("vendorId").as_::<u16>()
    }

    /// Setter of the `vendorId` attribute.
    pub fn set_vendor_id(&mut self, value: u16) {
        self.inner.set("vendorId", value);
    }
}
impl USBDeviceFilter {
    /// Getter of the `productId` attribute.
    pub fn product_id(&self) -> u16 {
        self.inner.get("productId").as_::<u16>()
    }

    /// Setter of the `productId` attribute.
    pub fn set_product_id(&mut self, value: u16) {
        self.inner.set("productId", value);
    }
}
impl USBDeviceFilter {
    /// Getter of the `classCode` attribute.
    pub fn class_code(&self) -> u8 {
        self.inner.get("classCode").as_::<u8>()
    }

    /// Setter of the `classCode` attribute.
    pub fn set_class_code(&mut self, value: u8) {
        self.inner.set("classCode", value);
    }
}
impl USBDeviceFilter {
    /// Getter of the `subclassCode` attribute.
    pub fn subclass_code(&self) -> u8 {
        self.inner.get("subclassCode").as_::<u8>()
    }

    /// Setter of the `subclassCode` attribute.
    pub fn set_subclass_code(&mut self, value: u8) {
        self.inner.set("subclassCode", value);
    }
}
impl USBDeviceFilter {
    /// Getter of the `protocolCode` attribute.
    pub fn protocol_code(&self) -> u8 {
        self.inner.get("protocolCode").as_::<u8>()
    }

    /// Setter of the `protocolCode` attribute.
    pub fn set_protocol_code(&mut self, value: u8) {
        self.inner.set("protocolCode", value);
    }
}
impl USBDeviceFilter {
    /// Getter of the `serialNumber` attribute.
    pub fn serial_number(&self) -> JsString {
        self.inner.get("serialNumber").as_::<JsString>()
    }

    /// Setter of the `serialNumber` attribute.
    pub fn set_serial_number(&mut self, value: &JsString) {
        self.inner.set("serialNumber", value);
    }
}
