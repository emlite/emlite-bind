use super::*;




/// The GenerateBidOutput dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GenerateBidOutput {
    inner: Any,
}

impl FromVal for GenerateBidOutput {
    fn from_val(v: &Any) -> Self {
        GenerateBidOutput { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GenerateBidOutput {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GenerateBidOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GenerateBidOutput {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GenerateBidOutput {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GenerateBidOutput> for Any {
    fn from(s: GenerateBidOutput) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GenerateBidOutput> for Any {
    fn from(s: &GenerateBidOutput) -> Any {
        s.inner.clone()
    }
}

impl GenerateBidOutput {
    /// Getter of the `bid` attribute.
    pub fn bid(&self) -> f64 {
        self.inner.get("bid").as_::<f64>()
    }

    /// Setter of the `bid` attribute.
    pub fn set_bid(&mut self, value: f64) {
        self.inner.set("bid", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `bidCurrency` attribute.
    pub fn bid_currency(&self) -> JsString {
        self.inner.get("bidCurrency").as_::<JsString>()
    }

    /// Setter of the `bidCurrency` attribute.
    pub fn set_bid_currency(&mut self, value: &JsString) {
        self.inner.set("bidCurrency", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `render` attribute.
    pub fn render(&self) -> Any {
        self.inner.get("render").as_::<Any>()
    }

    /// Setter of the `render` attribute.
    pub fn set_render(&mut self, value: &Any) {
        self.inner.set("render", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `ad` attribute.
    pub fn ad(&self) -> Any {
        self.inner.get("ad").as_::<Any>()
    }

    /// Setter of the `ad` attribute.
    pub fn set_ad(&mut self, value: &Any) {
        self.inner.set("ad", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `selectedBuyerAndSellerReportingId` attribute.
    pub fn selected_buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner.get("selectedBuyerAndSellerReportingId").as_::<JsString>()
    }

    /// Setter of the `selectedBuyerAndSellerReportingId` attribute.
    pub fn set_selected_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("selectedBuyerAndSellerReportingId", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `adComponents` attribute.
    pub fn ad_components(&self) -> TypedArray<Any> {
        self.inner.get("adComponents").as_::<TypedArray<Any>>()
    }

    /// Setter of the `adComponents` attribute.
    pub fn set_ad_components(&mut self, value: &TypedArray<Any>) {
        self.inner.set("adComponents", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `adCost` attribute.
    pub fn ad_cost(&self) -> f64 {
        self.inner.get("adCost").as_::<f64>()
    }

    /// Setter of the `adCost` attribute.
    pub fn set_ad_cost(&mut self, value: f64) {
        self.inner.set("adCost", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `modelingSignals` attribute.
    pub fn modeling_signals(&self) -> f64 {
        self.inner.get("modelingSignals").as_::<f64>()
    }

    /// Setter of the `modelingSignals` attribute.
    pub fn set_modeling_signals(&mut self, value: f64) {
        self.inner.set("modelingSignals", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `allowComponentAuction` attribute.
    pub fn allow_component_auction(&self) -> bool {
        self.inner.get("allowComponentAuction").as_::<bool>()
    }

    /// Setter of the `allowComponentAuction` attribute.
    pub fn set_allow_component_auction(&mut self, value: bool) {
        self.inner.set("allowComponentAuction", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `targetNumAdComponents` attribute.
    pub fn target_num_ad_components(&self) -> u32 {
        self.inner.get("targetNumAdComponents").as_::<u32>()
    }

    /// Setter of the `targetNumAdComponents` attribute.
    pub fn set_target_num_ad_components(&mut self, value: u32) {
        self.inner.set("targetNumAdComponents", value);
    }
}
impl GenerateBidOutput {
    /// Getter of the `numMandatoryAdComponents` attribute.
    pub fn num_mandatory_ad_components(&self) -> u32 {
        self.inner.get("numMandatoryAdComponents").as_::<u32>()
    }

    /// Setter of the `numMandatoryAdComponents` attribute.
    pub fn set_num_mandatory_ad_components(&mut self, value: u32) {
        self.inner.set("numMandatoryAdComponents", value);
    }
}
