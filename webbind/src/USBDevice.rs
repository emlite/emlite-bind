use super::*;




/// The USBDevice class.
/// [`USBDevice`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBDevice {
    inner: Any,
}

impl FromVal for USBDevice {
    fn from_val(v: &Any) -> Self {
        USBDevice { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBDevice {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBDevice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBDevice {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<USBDevice> for Any {
    fn from(s: USBDevice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBDevice> for Any {
    fn from(s: &USBDevice) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBDevice);


impl USBDevice {
    /// Getter of the `usbVersionMajor` attribute.
    /// [`USBDevice.usbVersionMajor`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionMajor)
    pub fn usb_version_major(&self) -> u8 {
        self.inner.get("usbVersionMajor").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `usbVersionMinor` attribute.
    /// [`USBDevice.usbVersionMinor`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionMinor)
    pub fn usb_version_minor(&self) -> u8 {
        self.inner.get("usbVersionMinor").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `usbVersionSubminor` attribute.
    /// [`USBDevice.usbVersionSubminor`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionSubminor)
    pub fn usb_version_subminor(&self) -> u8 {
        self.inner.get("usbVersionSubminor").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `deviceClass` attribute.
    /// [`USBDevice.deviceClass`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceClass)
    pub fn device_class(&self) -> u8 {
        self.inner.get("deviceClass").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `deviceSubclass` attribute.
    /// [`USBDevice.deviceSubclass`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceSubclass)
    pub fn device_subclass(&self) -> u8 {
        self.inner.get("deviceSubclass").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `deviceProtocol` attribute.
    /// [`USBDevice.deviceProtocol`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceProtocol)
    pub fn device_protocol(&self) -> u8 {
        self.inner.get("deviceProtocol").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `vendorId` attribute.
    /// [`USBDevice.vendorId`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/vendorId)
    pub fn vendor_id(&self) -> u16 {
        self.inner.get("vendorId").as_::<u16>()
    }

}
impl USBDevice {
    /// Getter of the `productId` attribute.
    /// [`USBDevice.productId`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/productId)
    pub fn product_id(&self) -> u16 {
        self.inner.get("productId").as_::<u16>()
    }

}
impl USBDevice {
    /// Getter of the `deviceVersionMajor` attribute.
    /// [`USBDevice.deviceVersionMajor`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionMajor)
    pub fn device_version_major(&self) -> u8 {
        self.inner.get("deviceVersionMajor").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `deviceVersionMinor` attribute.
    /// [`USBDevice.deviceVersionMinor`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionMinor)
    pub fn device_version_minor(&self) -> u8 {
        self.inner.get("deviceVersionMinor").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `deviceVersionSubminor` attribute.
    /// [`USBDevice.deviceVersionSubminor`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionSubminor)
    pub fn device_version_subminor(&self) -> u8 {
        self.inner.get("deviceVersionSubminor").as_::<u8>()
    }

}
impl USBDevice {
    /// Getter of the `manufacturerName` attribute.
    /// [`USBDevice.manufacturerName`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/manufacturerName)
    pub fn manufacturer_name(&self) -> JsString {
        self.inner.get("manufacturerName").as_::<JsString>()
    }

}
impl USBDevice {
    /// Getter of the `productName` attribute.
    /// [`USBDevice.productName`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/productName)
    pub fn product_name(&self) -> JsString {
        self.inner.get("productName").as_::<JsString>()
    }

}
impl USBDevice {
    /// Getter of the `serialNumber` attribute.
    /// [`USBDevice.serialNumber`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/serialNumber)
    pub fn serial_number(&self) -> JsString {
        self.inner.get("serialNumber").as_::<JsString>()
    }

}
impl USBDevice {
    /// Getter of the `configuration` attribute.
    /// [`USBDevice.configuration`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/configuration)
    pub fn configuration(&self) -> USBConfiguration {
        self.inner.get("configuration").as_::<USBConfiguration>()
    }

}
impl USBDevice {
    /// Getter of the `configurations` attribute.
    /// [`USBDevice.configurations`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/configurations)
    pub fn configurations(&self) -> TypedArray<USBConfiguration> {
        self.inner.get("configurations").as_::<TypedArray<USBConfiguration>>()
    }

}
impl USBDevice {
    /// Getter of the `opened` attribute.
    /// [`USBDevice.opened`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/opened)
    pub fn opened(&self) -> bool {
        self.inner.get("opened").as_::<bool>()
    }

}
impl USBDevice {
    /// The open method.
    /// [`USBDevice.open`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/open)
    pub fn open(&self, ) -> Promise<Undefined> {
        self.inner.call("open", &[]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The close method.
    /// [`USBDevice.close`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/close)
    pub fn close(&self, ) -> Promise<Undefined> {
        self.inner.call("close", &[]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The forget method.
    /// [`USBDevice.forget`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/forget)
    pub fn forget(&self, ) -> Promise<Undefined> {
        self.inner.call("forget", &[]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The selectConfiguration method.
    /// [`USBDevice.selectConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/selectConfiguration)
    pub fn select_configuration(&self, configuration_value: u8) -> Promise<Undefined> {
        self.inner.call("selectConfiguration", &[configuration_value.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The claimInterface method.
    /// [`USBDevice.claimInterface`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/claimInterface)
    pub fn claim_interface(&self, interface_number: u8) -> Promise<Undefined> {
        self.inner.call("claimInterface", &[interface_number.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The releaseInterface method.
    /// [`USBDevice.releaseInterface`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/releaseInterface)
    pub fn release_interface(&self, interface_number: u8) -> Promise<Undefined> {
        self.inner.call("releaseInterface", &[interface_number.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The selectAlternateInterface method.
    /// [`USBDevice.selectAlternateInterface`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/selectAlternateInterface)
    pub fn select_alternate_interface(&self, interface_number: u8, alternate_setting: u8) -> Promise<Undefined> {
        self.inner.call("selectAlternateInterface", &[interface_number.into(), alternate_setting.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The controlTransferIn method.
    /// [`USBDevice.controlTransferIn`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferIn)
    pub fn control_transfer_in(&self, setup: &USBControlTransferParameters, length: u16) -> Promise<USBInTransferResult> {
        self.inner.call("controlTransferIn", &[setup.into(), length.into(), ]).as_::<Promise<USBInTransferResult>>()
    }
}
impl USBDevice {
    /// The controlTransferOut method.
    /// [`USBDevice.controlTransferOut`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)
    pub fn control_transfer_out0(&self, setup: &USBControlTransferParameters) -> Promise<USBOutTransferResult> {
        self.inner.call("controlTransferOut", &[setup.into(), ]).as_::<Promise<USBOutTransferResult>>()
    }
    /// The controlTransferOut method.
    /// [`USBDevice.controlTransferOut`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)
    pub fn control_transfer_out1(&self, setup: &USBControlTransferParameters, data: &Any) -> Promise<USBOutTransferResult> {
        self.inner.call("controlTransferOut", &[setup.into(), data.into(), ]).as_::<Promise<USBOutTransferResult>>()
    }
}
impl USBDevice {
    /// The clearHalt method.
    /// [`USBDevice.clearHalt`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/clearHalt)
    pub fn clear_halt(&self, direction: &USBDirection, endpoint_number: u8) -> Promise<Undefined> {
        self.inner.call("clearHalt", &[direction.into(), endpoint_number.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl USBDevice {
    /// The transferIn method.
    /// [`USBDevice.transferIn`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferIn)
    pub fn transfer_in(&self, endpoint_number: u8, length: u32) -> Promise<USBInTransferResult> {
        self.inner.call("transferIn", &[endpoint_number.into(), length.into(), ]).as_::<Promise<USBInTransferResult>>()
    }
}
impl USBDevice {
    /// The transferOut method.
    /// [`USBDevice.transferOut`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferOut)
    pub fn transfer_out(&self, endpoint_number: u8, data: &Any) -> Promise<USBOutTransferResult> {
        self.inner.call("transferOut", &[endpoint_number.into(), data.into(), ]).as_::<Promise<USBOutTransferResult>>()
    }
}
impl USBDevice {
    /// The isochronousTransferIn method.
    /// [`USBDevice.isochronousTransferIn`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferIn)
    pub fn isochronous_transfer_in(&self, endpoint_number: u8, packet_lengths: TypedArray<u32>) -> Promise<USBIsochronousInTransferResult> {
        self.inner.call("isochronousTransferIn", &[endpoint_number.into(), packet_lengths.into(), ]).as_::<Promise<USBIsochronousInTransferResult>>()
    }
}
impl USBDevice {
    /// The isochronousTransferOut method.
    /// [`USBDevice.isochronousTransferOut`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferOut)
    pub fn isochronous_transfer_out(&self, endpoint_number: u8, data: &Any, packet_lengths: TypedArray<u32>) -> Promise<USBIsochronousOutTransferResult> {
        self.inner.call("isochronousTransferOut", &[endpoint_number.into(), data.into(), packet_lengths.into(), ]).as_::<Promise<USBIsochronousOutTransferResult>>()
    }
}
impl USBDevice {
    /// The reset method.
    /// [`USBDevice.reset`](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/reset)
    pub fn reset(&self, ) -> Promise<Undefined> {
        self.inner.call("reset", &[]).as_::<Promise<Undefined>>()
    }
}
