use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBControlTransferParameters {
    inner: emlite::Val,
}
impl FromVal for USBControlTransferParameters {
    fn from_val(v: &emlite::Val) -> Self {
        USBControlTransferParameters { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for USBControlTransferParameters {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBControlTransferParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBControlTransferParameters {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBControlTransferParameters {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<USBControlTransferParameters> for emlite::Val {
    fn from(s: USBControlTransferParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBControlTransferParameters {
    pub fn request_type(&self) -> USBRequestType {
        self.inner.get("requestType").as_::<USBRequestType>()
    }

    pub fn set_request_type(&mut self, value: USBRequestType) {
        self.inner.set("requestType", value);
    }

}
impl USBControlTransferParameters {
    pub fn recipient(&self) -> USBRecipient {
        self.inner.get("recipient").as_::<USBRecipient>()
    }

    pub fn set_recipient(&mut self, value: USBRecipient) {
        self.inner.set("recipient", value);
    }

}
impl USBControlTransferParameters {
    pub fn request(&self) -> u8 {
        self.inner.get("request").as_::<u8>()
    }

    pub fn set_request(&mut self, value: u8) {
        self.inner.set("request", value);
    }

}
impl USBControlTransferParameters {
    pub fn value(&self) -> u16 {
        self.inner.get("value").as_::<u16>()
    }

    pub fn set_value(&mut self, value: u16) {
        self.inner.set("value", value);
    }

}
impl USBControlTransferParameters {
    pub fn index(&self) -> u16 {
        self.inner.get("index").as_::<u16>()
    }

    pub fn set_index(&mut self, value: u16) {
        self.inner.set("index", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBDevice {
    inner: emlite::Val,
}
impl FromVal for USBDevice {
    fn from_val(v: &emlite::Val) -> Self {
        USBDevice { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for USBDevice {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBDevice {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBDevice {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<USBDevice> for emlite::Val {
    fn from(s: USBDevice) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USBDevice);


impl USBDevice {
    pub fn usb_version_major(&self) -> u8 {
        self.inner.get("usbVersionMajor").as_::<u8>()
    }

}
impl USBDevice {
    pub fn usb_version_minor(&self) -> u8 {
        self.inner.get("usbVersionMinor").as_::<u8>()
    }

}
impl USBDevice {
    pub fn usb_version_subminor(&self) -> u8 {
        self.inner.get("usbVersionSubminor").as_::<u8>()
    }

}
impl USBDevice {
    pub fn device_class(&self) -> u8 {
        self.inner.get("deviceClass").as_::<u8>()
    }

}
impl USBDevice {
    pub fn device_subclass(&self) -> u8 {
        self.inner.get("deviceSubclass").as_::<u8>()
    }

}
impl USBDevice {
    pub fn device_protocol(&self) -> u8 {
        self.inner.get("deviceProtocol").as_::<u8>()
    }

}
impl USBDevice {
    pub fn vendor_id(&self) -> u16 {
        self.inner.get("vendorId").as_::<u16>()
    }

}
impl USBDevice {
    pub fn product_id(&self) -> u16 {
        self.inner.get("productId").as_::<u16>()
    }

}
impl USBDevice {
    pub fn device_version_major(&self) -> u8 {
        self.inner.get("deviceVersionMajor").as_::<u8>()
    }

}
impl USBDevice {
    pub fn device_version_minor(&self) -> u8 {
        self.inner.get("deviceVersionMinor").as_::<u8>()
    }

}
impl USBDevice {
    pub fn device_version_subminor(&self) -> u8 {
        self.inner.get("deviceVersionSubminor").as_::<u8>()
    }

}
impl USBDevice {
    pub fn manufacturer_name(&self) -> DOMString {
        self.inner.get("manufacturerName").as_::<DOMString>()
    }

}
impl USBDevice {
    pub fn product_name(&self) -> DOMString {
        self.inner.get("productName").as_::<DOMString>()
    }

}
impl USBDevice {
    pub fn serial_number(&self) -> DOMString {
        self.inner.get("serialNumber").as_::<DOMString>()
    }

}
impl USBDevice {
    pub fn configuration(&self) -> USBConfiguration {
        self.inner.get("configuration").as_::<USBConfiguration>()
    }

}
impl USBDevice {
    pub fn configurations(&self) -> FrozenArray<USBConfiguration> {
        self.inner.get("configurations").as_::<FrozenArray<USBConfiguration>>()
    }

}
impl USBDevice {
    pub fn opened(&self) -> bool {
        self.inner.get("opened").as_::<bool>()
    }

}
impl USBDevice {
    pub fn open(&self, ) -> Promise {
        self.inner.call("open", &[]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn close(&self, ) -> Promise {
        self.inner.call("close", &[]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn forget(&self, ) -> Promise {
        self.inner.call("forget", &[]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn select_configuration(&self, configuration_value: u8) -> Promise {
        self.inner.call("selectConfiguration", &[configuration_value.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn claim_interface(&self, interface_number: u8) -> Promise {
        self.inner.call("claimInterface", &[interface_number.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn release_interface(&self, interface_number: u8) -> Promise {
        self.inner.call("releaseInterface", &[interface_number.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn select_alternate_interface(&self, interface_number: u8, alternate_setting: u8) -> Promise {
        self.inner.call("selectAlternateInterface", &[interface_number.into(), alternate_setting.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn control_transfer_in(&self, setup: USBControlTransferParameters, length: u16) -> Promise {
        self.inner.call("controlTransferIn", &[setup.into(), length.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn control_transfer_out0(&self, setup: USBControlTransferParameters) -> Promise {
        self.inner.call("controlTransferOut", &[setup.into(), ]).as_::<Promise>()
    }

    pub fn control_transfer_out1(&self, setup: USBControlTransferParameters, data: Any) -> Promise {
        self.inner.call("controlTransferOut", &[setup.into(), data.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn clear_halt(&self, direction: USBDirection, endpoint_number: u8) -> Promise {
        self.inner.call("clearHalt", &[direction.into(), endpoint_number.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn transfer_in(&self, endpoint_number: u8, length: u32) -> Promise {
        self.inner.call("transferIn", &[endpoint_number.into(), length.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn transfer_out(&self, endpoint_number: u8, data: Any) -> Promise {
        self.inner.call("transferOut", &[endpoint_number.into(), data.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn isochronous_transfer_in(&self, endpoint_number: u8, packet_lengths: Sequence<u32>) -> Promise {
        self.inner.call("isochronousTransferIn", &[endpoint_number.into(), packet_lengths.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn isochronous_transfer_out(&self, endpoint_number: u8, data: Any, packet_lengths: Sequence<u32>) -> Promise {
        self.inner.call("isochronousTransferOut", &[endpoint_number.into(), data.into(), packet_lengths.into(), ]).as_::<Promise>()
    }

}
impl USBDevice {
    pub fn reset(&self, ) -> Promise {
        self.inner.call("reset", &[]).as_::<Promise>()
    }

}
