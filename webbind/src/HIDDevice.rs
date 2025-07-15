use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDCollectionInfo {
    inner: emlite::Val,
}
impl FromVal for HIDCollectionInfo {
    fn from_val(v: &emlite::Val) -> Self {
        HIDCollectionInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HIDCollectionInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HIDCollectionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HIDCollectionInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HIDCollectionInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HIDCollectionInfo> for emlite::Val {
    fn from(s: HIDCollectionInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HIDCollectionInfo {
    pub fn usage_page(&self) -> u16 {
        self.inner.get("usagePage").as_::<u16>()
    }

    pub fn set_usage_page(&mut self, value: u16) {
        self.inner.set("usagePage", value);
    }
}
impl HIDCollectionInfo {
    pub fn usage(&self) -> u16 {
        self.inner.get("usage").as_::<u16>()
    }

    pub fn set_usage(&mut self, value: u16) {
        self.inner.set("usage", value);
    }
}
impl HIDCollectionInfo {
    pub fn type_(&self) -> u8 {
        self.inner.get("type").as_::<u8>()
    }

    pub fn set_type_(&mut self, value: u8) {
        self.inner.set("type", value);
    }
}
impl HIDCollectionInfo {
    pub fn children(&self) -> Sequence<HIDCollectionInfo> {
        self.inner
            .get("children")
            .as_::<Sequence<HIDCollectionInfo>>()
    }

    pub fn set_children(&mut self, value: Sequence<HIDCollectionInfo>) {
        self.inner.set("children", value);
    }
}
impl HIDCollectionInfo {
    pub fn input_reports(&self) -> Sequence<Any> {
        self.inner.get("inputReports").as_::<Sequence<Any>>()
    }

    pub fn set_input_reports(&mut self, value: Sequence<Any>) {
        self.inner.set("inputReports", value);
    }
}
impl HIDCollectionInfo {
    pub fn output_reports(&self) -> Sequence<Any> {
        self.inner.get("outputReports").as_::<Sequence<Any>>()
    }

    pub fn set_output_reports(&mut self, value: Sequence<Any>) {
        self.inner.set("outputReports", value);
    }
}
impl HIDCollectionInfo {
    pub fn feature_reports(&self) -> Sequence<Any> {
        self.inner.get("featureReports").as_::<Sequence<Any>>()
    }

    pub fn set_feature_reports(&mut self, value: Sequence<Any>) {
        self.inner.set("featureReports", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDDevice {
    inner: EventTarget,
}
impl FromVal for HIDDevice {
    fn from_val(v: &emlite::Val) -> Self {
        HIDDevice {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for HIDDevice {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HIDDevice {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HIDDevice> for emlite::Val {
    fn from(s: HIDDevice) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HIDDevice);

impl HIDDevice {
    pub fn oninputreport(&self) -> Any {
        self.inner.get("oninputreport").as_::<Any>()
    }

    pub fn set_oninputreport(&mut self, value: Any) {
        self.inner.set("oninputreport", value);
    }
}
impl HIDDevice {
    pub fn opened(&self) -> bool {
        self.inner.get("opened").as_::<bool>()
    }
}
impl HIDDevice {
    pub fn vendor_id(&self) -> u16 {
        self.inner.get("vendorId").as_::<u16>()
    }
}
impl HIDDevice {
    pub fn product_id(&self) -> u16 {
        self.inner.get("productId").as_::<u16>()
    }
}
impl HIDDevice {
    pub fn product_name(&self) -> DOMString {
        self.inner.get("productName").as_::<DOMString>()
    }
}
impl HIDDevice {
    pub fn collections(&self) -> FrozenArray<HIDCollectionInfo> {
        self.inner
            .get("collections")
            .as_::<FrozenArray<HIDCollectionInfo>>()
    }
}
impl HIDDevice {
    pub fn open(&self) -> Promise {
        self.inner.call("open", &[]).as_::<Promise>()
    }
}
impl HIDDevice {
    pub fn close(&self) -> Promise {
        self.inner.call("close", &[]).as_::<Promise>()
    }
}
impl HIDDevice {
    pub fn forget(&self) -> Promise {
        self.inner.call("forget", &[]).as_::<Promise>()
    }
}
impl HIDDevice {
    pub fn send_report(&self, report_id: u8, data: Any) -> Promise {
        self.inner
            .call("sendReport", &[report_id.into(), data.into()])
            .as_::<Promise>()
    }
}
impl HIDDevice {
    pub fn send_feature_report(&self, report_id: u8, data: Any) -> Promise {
        self.inner
            .call("sendFeatureReport", &[report_id.into(), data.into()])
            .as_::<Promise>()
    }
}
impl HIDDevice {
    pub fn receive_feature_report(&self, report_id: u8) -> Promise {
        self.inner
            .call("receiveFeatureReport", &[report_id.into()])
            .as_::<Promise>()
    }
}
