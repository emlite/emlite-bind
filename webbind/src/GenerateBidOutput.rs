use super::*;

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
    pub fn bid(&self) -> f64 {
        self.inner.get("bid").as_::<f64>()
    }

    pub fn set_bid(&mut self, value: f64) {
        self.inner.set("bid", value);
    }
}
impl GenerateBidOutput {
    pub fn bid_currency(&self) -> JsString {
        self.inner.get("bidCurrency").as_::<JsString>()
    }

    pub fn set_bid_currency(&mut self, value: &JsString) {
        self.inner.set("bidCurrency", value);
    }
}
impl GenerateBidOutput {
    pub fn render(&self) -> Any {
        self.inner.get("render").as_::<Any>()
    }

    pub fn set_render(&mut self, value: &Any) {
        self.inner.set("render", value);
    }
}
impl GenerateBidOutput {
    pub fn ad(&self) -> Any {
        self.inner.get("ad").as_::<Any>()
    }

    pub fn set_ad(&mut self, value: &Any) {
        self.inner.set("ad", value);
    }
}
impl GenerateBidOutput {
    pub fn selected_buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner
            .get("selectedBuyerAndSellerReportingId")
            .as_::<JsString>()
    }

    pub fn set_selected_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("selectedBuyerAndSellerReportingId", value);
    }
}
impl GenerateBidOutput {
    pub fn ad_components(&self) -> TypedArray<Any> {
        self.inner.get("adComponents").as_::<TypedArray<Any>>()
    }

    pub fn set_ad_components(&mut self, value: &TypedArray<Any>) {
        self.inner.set("adComponents", value);
    }
}
impl GenerateBidOutput {
    pub fn ad_cost(&self) -> f64 {
        self.inner.get("adCost").as_::<f64>()
    }

    pub fn set_ad_cost(&mut self, value: f64) {
        self.inner.set("adCost", value);
    }
}
impl GenerateBidOutput {
    pub fn modeling_signals(&self) -> f64 {
        self.inner.get("modelingSignals").as_::<f64>()
    }

    pub fn set_modeling_signals(&mut self, value: f64) {
        self.inner.set("modelingSignals", value);
    }
}
impl GenerateBidOutput {
    pub fn allow_component_auction(&self) -> bool {
        self.inner.get("allowComponentAuction").as_::<bool>()
    }

    pub fn set_allow_component_auction(&mut self, value: bool) {
        self.inner.set("allowComponentAuction", value);
    }
}
impl GenerateBidOutput {
    pub fn target_num_ad_components(&self) -> u32 {
        self.inner.get("targetNumAdComponents").as_::<u32>()
    }

    pub fn set_target_num_ad_components(&mut self, value: u32) {
        self.inner.set("targetNumAdComponents", value);
    }
}
impl GenerateBidOutput {
    pub fn num_mandatory_ad_components(&self) -> u32 {
        self.inner.get("numMandatoryAdComponents").as_::<u32>()
    }

    pub fn set_num_mandatory_ad_components(&mut self, value: u32) {
        self.inner.set("numMandatoryAdComponents", value);
    }
}
