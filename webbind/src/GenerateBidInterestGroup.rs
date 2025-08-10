use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GenerateBidInterestGroup {
    inner: Any,
}
impl FromVal for GenerateBidInterestGroup {
    fn from_val(v: &Any) -> Self {
        GenerateBidInterestGroup { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GenerateBidInterestGroup {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GenerateBidInterestGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GenerateBidInterestGroup {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GenerateBidInterestGroup {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GenerateBidInterestGroup> for Any {
    fn from(s: GenerateBidInterestGroup) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GenerateBidInterestGroup> for Any {
    fn from(s: &GenerateBidInterestGroup) -> Any {
        s.inner.clone()
    }
}

impl GenerateBidInterestGroup {
    pub fn owner(&self) -> JsString {
        self.inner.get("owner").as_::<JsString>()
    }

    pub fn set_owner(&mut self, value: &JsString) {
        self.inner.set("owner", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn enable_bidding_signals_prioritization(&self) -> bool {
        self.inner
            .get("enableBiddingSignalsPrioritization")
            .as_::<bool>()
    }

    pub fn set_enable_bidding_signals_prioritization(&mut self, value: bool) {
        self.inner.set("enableBiddingSignalsPrioritization", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn priority_vector(&self) -> Record<JsString, f64> {
        self.inner
            .get("priorityVector")
            .as_::<Record<JsString, f64>>()
    }

    pub fn set_priority_vector(&mut self, value: Record<JsString, f64>) {
        self.inner.set("priorityVector", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn seller_capabilities(&self) -> Record<JsString, TypedArray<JsString>> {
        self.inner
            .get("sellerCapabilities")
            .as_::<Record<JsString, TypedArray<JsString>>>()
    }

    pub fn set_seller_capabilities(&mut self, value: &Record<JsString, TypedArray<JsString>>) {
        self.inner.set("sellerCapabilities", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn execution_mode(&self) -> JsString {
        self.inner.get("executionMode").as_::<JsString>()
    }

    pub fn set_execution_mode(&mut self, value: &JsString) {
        self.inner.set("executionMode", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn bidding_logic_url(&self) -> JsString {
        self.inner.get("biddingLogicURL").as_::<JsString>()
    }

    pub fn set_bidding_logic_url(&mut self, value: &JsString) {
        self.inner.set("biddingLogicURL", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn bidding_wasm_helper_url(&self) -> JsString {
        self.inner.get("biddingWasmHelperURL").as_::<JsString>()
    }

    pub fn set_bidding_wasm_helper_url(&mut self, value: &JsString) {
        self.inner.set("biddingWasmHelperURL", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn update_url(&self) -> JsString {
        self.inner.get("updateURL").as_::<JsString>()
    }

    pub fn set_update_url(&mut self, value: &JsString) {
        self.inner.set("updateURL", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn trusted_bidding_signals_url(&self) -> JsString {
        self.inner.get("trustedBiddingSignalsURL").as_::<JsString>()
    }

    pub fn set_trusted_bidding_signals_url(&mut self, value: &JsString) {
        self.inner.set("trustedBiddingSignalsURL", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn trusted_bidding_signals_keys(&self) -> TypedArray<JsString> {
        self.inner
            .get("trustedBiddingSignalsKeys")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_trusted_bidding_signals_keys(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("trustedBiddingSignalsKeys", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn trusted_bidding_signals_slot_size_mode(&self) -> JsString {
        self.inner
            .get("trustedBiddingSignalsSlotSizeMode")
            .as_::<JsString>()
    }

    pub fn set_trusted_bidding_signals_slot_size_mode(&mut self, value: &JsString) {
        self.inner.set("trustedBiddingSignalsSlotSizeMode", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn max_trusted_bidding_signals_url_length(&self) -> i32 {
        self.inner
            .get("maxTrustedBiddingSignalsURLLength")
            .as_::<i32>()
    }

    pub fn set_max_trusted_bidding_signals_url_length(&mut self, value: i32) {
        self.inner.set("maxTrustedBiddingSignalsURLLength", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn trusted_bidding_signals_coordinator(&self) -> JsString {
        self.inner
            .get("trustedBiddingSignalsCoordinator")
            .as_::<JsString>()
    }

    pub fn set_trusted_bidding_signals_coordinator(&mut self, value: &JsString) {
        self.inner.set("trustedBiddingSignalsCoordinator", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn user_bidding_signals(&self) -> Any {
        self.inner.get("userBiddingSignals").as_::<Any>()
    }

    pub fn set_user_bidding_signals(&mut self, value: &Any) {
        self.inner.set("userBiddingSignals", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn ads(&self) -> TypedArray<AuctionAd> {
        self.inner.get("ads").as_::<TypedArray<AuctionAd>>()
    }

    pub fn set_ads(&mut self, value: &TypedArray<AuctionAd>) {
        self.inner.set("ads", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn ad_components(&self) -> TypedArray<AuctionAd> {
        self.inner
            .get("adComponents")
            .as_::<TypedArray<AuctionAd>>()
    }

    pub fn set_ad_components(&mut self, value: &TypedArray<AuctionAd>) {
        self.inner.set("adComponents", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn ad_sizes(&self) -> Record<JsString, AuctionAdInterestGroupSize> {
        self.inner
            .get("adSizes")
            .as_::<Record<JsString, AuctionAdInterestGroupSize>>()
    }

    pub fn set_ad_sizes(&mut self, value: &Record<JsString, AuctionAdInterestGroupSize>) {
        self.inner.set("adSizes", value);
    }
}
impl GenerateBidInterestGroup {
    pub fn size_groups(&self) -> Record<JsString, TypedArray<JsString>> {
        self.inner
            .get("sizeGroups")
            .as_::<Record<JsString, TypedArray<JsString>>>()
    }

    pub fn set_size_groups(&mut self, value: &Record<JsString, TypedArray<JsString>>) {
        self.inner.set("sizeGroups", value);
    }
}
