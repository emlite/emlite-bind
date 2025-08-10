use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionAd {
    inner: Any,
}
impl FromVal for AuctionAd {
    fn from_val(v: &Any) -> Self {
        AuctionAd { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuctionAd {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuctionAd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuctionAd {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuctionAd {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuctionAd> for Any {
    fn from(s: AuctionAd) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuctionAd> for Any {
    fn from(s: &AuctionAd) -> Any {
        s.inner.clone()
    }
}

impl AuctionAd {
    pub fn render_url(&self) -> JsString {
        self.inner.get("renderURL").as_::<JsString>()
    }

    pub fn set_render_url(&mut self, value: &JsString) {
        self.inner.set("renderURL", value);
    }
}
impl AuctionAd {
    pub fn size_group(&self) -> JsString {
        self.inner.get("sizeGroup").as_::<JsString>()
    }

    pub fn set_size_group(&mut self, value: &JsString) {
        self.inner.set("sizeGroup", value);
    }
}
impl AuctionAd {
    pub fn metadata(&self) -> Any {
        self.inner.get("metadata").as_::<Any>()
    }

    pub fn set_metadata(&mut self, value: &Any) {
        self.inner.set("metadata", value);
    }
}
impl AuctionAd {
    pub fn buyer_reporting_id(&self) -> JsString {
        self.inner.get("buyerReportingId").as_::<JsString>()
    }

    pub fn set_buyer_reporting_id(&mut self, value: &JsString) {
        self.inner.set("buyerReportingId", value);
    }
}
impl AuctionAd {
    pub fn buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner
            .get("buyerAndSellerReportingId")
            .as_::<JsString>()
    }

    pub fn set_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("buyerAndSellerReportingId", value);
    }
}
impl AuctionAd {
    pub fn selectable_buyer_and_seller_reporting_ids(&self) -> TypedArray<JsString> {
        self.inner
            .get("selectableBuyerAndSellerReportingIds")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_selectable_buyer_and_seller_reporting_ids(&mut self, value: &TypedArray<JsString>) {
        self.inner
            .set("selectableBuyerAndSellerReportingIds", value);
    }
}
impl AuctionAd {
    pub fn allowed_reporting_origins(&self) -> TypedArray<JsString> {
        self.inner
            .get("allowedReportingOrigins")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_allowed_reporting_origins(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("allowedReportingOrigins", value);
    }
}
impl AuctionAd {
    pub fn ad_render_id(&self) -> JsString {
        self.inner.get("adRenderId").as_::<JsString>()
    }

    pub fn set_ad_render_id(&mut self, value: &JsString) {
        self.inner.set("adRenderId", value);
    }
}
impl AuctionAd {
    pub fn creative_scanning_metadata(&self) -> JsString {
        self.inner.get("creativeScanningMetadata").as_::<JsString>()
    }

    pub fn set_creative_scanning_metadata(&mut self, value: &JsString) {
        self.inner.set("creativeScanningMetadata", value);
    }
}
