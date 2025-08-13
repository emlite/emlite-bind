use super::*;




/// The GenerateBidInterestGroup dictionary.
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
    /// Getter of the `owner` attribute.
    pub fn owner(&self) -> JsString {
        self.inner.get("owner").as_::<JsString>()
    }

    /// Setter of the `owner` attribute.
    pub fn set_owner(&mut self, value: &JsString) {
        self.inner.set("owner", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `enableBiddingSignalsPrioritization` attribute.
    pub fn enable_bidding_signals_prioritization(&self) -> bool {
        self.inner.get("enableBiddingSignalsPrioritization").as_::<bool>()
    }

    /// Setter of the `enableBiddingSignalsPrioritization` attribute.
    pub fn set_enable_bidding_signals_prioritization(&mut self, value: bool) {
        self.inner.set("enableBiddingSignalsPrioritization", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `priorityVector` attribute.
    pub fn priority_vector(&self) -> Record<JsString, f64> {
        self.inner.get("priorityVector").as_::<Record<JsString, f64>>()
    }

    /// Setter of the `priorityVector` attribute.
    pub fn set_priority_vector(&mut self, value: Record<JsString, f64>) {
        self.inner.set("priorityVector", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `sellerCapabilities` attribute.
    pub fn seller_capabilities(&self) -> Record<JsString, TypedArray<JsString>> {
        self.inner.get("sellerCapabilities").as_::<Record<JsString, TypedArray<JsString>>>()
    }

    /// Setter of the `sellerCapabilities` attribute.
    pub fn set_seller_capabilities(&mut self, value: &Record<JsString, TypedArray<JsString>>) {
        self.inner.set("sellerCapabilities", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `executionMode` attribute.
    pub fn execution_mode(&self) -> JsString {
        self.inner.get("executionMode").as_::<JsString>()
    }

    /// Setter of the `executionMode` attribute.
    pub fn set_execution_mode(&mut self, value: &JsString) {
        self.inner.set("executionMode", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `biddingLogicURL` attribute.
    pub fn bidding_logic_url(&self) -> JsString {
        self.inner.get("biddingLogicURL").as_::<JsString>()
    }

    /// Setter of the `biddingLogicURL` attribute.
    pub fn set_bidding_logic_url(&mut self, value: &JsString) {
        self.inner.set("biddingLogicURL", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `biddingWasmHelperURL` attribute.
    pub fn bidding_wasm_helper_url(&self) -> JsString {
        self.inner.get("biddingWasmHelperURL").as_::<JsString>()
    }

    /// Setter of the `biddingWasmHelperURL` attribute.
    pub fn set_bidding_wasm_helper_url(&mut self, value: &JsString) {
        self.inner.set("biddingWasmHelperURL", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `updateURL` attribute.
    pub fn update_url(&self) -> JsString {
        self.inner.get("updateURL").as_::<JsString>()
    }

    /// Setter of the `updateURL` attribute.
    pub fn set_update_url(&mut self, value: &JsString) {
        self.inner.set("updateURL", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `trustedBiddingSignalsURL` attribute.
    pub fn trusted_bidding_signals_url(&self) -> JsString {
        self.inner.get("trustedBiddingSignalsURL").as_::<JsString>()
    }

    /// Setter of the `trustedBiddingSignalsURL` attribute.
    pub fn set_trusted_bidding_signals_url(&mut self, value: &JsString) {
        self.inner.set("trustedBiddingSignalsURL", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `trustedBiddingSignalsKeys` attribute.
    pub fn trusted_bidding_signals_keys(&self) -> TypedArray<JsString> {
        self.inner.get("trustedBiddingSignalsKeys").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `trustedBiddingSignalsKeys` attribute.
    pub fn set_trusted_bidding_signals_keys(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("trustedBiddingSignalsKeys", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `trustedBiddingSignalsSlotSizeMode` attribute.
    pub fn trusted_bidding_signals_slot_size_mode(&self) -> JsString {
        self.inner.get("trustedBiddingSignalsSlotSizeMode").as_::<JsString>()
    }

    /// Setter of the `trustedBiddingSignalsSlotSizeMode` attribute.
    pub fn set_trusted_bidding_signals_slot_size_mode(&mut self, value: &JsString) {
        self.inner.set("trustedBiddingSignalsSlotSizeMode", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `maxTrustedBiddingSignalsURLLength` attribute.
    pub fn max_trusted_bidding_signals_url_length(&self) -> i32 {
        self.inner.get("maxTrustedBiddingSignalsURLLength").as_::<i32>()
    }

    /// Setter of the `maxTrustedBiddingSignalsURLLength` attribute.
    pub fn set_max_trusted_bidding_signals_url_length(&mut self, value: i32) {
        self.inner.set("maxTrustedBiddingSignalsURLLength", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `trustedBiddingSignalsCoordinator` attribute.
    pub fn trusted_bidding_signals_coordinator(&self) -> JsString {
        self.inner.get("trustedBiddingSignalsCoordinator").as_::<JsString>()
    }

    /// Setter of the `trustedBiddingSignalsCoordinator` attribute.
    pub fn set_trusted_bidding_signals_coordinator(&mut self, value: &JsString) {
        self.inner.set("trustedBiddingSignalsCoordinator", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `userBiddingSignals` attribute.
    pub fn user_bidding_signals(&self) -> Any {
        self.inner.get("userBiddingSignals").as_::<Any>()
    }

    /// Setter of the `userBiddingSignals` attribute.
    pub fn set_user_bidding_signals(&mut self, value: &Any) {
        self.inner.set("userBiddingSignals", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `ads` attribute.
    pub fn ads(&self) -> TypedArray<AuctionAd> {
        self.inner.get("ads").as_::<TypedArray<AuctionAd>>()
    }

    /// Setter of the `ads` attribute.
    pub fn set_ads(&mut self, value: &TypedArray<AuctionAd>) {
        self.inner.set("ads", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `adComponents` attribute.
    pub fn ad_components(&self) -> TypedArray<AuctionAd> {
        self.inner.get("adComponents").as_::<TypedArray<AuctionAd>>()
    }

    /// Setter of the `adComponents` attribute.
    pub fn set_ad_components(&mut self, value: &TypedArray<AuctionAd>) {
        self.inner.set("adComponents", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `adSizes` attribute.
    pub fn ad_sizes(&self) -> Record<JsString, AuctionAdInterestGroupSize> {
        self.inner.get("adSizes").as_::<Record<JsString, AuctionAdInterestGroupSize>>()
    }

    /// Setter of the `adSizes` attribute.
    pub fn set_ad_sizes(&mut self, value: &Record<JsString, AuctionAdInterestGroupSize>) {
        self.inner.set("adSizes", value);
    }
}
impl GenerateBidInterestGroup {
    /// Getter of the `sizeGroups` attribute.
    pub fn size_groups(&self) -> Record<JsString, TypedArray<JsString>> {
        self.inner.get("sizeGroups").as_::<Record<JsString, TypedArray<JsString>>>()
    }

    /// Setter of the `sizeGroups` attribute.
    pub fn set_size_groups(&mut self, value: &Record<JsString, TypedArray<JsString>>) {
        self.inner.set("sizeGroups", value);
    }
}
