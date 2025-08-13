use super::*;




/// The HIDDevice class.
/// [`HIDDevice`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDDevice {
    inner: EventTarget,
}

impl FromVal for HIDDevice {
    fn from_val(v: &Any) -> Self {
        HIDDevice { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HIDDevice {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HIDDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HIDDevice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HIDDevice {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HIDDevice> for Any {
    fn from(s: HIDDevice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HIDDevice> for Any {
    fn from(s: &HIDDevice) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HIDDevice);


impl HIDDevice {
    /// Getter of the `oninputreport` attribute.
    /// [`HIDDevice.oninputreport`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/oninputreport)
    pub fn oninputreport(&self) -> Any {
        self.inner.get("oninputreport").as_::<Any>()
    }

    /// Setter of the `oninputreport` attribute.
    /// [`HIDDevice.oninputreport`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/oninputreport)
    pub fn set_oninputreport(&mut self, value: &Any) {
        self.inner.set("oninputreport", value);
    }
}
impl HIDDevice {
    /// Getter of the `opened` attribute.
    /// [`HIDDevice.opened`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/opened)
    pub fn opened(&self) -> bool {
        self.inner.get("opened").as_::<bool>()
    }

}
impl HIDDevice {
    /// Getter of the `vendorId` attribute.
    /// [`HIDDevice.vendorId`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/vendorId)
    pub fn vendor_id(&self) -> u16 {
        self.inner.get("vendorId").as_::<u16>()
    }

}
impl HIDDevice {
    /// Getter of the `productId` attribute.
    /// [`HIDDevice.productId`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/productId)
    pub fn product_id(&self) -> u16 {
        self.inner.get("productId").as_::<u16>()
    }

}
impl HIDDevice {
    /// Getter of the `productName` attribute.
    /// [`HIDDevice.productName`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/productName)
    pub fn product_name(&self) -> JsString {
        self.inner.get("productName").as_::<JsString>()
    }

}
impl HIDDevice {
    /// Getter of the `collections` attribute.
    /// [`HIDDevice.collections`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/collections)
    pub fn collections(&self) -> TypedArray<HIDCollectionInfo> {
        self.inner.get("collections").as_::<TypedArray<HIDCollectionInfo>>()
    }

}
impl HIDDevice {
    /// The open method.
    /// [`HIDDevice.open`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/open)
    pub fn open(&self, ) -> Promise<Undefined> {
        self.inner.call("open", &[]).as_::<Promise<Undefined>>()
    }
}
impl HIDDevice {
    /// The close method.
    /// [`HIDDevice.close`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/close)
    pub fn close(&self, ) -> Promise<Undefined> {
        self.inner.call("close", &[]).as_::<Promise<Undefined>>()
    }
}
impl HIDDevice {
    /// The forget method.
    /// [`HIDDevice.forget`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/forget)
    pub fn forget(&self, ) -> Promise<Undefined> {
        self.inner.call("forget", &[]).as_::<Promise<Undefined>>()
    }
}
impl HIDDevice {
    /// The sendReport method.
    /// [`HIDDevice.sendReport`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendReport)
    pub fn send_report(&self, report_id: u8, data: &Any) -> Promise<Undefined> {
        self.inner.call("sendReport", &[report_id.into(), data.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl HIDDevice {
    /// The sendFeatureReport method.
    /// [`HIDDevice.sendFeatureReport`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendFeatureReport)
    pub fn send_feature_report(&self, report_id: u8, data: &Any) -> Promise<Undefined> {
        self.inner.call("sendFeatureReport", &[report_id.into(), data.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl HIDDevice {
    /// The receiveFeatureReport method.
    /// [`HIDDevice.receiveFeatureReport`](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/receiveFeatureReport)
    pub fn receive_feature_report(&self, report_id: u8) -> Promise<DataView> {
        self.inner.call("receiveFeatureReport", &[report_id.into(), ]).as_::<Promise<DataView>>()
    }
}
