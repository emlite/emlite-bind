use super::*;

/// The AuctionAd dictionary.
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
    /// Getter of the `renderURL` attribute.
    pub fn render_url(&self) -> JsString {
        self.inner.get("renderURL").as_::<JsString>()
    }

    /// Setter of the `renderURL` attribute.
    pub fn set_render_url(&mut self, value: &JsString) {
        self.inner.set("renderURL", value);
    }
}
impl AuctionAd {
    /// Getter of the `sizeGroup` attribute.
    pub fn size_group(&self) -> JsString {
        self.inner.get("sizeGroup").as_::<JsString>()
    }

    /// Setter of the `sizeGroup` attribute.
    pub fn set_size_group(&mut self, value: &JsString) {
        self.inner.set("sizeGroup", value);
    }
}
impl AuctionAd {
    /// Getter of the `metadata` attribute.
    pub fn metadata(&self) -> Any {
        self.inner.get("metadata").as_::<Any>()
    }

    /// Setter of the `metadata` attribute.
    pub fn set_metadata(&mut self, value: &Any) {
        self.inner.set("metadata", value);
    }
}
impl AuctionAd {
    /// Getter of the `buyerReportingId` attribute.
    pub fn buyer_reporting_id(&self) -> JsString {
        self.inner.get("buyerReportingId").as_::<JsString>()
    }

    /// Setter of the `buyerReportingId` attribute.
    pub fn set_buyer_reporting_id(&mut self, value: &JsString) {
        self.inner.set("buyerReportingId", value);
    }
}
impl AuctionAd {
    /// Getter of the `buyerAndSellerReportingId` attribute.
    pub fn buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner
            .get("buyerAndSellerReportingId")
            .as_::<JsString>()
    }

    /// Setter of the `buyerAndSellerReportingId` attribute.
    pub fn set_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("buyerAndSellerReportingId", value);
    }
}
impl AuctionAd {
    /// Getter of the `selectableBuyerAndSellerReportingIds` attribute.
    pub fn selectable_buyer_and_seller_reporting_ids(&self) -> TypedArray<JsString> {
        self.inner
            .get("selectableBuyerAndSellerReportingIds")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `selectableBuyerAndSellerReportingIds` attribute.
    pub fn set_selectable_buyer_and_seller_reporting_ids(&mut self, value: &TypedArray<JsString>) {
        self.inner
            .set("selectableBuyerAndSellerReportingIds", value);
    }
}
impl AuctionAd {
    /// Getter of the `allowedReportingOrigins` attribute.
    pub fn allowed_reporting_origins(&self) -> TypedArray<JsString> {
        self.inner
            .get("allowedReportingOrigins")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `allowedReportingOrigins` attribute.
    pub fn set_allowed_reporting_origins(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("allowedReportingOrigins", value);
    }
}
impl AuctionAd {
    /// Getter of the `adRenderId` attribute.
    pub fn ad_render_id(&self) -> JsString {
        self.inner.get("adRenderId").as_::<JsString>()
    }

    /// Setter of the `adRenderId` attribute.
    pub fn set_ad_render_id(&mut self, value: &JsString) {
        self.inner.set("adRenderId", value);
    }
}
impl AuctionAd {
    /// Getter of the `creativeScanningMetadata` attribute.
    pub fn creative_scanning_metadata(&self) -> JsString {
        self.inner.get("creativeScanningMetadata").as_::<JsString>()
    }

    /// Setter of the `creativeScanningMetadata` attribute.
    pub fn set_creative_scanning_metadata(&mut self, value: &JsString) {
        self.inner.set("creativeScanningMetadata", value);
    }
}
