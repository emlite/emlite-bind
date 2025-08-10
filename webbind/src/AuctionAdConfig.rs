use super::*;

/// The AuctionAdConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionAdConfig {
    inner: Any,
}

impl FromVal for AuctionAdConfig {
    fn from_val(v: &Any) -> Self {
        AuctionAdConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuctionAdConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuctionAdConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuctionAdConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuctionAdConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuctionAdConfig> for Any {
    fn from(s: AuctionAdConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuctionAdConfig> for Any {
    fn from(s: &AuctionAdConfig) -> Any {
        s.inner.clone()
    }
}

impl AuctionAdConfig {
    /// Getter of the `seller` attribute.
    pub fn seller(&self) -> JsString {
        self.inner.get("seller").as_::<JsString>()
    }

    /// Setter of the `seller` attribute.
    pub fn set_seller(&mut self, value: &JsString) {
        self.inner.set("seller", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `decisionLogicURL` attribute.
    pub fn decision_logic_url(&self) -> JsString {
        self.inner.get("decisionLogicURL").as_::<JsString>()
    }

    /// Setter of the `decisionLogicURL` attribute.
    pub fn set_decision_logic_url(&mut self, value: &JsString) {
        self.inner.set("decisionLogicURL", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `trustedScoringSignalsURL` attribute.
    pub fn trusted_scoring_signals_url(&self) -> JsString {
        self.inner.get("trustedScoringSignalsURL").as_::<JsString>()
    }

    /// Setter of the `trustedScoringSignalsURL` attribute.
    pub fn set_trusted_scoring_signals_url(&mut self, value: &JsString) {
        self.inner.set("trustedScoringSignalsURL", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `maxTrustedScoringSignalsURLLength` attribute.
    pub fn max_trusted_scoring_signals_url_length(&self) -> i32 {
        self.inner
            .get("maxTrustedScoringSignalsURLLength")
            .as_::<i32>()
    }

    /// Setter of the `maxTrustedScoringSignalsURLLength` attribute.
    pub fn set_max_trusted_scoring_signals_url_length(&mut self, value: i32) {
        self.inner.set("maxTrustedScoringSignalsURLLength", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `trustedScoringSignalsCoordinator` attribute.
    pub fn trusted_scoring_signals_coordinator(&self) -> JsString {
        self.inner
            .get("trustedScoringSignalsCoordinator")
            .as_::<JsString>()
    }

    /// Setter of the `trustedScoringSignalsCoordinator` attribute.
    pub fn set_trusted_scoring_signals_coordinator(&mut self, value: &JsString) {
        self.inner.set("trustedScoringSignalsCoordinator", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `sendCreativeScanningMetadata` attribute.
    pub fn send_creative_scanning_metadata(&self) -> bool {
        self.inner.get("sendCreativeScanningMetadata").as_::<bool>()
    }

    /// Setter of the `sendCreativeScanningMetadata` attribute.
    pub fn set_send_creative_scanning_metadata(&mut self, value: bool) {
        self.inner.set("sendCreativeScanningMetadata", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `interestGroupBuyers` attribute.
    pub fn interest_group_buyers(&self) -> TypedArray<JsString> {
        self.inner
            .get("interestGroupBuyers")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `interestGroupBuyers` attribute.
    pub fn set_interest_group_buyers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("interestGroupBuyers", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `auctionSignals` attribute.
    pub fn auction_signals(&self) -> Promise<Any> {
        self.inner.get("auctionSignals").as_::<Promise<Any>>()
    }

    /// Setter of the `auctionSignals` attribute.
    pub fn set_auction_signals(&mut self, value: &Promise<Any>) {
        self.inner.set("auctionSignals", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `sellerSignals` attribute.
    pub fn seller_signals(&self) -> Promise<Any> {
        self.inner.get("sellerSignals").as_::<Promise<Any>>()
    }

    /// Setter of the `sellerSignals` attribute.
    pub fn set_seller_signals(&mut self, value: &Promise<Any>) {
        self.inner.set("sellerSignals", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `directFromSellerSignalsHeaderAdSlot` attribute.
    pub fn direct_from_seller_signals_header_ad_slot(&self) -> Promise<JsString> {
        self.inner
            .get("directFromSellerSignalsHeaderAdSlot")
            .as_::<Promise<JsString>>()
    }

    /// Setter of the `directFromSellerSignalsHeaderAdSlot` attribute.
    pub fn set_direct_from_seller_signals_header_ad_slot(&mut self, value: &Promise<JsString>) {
        self.inner.set("directFromSellerSignalsHeaderAdSlot", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `deprecatedRenderURLReplacements` attribute.
    pub fn deprecated_render_url_replacements(&self) -> Promise<Record<JsString, JsString>> {
        self.inner
            .get("deprecatedRenderURLReplacements")
            .as_::<Promise<Record<JsString, JsString>>>()
    }

    /// Setter of the `deprecatedRenderURLReplacements` attribute.
    pub fn set_deprecated_render_url_replacements(
        &mut self,
        value: &Promise<Record<JsString, JsString>>,
    ) {
        self.inner.set("deprecatedRenderURLReplacements", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `sellerTimeout` attribute.
    pub fn seller_timeout(&self) -> u64 {
        self.inner.get("sellerTimeout").as_::<u64>()
    }

    /// Setter of the `sellerTimeout` attribute.
    pub fn set_seller_timeout(&mut self, value: u64) {
        self.inner.set("sellerTimeout", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `sellerExperimentGroupId` attribute.
    pub fn seller_experiment_group_id(&self) -> u16 {
        self.inner.get("sellerExperimentGroupId").as_::<u16>()
    }

    /// Setter of the `sellerExperimentGroupId` attribute.
    pub fn set_seller_experiment_group_id(&mut self, value: u16) {
        self.inner.set("sellerExperimentGroupId", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerSignals` attribute.
    pub fn per_buyer_signals(&self) -> Promise<Record<JsString, Any>> {
        self.inner
            .get("perBuyerSignals")
            .as_::<Promise<Record<JsString, Any>>>()
    }

    /// Setter of the `perBuyerSignals` attribute.
    pub fn set_per_buyer_signals(&mut self, value: &Promise<Record<JsString, Any>>) {
        self.inner.set("perBuyerSignals", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerTimeouts` attribute.
    pub fn per_buyer_timeouts(&self) -> Promise<Record<JsString, u64>> {
        self.inner
            .get("perBuyerTimeouts")
            .as_::<Promise<Record<JsString, u64>>>()
    }

    /// Setter of the `perBuyerTimeouts` attribute.
    pub fn set_per_buyer_timeouts(&mut self, value: Promise<Record<JsString, u64>>) {
        self.inner.set("perBuyerTimeouts", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerCumulativeTimeouts` attribute.
    pub fn per_buyer_cumulative_timeouts(&self) -> Promise<Record<JsString, u64>> {
        self.inner
            .get("perBuyerCumulativeTimeouts")
            .as_::<Promise<Record<JsString, u64>>>()
    }

    /// Setter of the `perBuyerCumulativeTimeouts` attribute.
    pub fn set_per_buyer_cumulative_timeouts(&mut self, value: Promise<Record<JsString, u64>>) {
        self.inner.set("perBuyerCumulativeTimeouts", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `reportingTimeout` attribute.
    pub fn reporting_timeout(&self) -> u64 {
        self.inner.get("reportingTimeout").as_::<u64>()
    }

    /// Setter of the `reportingTimeout` attribute.
    pub fn set_reporting_timeout(&mut self, value: u64) {
        self.inner.set("reportingTimeout", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `sellerCurrency` attribute.
    pub fn seller_currency(&self) -> JsString {
        self.inner.get("sellerCurrency").as_::<JsString>()
    }

    /// Setter of the `sellerCurrency` attribute.
    pub fn set_seller_currency(&mut self, value: &JsString) {
        self.inner.set("sellerCurrency", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerCurrencies` attribute.
    pub fn per_buyer_currencies(&self) -> Promise<Record<JsString, JsString>> {
        self.inner
            .get("perBuyerCurrencies")
            .as_::<Promise<Record<JsString, JsString>>>()
    }

    /// Setter of the `perBuyerCurrencies` attribute.
    pub fn set_per_buyer_currencies(&mut self, value: &Promise<Record<JsString, JsString>>) {
        self.inner.set("perBuyerCurrencies", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerMultiBidLimits` attribute.
    pub fn per_buyer_multi_bid_limits(&self) -> Record<JsString, u16> {
        self.inner
            .get("perBuyerMultiBidLimits")
            .as_::<Record<JsString, u16>>()
    }

    /// Setter of the `perBuyerMultiBidLimits` attribute.
    pub fn set_per_buyer_multi_bid_limits(&mut self, value: Record<JsString, u16>) {
        self.inner.set("perBuyerMultiBidLimits", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerGroupLimits` attribute.
    pub fn per_buyer_group_limits(&self) -> Record<JsString, u16> {
        self.inner
            .get("perBuyerGroupLimits")
            .as_::<Record<JsString, u16>>()
    }

    /// Setter of the `perBuyerGroupLimits` attribute.
    pub fn set_per_buyer_group_limits(&mut self, value: Record<JsString, u16>) {
        self.inner.set("perBuyerGroupLimits", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerExperimentGroupIds` attribute.
    pub fn per_buyer_experiment_group_ids(&self) -> Record<JsString, u16> {
        self.inner
            .get("perBuyerExperimentGroupIds")
            .as_::<Record<JsString, u16>>()
    }

    /// Setter of the `perBuyerExperimentGroupIds` attribute.
    pub fn set_per_buyer_experiment_group_ids(&mut self, value: Record<JsString, u16>) {
        self.inner.set("perBuyerExperimentGroupIds", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerPrioritySignals` attribute.
    pub fn per_buyer_priority_signals(&self) -> Record<JsString, Record<JsString, f64>> {
        self.inner
            .get("perBuyerPrioritySignals")
            .as_::<Record<JsString, Record<JsString, f64>>>()
    }

    /// Setter of the `perBuyerPrioritySignals` attribute.
    pub fn set_per_buyer_priority_signals(
        &mut self,
        value: Record<JsString, Record<JsString, f64>>,
    ) {
        self.inner.set("perBuyerPrioritySignals", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `auctionReportBuyerKeys` attribute.
    pub fn auction_report_buyer_keys(&self) -> TypedArray<i64> {
        self.inner
            .get("auctionReportBuyerKeys")
            .as_::<TypedArray<i64>>()
    }

    /// Setter of the `auctionReportBuyerKeys` attribute.
    pub fn set_auction_report_buyer_keys(&mut self, value: TypedArray<i64>) {
        self.inner.set("auctionReportBuyerKeys", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `auctionReportBuyers` attribute.
    pub fn auction_report_buyers(&self) -> Record<JsString, AuctionReportBuyersConfig> {
        self.inner
            .get("auctionReportBuyers")
            .as_::<Record<JsString, AuctionReportBuyersConfig>>()
    }

    /// Setter of the `auctionReportBuyers` attribute.
    pub fn set_auction_report_buyers(
        &mut self,
        value: &Record<JsString, AuctionReportBuyersConfig>,
    ) {
        self.inner.set("auctionReportBuyers", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `auctionReportBuyerDebugModeConfig` attribute.
    pub fn auction_report_buyer_debug_mode_config(&self) -> AuctionReportBuyerDebugModeConfig {
        self.inner
            .get("auctionReportBuyerDebugModeConfig")
            .as_::<AuctionReportBuyerDebugModeConfig>()
    }

    /// Setter of the `auctionReportBuyerDebugModeConfig` attribute.
    pub fn set_auction_report_buyer_debug_mode_config(
        &mut self,
        value: &AuctionReportBuyerDebugModeConfig,
    ) {
        self.inner.set("auctionReportBuyerDebugModeConfig", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `requiredSellerCapabilities` attribute.
    pub fn required_seller_capabilities(&self) -> TypedArray<JsString> {
        self.inner
            .get("requiredSellerCapabilities")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `requiredSellerCapabilities` attribute.
    pub fn set_required_seller_capabilities(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("requiredSellerCapabilities", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `privateAggregationConfig` attribute.
    pub fn private_aggregation_config(&self) -> ProtectedAudiencePrivateAggregationConfig {
        self.inner
            .get("privateAggregationConfig")
            .as_::<ProtectedAudiencePrivateAggregationConfig>()
    }

    /// Setter of the `privateAggregationConfig` attribute.
    pub fn set_private_aggregation_config(
        &mut self,
        value: &ProtectedAudiencePrivateAggregationConfig,
    ) {
        self.inner.set("privateAggregationConfig", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `requestedSize` attribute.
    pub fn requested_size(&self) -> Record<JsString, JsString> {
        self.inner
            .get("requestedSize")
            .as_::<Record<JsString, JsString>>()
    }

    /// Setter of the `requestedSize` attribute.
    pub fn set_requested_size(&mut self, value: &Record<JsString, JsString>) {
        self.inner.set("requestedSize", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `allSlotsRequestedSizes` attribute.
    pub fn all_slots_requested_sizes(&self) -> TypedArray<Record<JsString, JsString>> {
        self.inner
            .get("allSlotsRequestedSizes")
            .as_::<TypedArray<Record<JsString, JsString>>>()
    }

    /// Setter of the `allSlotsRequestedSizes` attribute.
    pub fn set_all_slots_requested_sizes(
        &mut self,
        value: &TypedArray<Record<JsString, JsString>>,
    ) {
        self.inner.set("allSlotsRequestedSizes", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `additionalBids` attribute.
    pub fn additional_bids(&self) -> Promise<Undefined> {
        self.inner.get("additionalBids").as_::<Promise<Undefined>>()
    }

    /// Setter of the `additionalBids` attribute.
    pub fn set_additional_bids(&mut self, value: &Promise<Undefined>) {
        self.inner.set("additionalBids", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `auctionNonce` attribute.
    pub fn auction_nonce(&self) -> JsString {
        self.inner.get("auctionNonce").as_::<JsString>()
    }

    /// Setter of the `auctionNonce` attribute.
    pub fn set_auction_nonce(&mut self, value: &JsString) {
        self.inner.set("auctionNonce", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `sellerRealTimeReportingConfig` attribute.
    pub fn seller_real_time_reporting_config(&self) -> AuctionRealTimeReportingConfig {
        self.inner
            .get("sellerRealTimeReportingConfig")
            .as_::<AuctionRealTimeReportingConfig>()
    }

    /// Setter of the `sellerRealTimeReportingConfig` attribute.
    pub fn set_seller_real_time_reporting_config(
        &mut self,
        value: &AuctionRealTimeReportingConfig,
    ) {
        self.inner.set("sellerRealTimeReportingConfig", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `perBuyerRealTimeReportingConfig` attribute.
    pub fn per_buyer_real_time_reporting_config(
        &self,
    ) -> Record<JsString, AuctionRealTimeReportingConfig> {
        self.inner
            .get("perBuyerRealTimeReportingConfig")
            .as_::<Record<JsString, AuctionRealTimeReportingConfig>>()
    }

    /// Setter of the `perBuyerRealTimeReportingConfig` attribute.
    pub fn set_per_buyer_real_time_reporting_config(
        &mut self,
        value: &Record<JsString, AuctionRealTimeReportingConfig>,
    ) {
        self.inner.set("perBuyerRealTimeReportingConfig", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `componentAuctions` attribute.
    pub fn component_auctions(&self) -> TypedArray<AuctionAdConfig> {
        self.inner
            .get("componentAuctions")
            .as_::<TypedArray<AuctionAdConfig>>()
    }

    /// Setter of the `componentAuctions` attribute.
    pub fn set_component_auctions(&mut self, value: &TypedArray<AuctionAdConfig>) {
        self.inner.set("componentAuctions", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `resolveToConfig` attribute.
    pub fn resolve_to_config(&self) -> Promise<bool> {
        self.inner.get("resolveToConfig").as_::<Promise<bool>>()
    }

    /// Setter of the `resolveToConfig` attribute.
    pub fn set_resolve_to_config(&mut self, value: Promise<bool>) {
        self.inner.set("resolveToConfig", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `serverResponse` attribute.
    pub fn server_response(&self) -> Promise<Uint8Array> {
        self.inner
            .get("serverResponse")
            .as_::<Promise<Uint8Array>>()
    }

    /// Setter of the `serverResponse` attribute.
    pub fn set_server_response(&mut self, value: &Promise<Uint8Array>) {
        self.inner.set("serverResponse", value);
    }
}
impl AuctionAdConfig {
    /// Getter of the `requestId` attribute.
    pub fn request_id(&self) -> JsString {
        self.inner.get("requestId").as_::<JsString>()
    }

    /// Setter of the `requestId` attribute.
    pub fn set_request_id(&mut self, value: &JsString) {
        self.inner.set("requestId", value);
    }
}
