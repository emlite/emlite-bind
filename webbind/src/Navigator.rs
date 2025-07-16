use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeySystemConfiguration {
    inner: Any,
}
impl FromVal for MediaKeySystemConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaKeySystemConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaKeySystemConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeySystemConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaKeySystemConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaKeySystemConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaKeySystemConfiguration> for Any {
    fn from(s: MediaKeySystemConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaKeySystemConfiguration> for Any {
    fn from(s: &MediaKeySystemConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaKeySystemConfiguration {
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }

    pub fn set_label(&mut self, value: &str) {
        self.inner.set("label", value);
    }
}
impl MediaKeySystemConfiguration {
    pub fn init_data_types(&self) -> Sequence<String> {
        self.inner.get("initDataTypes").as_::<Sequence<String>>()
    }

    pub fn set_init_data_types(&mut self, value: &Sequence<String>) {
        self.inner.set("initDataTypes", value);
    }
}
impl MediaKeySystemConfiguration {
    pub fn audio_capabilities(&self) -> Sequence<Any> {
        self.inner.get("audioCapabilities").as_::<Sequence<Any>>()
    }

    pub fn set_audio_capabilities(&mut self, value: &Sequence<Any>) {
        self.inner.set("audioCapabilities", value);
    }
}
impl MediaKeySystemConfiguration {
    pub fn video_capabilities(&self) -> Sequence<Any> {
        self.inner.get("videoCapabilities").as_::<Sequence<Any>>()
    }

    pub fn set_video_capabilities(&mut self, value: &Sequence<Any>) {
        self.inner.set("videoCapabilities", value);
    }
}
impl MediaKeySystemConfiguration {
    pub fn distinctive_identifier(&self) -> MediaKeysRequirement {
        self.inner
            .get("distinctiveIdentifier")
            .as_::<MediaKeysRequirement>()
    }

    pub fn set_distinctive_identifier(&mut self, value: &MediaKeysRequirement) {
        self.inner.set("distinctiveIdentifier", value);
    }
}
impl MediaKeySystemConfiguration {
    pub fn persistent_state(&self) -> MediaKeysRequirement {
        self.inner
            .get("persistentState")
            .as_::<MediaKeysRequirement>()
    }

    pub fn set_persistent_state(&mut self, value: &MediaKeysRequirement) {
        self.inner.set("persistentState", value);
    }
}
impl MediaKeySystemConfiguration {
    pub fn session_types(&self) -> Sequence<String> {
        self.inner.get("sessionTypes").as_::<Sequence<String>>()
    }

    pub fn set_session_types(&mut self, value: &Sequence<String>) {
        self.inner.set("sessionTypes", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RelatedApplication {
    inner: Any,
}
impl FromVal for RelatedApplication {
    fn from_val(v: &Any) -> Self {
        RelatedApplication { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RelatedApplication {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RelatedApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RelatedApplication {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RelatedApplication {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RelatedApplication> for Any {
    fn from(s: RelatedApplication) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RelatedApplication> for Any {
    fn from(s: &RelatedApplication) -> Any {
        s.inner.clone()
    }
}

impl RelatedApplication {
    pub fn platform(&self) -> String {
        self.inner.get("platform").as_::<String>()
    }

    pub fn set_platform(&mut self, value: &str) {
        self.inner.set("platform", value);
    }
}
impl RelatedApplication {
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }

    pub fn set_url(&mut self, value: &str) {
        self.inner.set("url", value);
    }
}
impl RelatedApplication {
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }

    pub fn set_id(&mut self, value: &str) {
        self.inner.set("id", value);
    }
}
impl RelatedApplication {
    pub fn version(&self) -> String {
        self.inner.get("version").as_::<String>()
    }

    pub fn set_version(&mut self, value: &str) {
        self.inner.set("version", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingRecognizerQueryResult {
    inner: Any,
}
impl FromVal for HandwritingRecognizerQueryResult {
    fn from_val(v: &Any) -> Self {
        HandwritingRecognizerQueryResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingRecognizerQueryResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingRecognizerQueryResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingRecognizerQueryResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingRecognizerQueryResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingRecognizerQueryResult> for Any {
    fn from(s: HandwritingRecognizerQueryResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingRecognizerQueryResult> for Any {
    fn from(s: &HandwritingRecognizerQueryResult) -> Any {
        s.inner.clone()
    }
}

impl HandwritingRecognizerQueryResult {
    pub fn text_alternatives(&self) -> bool {
        self.inner.get("textAlternatives").as_::<bool>()
    }

    pub fn set_text_alternatives(&mut self, value: bool) {
        self.inner.set("textAlternatives", value);
    }
}
impl HandwritingRecognizerQueryResult {
    pub fn text_segmentation(&self) -> bool {
        self.inner.get("textSegmentation").as_::<bool>()
    }

    pub fn set_text_segmentation(&mut self, value: bool) {
        self.inner.set("textSegmentation", value);
    }
}
impl HandwritingRecognizerQueryResult {
    pub fn hints(&self) -> Any {
        self.inner.get("hints").as_::<Any>()
    }

    pub fn set_hints(&mut self, value: &Any) {
        self.inner.set("hints", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingModelConstraint {
    inner: Any,
}
impl FromVal for HandwritingModelConstraint {
    fn from_val(v: &Any) -> Self {
        HandwritingModelConstraint { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingModelConstraint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingModelConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingModelConstraint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingModelConstraint {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingModelConstraint> for Any {
    fn from(s: HandwritingModelConstraint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingModelConstraint> for Any {
    fn from(s: &HandwritingModelConstraint) -> Any {
        s.inner.clone()
    }
}

impl HandwritingModelConstraint {
    pub fn languages(&self) -> Sequence<String> {
        self.inner.get("languages").as_::<Sequence<String>>()
    }

    pub fn set_languages(&mut self, value: &Sequence<String>) {
        self.inner.set("languages", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionAdInterestGroup {
    inner: Any,
}
impl FromVal for AuctionAdInterestGroup {
    fn from_val(v: &Any) -> Self {
        AuctionAdInterestGroup { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuctionAdInterestGroup {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuctionAdInterestGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuctionAdInterestGroup {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuctionAdInterestGroup {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuctionAdInterestGroup> for Any {
    fn from(s: AuctionAdInterestGroup) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuctionAdInterestGroup> for Any {
    fn from(s: &AuctionAdInterestGroup) -> Any {
        s.inner.clone()
    }
}

impl AuctionAdInterestGroup {
    pub fn priority(&self) -> f64 {
        self.inner.get("priority").as_::<f64>()
    }

    pub fn set_priority(&mut self, value: f64) {
        self.inner.set("priority", value);
    }
}
impl AuctionAdInterestGroup {
    pub fn priority_signals_overrides(&self) -> Record<String, f64> {
        self.inner
            .get("prioritySignalsOverrides")
            .as_::<Record<String, f64>>()
    }

    pub fn set_priority_signals_overrides(&mut self, value: Record<String, f64>) {
        self.inner.set("prioritySignalsOverrides", value);
    }
}
impl AuctionAdInterestGroup {
    pub fn lifetime_ms(&self) -> f64 {
        self.inner.get("lifetimeMs").as_::<f64>()
    }

    pub fn set_lifetime_ms(&mut self, value: f64) {
        self.inner.set("lifetimeMs", value);
    }
}
impl AuctionAdInterestGroup {
    pub fn additional_bid_key(&self) -> String {
        self.inner.get("additionalBidKey").as_::<String>()
    }

    pub fn set_additional_bid_key(&mut self, value: &str) {
        self.inner.set("additionalBidKey", value);
    }
}
impl AuctionAdInterestGroup {
    pub fn private_aggregation_config(&self) -> Any {
        self.inner.get("privateAggregationConfig").as_::<Any>()
    }

    pub fn set_private_aggregation_config(&mut self, value: &Any) {
        self.inner.set("privateAggregationConfig", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionAdInterestGroupKey {
    inner: Any,
}
impl FromVal for AuctionAdInterestGroupKey {
    fn from_val(v: &Any) -> Self {
        AuctionAdInterestGroupKey { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuctionAdInterestGroupKey {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuctionAdInterestGroupKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuctionAdInterestGroupKey {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuctionAdInterestGroupKey {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuctionAdInterestGroupKey> for Any {
    fn from(s: AuctionAdInterestGroupKey) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuctionAdInterestGroupKey> for Any {
    fn from(s: &AuctionAdInterestGroupKey) -> Any {
        s.inner.clone()
    }
}

impl AuctionAdInterestGroupKey {
    pub fn owner(&self) -> String {
        self.inner.get("owner").as_::<String>()
    }

    pub fn set_owner(&mut self, value: &str) {
        self.inner.set("owner", value);
    }
}
impl AuctionAdInterestGroupKey {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
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
    pub fn seller(&self) -> String {
        self.inner.get("seller").as_::<String>()
    }

    pub fn set_seller(&mut self, value: &str) {
        self.inner.set("seller", value);
    }
}
impl AuctionAdConfig {
    pub fn decision_logic_url(&self) -> String {
        self.inner.get("decisionLogicURL").as_::<String>()
    }

    pub fn set_decision_logic_url(&mut self, value: &str) {
        self.inner.set("decisionLogicURL", value);
    }
}
impl AuctionAdConfig {
    pub fn trusted_scoring_signals_url(&self) -> String {
        self.inner.get("trustedScoringSignalsURL").as_::<String>()
    }

    pub fn set_trusted_scoring_signals_url(&mut self, value: &str) {
        self.inner.set("trustedScoringSignalsURL", value);
    }
}
impl AuctionAdConfig {
    pub fn max_trusted_scoring_signals_url_length(&self) -> i32 {
        self.inner
            .get("maxTrustedScoringSignalsURLLength")
            .as_::<i32>()
    }

    pub fn set_max_trusted_scoring_signals_url_length(&mut self, value: i32) {
        self.inner.set("maxTrustedScoringSignalsURLLength", value);
    }
}
impl AuctionAdConfig {
    pub fn trusted_scoring_signals_coordinator(&self) -> String {
        self.inner
            .get("trustedScoringSignalsCoordinator")
            .as_::<String>()
    }

    pub fn set_trusted_scoring_signals_coordinator(&mut self, value: &str) {
        self.inner.set("trustedScoringSignalsCoordinator", value);
    }
}
impl AuctionAdConfig {
    pub fn send_creative_scanning_metadata(&self) -> bool {
        self.inner.get("sendCreativeScanningMetadata").as_::<bool>()
    }

    pub fn set_send_creative_scanning_metadata(&mut self, value: bool) {
        self.inner.set("sendCreativeScanningMetadata", value);
    }
}
impl AuctionAdConfig {
    pub fn interest_group_buyers(&self) -> Sequence<String> {
        self.inner
            .get("interestGroupBuyers")
            .as_::<Sequence<String>>()
    }

    pub fn set_interest_group_buyers(&mut self, value: &Sequence<String>) {
        self.inner.set("interestGroupBuyers", value);
    }
}
impl AuctionAdConfig {
    pub fn auction_signals(&self) -> Promise {
        self.inner.get("auctionSignals").as_::<Promise>()
    }

    pub fn set_auction_signals(&mut self, value: &Promise) {
        self.inner.set("auctionSignals", value);
    }
}
impl AuctionAdConfig {
    pub fn seller_signals(&self) -> Promise {
        self.inner.get("sellerSignals").as_::<Promise>()
    }

    pub fn set_seller_signals(&mut self, value: &Promise) {
        self.inner.set("sellerSignals", value);
    }
}
impl AuctionAdConfig {
    pub fn direct_from_seller_signals_header_ad_slot(&self) -> Promise {
        self.inner
            .get("directFromSellerSignalsHeaderAdSlot")
            .as_::<Promise>()
    }

    pub fn set_direct_from_seller_signals_header_ad_slot(&mut self, value: &Promise) {
        self.inner.set("directFromSellerSignalsHeaderAdSlot", value);
    }
}
impl AuctionAdConfig {
    pub fn deprecated_render_url_replacements(&self) -> Promise {
        self.inner
            .get("deprecatedRenderURLReplacements")
            .as_::<Promise>()
    }

    pub fn set_deprecated_render_url_replacements(&mut self, value: &Promise) {
        self.inner.set("deprecatedRenderURLReplacements", value);
    }
}
impl AuctionAdConfig {
    pub fn seller_timeout(&self) -> u64 {
        self.inner.get("sellerTimeout").as_::<u64>()
    }

    pub fn set_seller_timeout(&mut self, value: u64) {
        self.inner.set("sellerTimeout", value);
    }
}
impl AuctionAdConfig {
    pub fn seller_experiment_group_id(&self) -> u16 {
        self.inner.get("sellerExperimentGroupId").as_::<u16>()
    }

    pub fn set_seller_experiment_group_id(&mut self, value: u16) {
        self.inner.set("sellerExperimentGroupId", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_signals(&self) -> Promise {
        self.inner.get("perBuyerSignals").as_::<Promise>()
    }

    pub fn set_per_buyer_signals(&mut self, value: &Promise) {
        self.inner.set("perBuyerSignals", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_timeouts(&self) -> Promise {
        self.inner.get("perBuyerTimeouts").as_::<Promise>()
    }

    pub fn set_per_buyer_timeouts(&mut self, value: &Promise) {
        self.inner.set("perBuyerTimeouts", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_cumulative_timeouts(&self) -> Promise {
        self.inner
            .get("perBuyerCumulativeTimeouts")
            .as_::<Promise>()
    }

    pub fn set_per_buyer_cumulative_timeouts(&mut self, value: &Promise) {
        self.inner.set("perBuyerCumulativeTimeouts", value);
    }
}
impl AuctionAdConfig {
    pub fn reporting_timeout(&self) -> u64 {
        self.inner.get("reportingTimeout").as_::<u64>()
    }

    pub fn set_reporting_timeout(&mut self, value: u64) {
        self.inner.set("reportingTimeout", value);
    }
}
impl AuctionAdConfig {
    pub fn seller_currency(&self) -> String {
        self.inner.get("sellerCurrency").as_::<String>()
    }

    pub fn set_seller_currency(&mut self, value: &str) {
        self.inner.set("sellerCurrency", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_currencies(&self) -> Promise {
        self.inner.get("perBuyerCurrencies").as_::<Promise>()
    }

    pub fn set_per_buyer_currencies(&mut self, value: &Promise) {
        self.inner.set("perBuyerCurrencies", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_multi_bid_limits(&self) -> Record<String, u16> {
        self.inner
            .get("perBuyerMultiBidLimits")
            .as_::<Record<String, u16>>()
    }

    pub fn set_per_buyer_multi_bid_limits(&mut self, value: Record<String, u16>) {
        self.inner.set("perBuyerMultiBidLimits", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_group_limits(&self) -> Record<String, u16> {
        self.inner
            .get("perBuyerGroupLimits")
            .as_::<Record<String, u16>>()
    }

    pub fn set_per_buyer_group_limits(&mut self, value: Record<String, u16>) {
        self.inner.set("perBuyerGroupLimits", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_experiment_group_ids(&self) -> Record<String, u16> {
        self.inner
            .get("perBuyerExperimentGroupIds")
            .as_::<Record<String, u16>>()
    }

    pub fn set_per_buyer_experiment_group_ids(&mut self, value: Record<String, u16>) {
        self.inner.set("perBuyerExperimentGroupIds", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_priority_signals(&self) -> Record<String, Record<String, f64>> {
        self.inner
            .get("perBuyerPrioritySignals")
            .as_::<Record<String, Record<String, f64>>>()
    }

    pub fn set_per_buyer_priority_signals(&mut self, value: Record<String, Record<String, f64>>) {
        self.inner.set("perBuyerPrioritySignals", value);
    }
}
impl AuctionAdConfig {
    pub fn auction_report_buyer_keys(&self) -> Sequence<i64> {
        self.inner
            .get("auctionReportBuyerKeys")
            .as_::<Sequence<i64>>()
    }

    pub fn set_auction_report_buyer_keys(&mut self, value: Sequence<i64>) {
        self.inner.set("auctionReportBuyerKeys", value);
    }
}
impl AuctionAdConfig {
    pub fn auction_report_buyers(&self) -> Record<String, Any> {
        self.inner
            .get("auctionReportBuyers")
            .as_::<Record<String, Any>>()
    }

    pub fn set_auction_report_buyers(&mut self, value: &Record<String, Any>) {
        self.inner.set("auctionReportBuyers", value);
    }
}
impl AuctionAdConfig {
    pub fn auction_report_buyer_debug_mode_config(&self) -> Any {
        self.inner
            .get("auctionReportBuyerDebugModeConfig")
            .as_::<Any>()
    }

    pub fn set_auction_report_buyer_debug_mode_config(&mut self, value: &Any) {
        self.inner.set("auctionReportBuyerDebugModeConfig", value);
    }
}
impl AuctionAdConfig {
    pub fn required_seller_capabilities(&self) -> Sequence<String> {
        self.inner
            .get("requiredSellerCapabilities")
            .as_::<Sequence<String>>()
    }

    pub fn set_required_seller_capabilities(&mut self, value: &Sequence<String>) {
        self.inner.set("requiredSellerCapabilities", value);
    }
}
impl AuctionAdConfig {
    pub fn private_aggregation_config(&self) -> Any {
        self.inner.get("privateAggregationConfig").as_::<Any>()
    }

    pub fn set_private_aggregation_config(&mut self, value: &Any) {
        self.inner.set("privateAggregationConfig", value);
    }
}
impl AuctionAdConfig {
    pub fn requested_size(&self) -> Record<String, String> {
        self.inner
            .get("requestedSize")
            .as_::<Record<String, String>>()
    }

    pub fn set_requested_size(&mut self, value: &Record<String, String>) {
        self.inner.set("requestedSize", value);
    }
}
impl AuctionAdConfig {
    pub fn all_slots_requested_sizes(&self) -> Sequence<Record<String, String>> {
        self.inner
            .get("allSlotsRequestedSizes")
            .as_::<Sequence<Record<String, String>>>()
    }

    pub fn set_all_slots_requested_sizes(&mut self, value: &Sequence<Record<String, String>>) {
        self.inner.set("allSlotsRequestedSizes", value);
    }
}
impl AuctionAdConfig {
    pub fn additional_bids(&self) -> Promise {
        self.inner.get("additionalBids").as_::<Promise>()
    }

    pub fn set_additional_bids(&mut self, value: &Promise) {
        self.inner.set("additionalBids", value);
    }
}
impl AuctionAdConfig {
    pub fn auction_nonce(&self) -> String {
        self.inner.get("auctionNonce").as_::<String>()
    }

    pub fn set_auction_nonce(&mut self, value: &str) {
        self.inner.set("auctionNonce", value);
    }
}
impl AuctionAdConfig {
    pub fn seller_real_time_reporting_config(&self) -> Any {
        self.inner.get("sellerRealTimeReportingConfig").as_::<Any>()
    }

    pub fn set_seller_real_time_reporting_config(&mut self, value: &Any) {
        self.inner.set("sellerRealTimeReportingConfig", value);
    }
}
impl AuctionAdConfig {
    pub fn per_buyer_real_time_reporting_config(&self) -> Record<String, Any> {
        self.inner
            .get("perBuyerRealTimeReportingConfig")
            .as_::<Record<String, Any>>()
    }

    pub fn set_per_buyer_real_time_reporting_config(&mut self, value: &Record<String, Any>) {
        self.inner.set("perBuyerRealTimeReportingConfig", value);
    }
}
impl AuctionAdConfig {
    pub fn component_auctions(&self) -> Sequence<AuctionAdConfig> {
        self.inner
            .get("componentAuctions")
            .as_::<Sequence<AuctionAdConfig>>()
    }

    pub fn set_component_auctions(&mut self, value: &Sequence<AuctionAdConfig>) {
        self.inner.set("componentAuctions", value);
    }
}
impl AuctionAdConfig {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl AuctionAdConfig {
    pub fn resolve_to_config(&self) -> Promise {
        self.inner.get("resolveToConfig").as_::<Promise>()
    }

    pub fn set_resolve_to_config(&mut self, value: &Promise) {
        self.inner.set("resolveToConfig", value);
    }
}
impl AuctionAdConfig {
    pub fn server_response(&self) -> Promise {
        self.inner.get("serverResponse").as_::<Promise>()
    }

    pub fn set_server_response(&mut self, value: &Promise) {
        self.inner.set("serverResponse", value);
    }
}
impl AuctionAdConfig {
    pub fn request_id(&self) -> String {
        self.inner.get("requestId").as_::<String>()
    }

    pub fn set_request_id(&mut self, value: &str) {
        self.inner.set("requestId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdAuctionData {
    inner: Any,
}
impl FromVal for AdAuctionData {
    fn from_val(v: &Any) -> Self {
        AdAuctionData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AdAuctionData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AdAuctionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AdAuctionData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AdAuctionData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AdAuctionData> for Any {
    fn from(s: AdAuctionData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AdAuctionData> for Any {
    fn from(s: &AdAuctionData) -> Any {
        s.inner.clone()
    }
}

impl AdAuctionData {
    pub fn request_id(&self) -> String {
        self.inner.get("requestId").as_::<String>()
    }

    pub fn set_request_id(&mut self, value: &str) {
        self.inner.set("requestId", value);
    }
}
impl AdAuctionData {
    pub fn request(&self) -> Uint8Array {
        self.inner.get("request").as_::<Uint8Array>()
    }

    pub fn set_request(&mut self, value: &Uint8Array) {
        self.inner.set("request", value);
    }
}
impl AdAuctionData {
    pub fn requests(&self) -> Sequence<Any> {
        self.inner.get("requests").as_::<Sequence<Any>>()
    }

    pub fn set_requests(&mut self, value: &Sequence<Any>) {
        self.inner.set("requests", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdAuctionDataConfig {
    inner: Any,
}
impl FromVal for AdAuctionDataConfig {
    fn from_val(v: &Any) -> Self {
        AdAuctionDataConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AdAuctionDataConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AdAuctionDataConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AdAuctionDataConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AdAuctionDataConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AdAuctionDataConfig> for Any {
    fn from(s: AdAuctionDataConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AdAuctionDataConfig> for Any {
    fn from(s: &AdAuctionDataConfig) -> Any {
        s.inner.clone()
    }
}

impl AdAuctionDataConfig {
    pub fn seller(&self) -> String {
        self.inner.get("seller").as_::<String>()
    }

    pub fn set_seller(&mut self, value: &str) {
        self.inner.set("seller", value);
    }
}
impl AdAuctionDataConfig {
    pub fn coordinator_origin(&self) -> String {
        self.inner.get("coordinatorOrigin").as_::<String>()
    }

    pub fn set_coordinator_origin(&mut self, value: &str) {
        self.inner.set("coordinatorOrigin", value);
    }
}
impl AdAuctionDataConfig {
    pub fn sellers(&self) -> Sequence<Any> {
        self.inner.get("sellers").as_::<Sequence<Any>>()
    }

    pub fn set_sellers(&mut self, value: &Sequence<Any>) {
        self.inner.set("sellers", value);
    }
}
impl AdAuctionDataConfig {
    pub fn request_size(&self) -> u32 {
        self.inner.get("requestSize").as_::<u32>()
    }

    pub fn set_request_size(&mut self, value: u32) {
        self.inner.set("requestSize", value);
    }
}
impl AdAuctionDataConfig {
    pub fn per_buyer_config(&self) -> Record<String, Any> {
        self.inner
            .get("perBuyerConfig")
            .as_::<Record<String, Any>>()
    }

    pub fn set_per_buyer_config(&mut self, value: &Record<String, Any>) {
        self.inner.set("perBuyerConfig", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShareData {
    inner: Any,
}
impl FromVal for ShareData {
    fn from_val(v: &Any) -> Self {
        ShareData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ShareData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ShareData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ShareData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ShareData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ShareData> for Any {
    fn from(s: ShareData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ShareData> for Any {
    fn from(s: &ShareData) -> Any {
        s.inner.clone()
    }
}

impl ShareData {
    pub fn files(&self) -> Sequence<File> {
        self.inner.get("files").as_::<Sequence<File>>()
    }

    pub fn set_files(&mut self, value: &Sequence<File>) {
        self.inner.set("files", value);
    }
}
impl ShareData {
    pub fn title(&self) -> String {
        self.inner.get("title").as_::<String>()
    }

    pub fn set_title(&mut self, value: &str) {
        self.inner.set("title", value);
    }
}
impl ShareData {
    pub fn text(&self) -> String {
        self.inner.get("text").as_::<String>()
    }

    pub fn set_text(&mut self, value: &str) {
        self.inner.set("text", value);
    }
}
impl ShareData {
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }

    pub fn set_url(&mut self, value: &str) {
        self.inner.set("url", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIOptions {
    inner: Any,
}
impl FromVal for MIDIOptions {
    fn from_val(v: &Any) -> Self {
        MIDIOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MIDIOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MIDIOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MIDIOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MIDIOptions> for Any {
    fn from(s: MIDIOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MIDIOptions> for Any {
    fn from(s: &MIDIOptions) -> Any {
        s.inner.clone()
    }
}

impl MIDIOptions {
    pub fn sysex(&self) -> bool {
        self.inner.get("sysex").as_::<bool>()
    }

    pub fn set_sysex(&mut self, value: bool) {
        self.inner.set("sysex", value);
    }
}
impl MIDIOptions {
    pub fn software(&self) -> bool {
        self.inner.get("software").as_::<bool>()
    }

    pub fn set_software(&mut self, value: bool) {
        self.inner.set("software", value);
    }
}
/// The Navigator class.
/// [`Navigator`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Navigator {
    inner: Any,
}
impl FromVal for Navigator {
    fn from_val(v: &Any) -> Self {
        Navigator {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Navigator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Navigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Navigator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Navigator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Navigator> for Any {
    fn from(s: Navigator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Navigator> for Any {
    fn from(s: &Navigator) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Navigator);

impl Navigator {
    /// Getter of the `audioSession` attribute.
    /// [`Navigator.audioSession`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/audioSession)
    pub fn audio_session(&self) -> AudioSession {
        self.inner.get("audioSession").as_::<AudioSession>()
    }
}
impl Navigator {
    /// The getAutoplayPolicy method.
    /// [`Navigator.getAutoplayPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getAutoplayPolicy)
    pub fn get_autoplay_policy(&self, context: &AudioContext) -> AutoplayPolicy {
        self.inner
            .call("getAutoplayPolicy", &[context.into()])
            .as_::<AutoplayPolicy>()
    }
}
impl Navigator {
    /// The getBattery method.
    /// [`Navigator.getBattery`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getBattery)
    pub fn get_battery(&self) -> Promise {
        self.inner.call("getBattery", &[]).as_::<Promise>()
    }
}
impl Navigator {
    /// The sendBeacon method.
    /// [`Navigator.sendBeacon`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    pub fn send_beacon0(&self, url: &str) -> bool {
        self.inner.call("sendBeacon", &[url.into()]).as_::<bool>()
    }
    /// The sendBeacon method.
    /// [`Navigator.sendBeacon`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    pub fn send_beacon1(&self, url: &str, data: &Any) -> bool {
        self.inner
            .call("sendBeacon", &[url.into(), data.into()])
            .as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `clipboard` attribute.
    /// [`Navigator.clipboard`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clipboard)
    pub fn clipboard(&self) -> Clipboard {
        self.inner.get("clipboard").as_::<Clipboard>()
    }
}
impl Navigator {
    /// Getter of the `contacts` attribute.
    /// [`Navigator.contacts`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/contacts)
    pub fn contacts(&self) -> ContactsManager {
        self.inner.get("contacts").as_::<ContactsManager>()
    }
}
impl Navigator {
    /// Getter of the `credentials` attribute.
    /// [`Navigator.credentials`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/credentials)
    pub fn credentials(&self) -> CredentialsContainer {
        self.inner.get("credentials").as_::<CredentialsContainer>()
    }
}
impl Navigator {
    /// Getter of the `devicePosture` attribute.
    /// [`Navigator.devicePosture`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/devicePosture)
    pub fn device_posture(&self) -> DevicePosture {
        self.inner.get("devicePosture").as_::<DevicePosture>()
    }
}
impl Navigator {
    /// The requestMediaKeySystemAccess method.
    /// [`Navigator.requestMediaKeySystemAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMediaKeySystemAccess)
    pub fn request_media_key_system_access(
        &self,
        key_system: &str,
        supported_configurations: &Sequence<MediaKeySystemConfiguration>,
    ) -> Promise {
        self.inner
            .call(
                "requestMediaKeySystemAccess",
                &[key_system.into(), supported_configurations.into()],
            )
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The deprecatedReplaceInURN method.
    /// [`Navigator.deprecatedReplaceInURN`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedReplaceInURN)
    pub fn deprecated_replace_in_urn(
        &self,
        urn_or_config: &Any,
        replacements: &Record<String, String>,
    ) -> Promise {
        self.inner
            .call(
                "deprecatedReplaceInURN",
                &[urn_or_config.into(), replacements.into()],
            )
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The deprecatedURNtoURL method.
    /// [`Navigator.deprecatedURNtoURL`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedURNtoURL)
    pub fn deprecated_ur_nto_url0(&self, urn_or_config: &Any) -> Promise {
        self.inner
            .call("deprecatedURNtoURL", &[urn_or_config.into()])
            .as_::<Promise>()
    }
    /// The deprecatedURNtoURL method.
    /// [`Navigator.deprecatedURNtoURL`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedURNtoURL)
    pub fn deprecated_ur_nto_url1(&self, urn_or_config: &Any, send_reports: bool) -> Promise {
        self.inner
            .call(
                "deprecatedURNtoURL",
                &[urn_or_config.into(), send_reports.into()],
            )
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The adAuctionComponents method.
    /// [`Navigator.adAuctionComponents`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/adAuctionComponents)
    pub fn ad_auction_components(&self, num_ad_components: u16) -> Sequence<String> {
        self.inner
            .call("adAuctionComponents", &[num_ad_components.into()])
            .as_::<Sequence<String>>()
    }
}
impl Navigator {
    /// The getGamepads method.
    /// [`Navigator.getGamepads`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)
    pub fn get_gamepads(&self) -> Sequence<Gamepad> {
        self.inner
            .call("getGamepads", &[])
            .as_::<Sequence<Gamepad>>()
    }
}
impl Navigator {
    /// Getter of the `geolocation` attribute.
    /// [`Navigator.geolocation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)
    pub fn geolocation(&self) -> Geolocation {
        self.inner.get("geolocation").as_::<Geolocation>()
    }
}
impl Navigator {
    /// The getInstalledRelatedApps method.
    /// [`Navigator.getInstalledRelatedApps`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getInstalledRelatedApps)
    pub fn get_installed_related_apps(&self) -> Promise {
        self.inner
            .call("getInstalledRelatedApps", &[])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The queryHandwritingRecognizer method.
    /// [`Navigator.queryHandwritingRecognizer`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/queryHandwritingRecognizer)
    pub fn query_handwriting_recognizer(&self, constraint: &HandwritingModelConstraint) -> Promise {
        self.inner
            .call("queryHandwritingRecognizer", &[constraint.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The createHandwritingRecognizer method.
    /// [`Navigator.createHandwritingRecognizer`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/createHandwritingRecognizer)
    pub fn create_handwriting_recognizer(
        &self,
        constraint: &HandwritingModelConstraint,
    ) -> Promise {
        self.inner
            .call("createHandwritingRecognizer", &[constraint.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// Getter of the `userActivation` attribute.
    /// [`Navigator.userActivation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userActivation)
    pub fn user_activation(&self) -> UserActivation {
        self.inner.get("userActivation").as_::<UserActivation>()
    }
}
impl Navigator {
    /// Getter of the `ink` attribute.
    /// [`Navigator.ink`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/ink)
    pub fn ink(&self) -> Ink {
        self.inner.get("ink").as_::<Ink>()
    }
}
impl Navigator {
    /// Getter of the `scheduling` attribute.
    /// [`Navigator.scheduling`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/scheduling)
    pub fn scheduling(&self) -> Scheduling {
        self.inner.get("scheduling").as_::<Scheduling>()
    }
}
impl Navigator {
    /// Getter of the `keyboard` attribute.
    /// [`Navigator.keyboard`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/keyboard)
    pub fn keyboard(&self) -> Keyboard {
        self.inner.get("keyboard").as_::<Keyboard>()
    }
}
impl Navigator {
    /// Getter of the `login` attribute.
    /// [`Navigator.login`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/login)
    pub fn login(&self) -> NavigatorLogin {
        self.inner.get("login").as_::<NavigatorLogin>()
    }
}
impl Navigator {
    /// Getter of the `managed` attribute.
    /// [`Navigator.managed`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/managed)
    pub fn managed(&self) -> NavigatorManagedData {
        self.inner.get("managed").as_::<NavigatorManagedData>()
    }
}
impl Navigator {
    /// Getter of the `mediaCapabilities` attribute.
    /// [`Navigator.mediaCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaCapabilities)
    pub fn media_capabilities(&self) -> MediaCapabilities {
        self.inner
            .get("mediaCapabilities")
            .as_::<MediaCapabilities>()
    }
}
impl Navigator {
    /// Getter of the `mediaDevices` attribute.
    /// [`Navigator.mediaDevices`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices)
    pub fn media_devices(&self) -> MediaDevices {
        self.inner.get("mediaDevices").as_::<MediaDevices>()
    }
}
impl Navigator {
    /// Getter of the `preferences` attribute.
    /// [`Navigator.preferences`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/preferences)
    pub fn preferences(&self) -> PreferenceManager {
        self.inner.get("preferences").as_::<PreferenceManager>()
    }
}
impl Navigator {
    /// Getter of the `mediaSession` attribute.
    /// [`Navigator.mediaSession`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaSession)
    pub fn media_session(&self) -> MediaSession {
        self.inner.get("mediaSession").as_::<MediaSession>()
    }
}
impl Navigator {
    /// Getter of the `permissions` attribute.
    /// [`Navigator.permissions`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/permissions)
    pub fn permissions(&self) -> Permissions {
        self.inner.get("permissions").as_::<Permissions>()
    }
}
impl Navigator {
    /// Getter of the `maxTouchPoints` attribute.
    /// [`Navigator.maxTouchPoints`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints)
    pub fn max_touch_points(&self) -> i32 {
        self.inner.get("maxTouchPoints").as_::<i32>()
    }
}
impl Navigator {
    /// Getter of the `presentation` attribute.
    /// [`Navigator.presentation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/presentation)
    pub fn presentation(&self) -> Presentation {
        self.inner.get("presentation").as_::<Presentation>()
    }
}
impl Navigator {
    /// Getter of the `attribution` attribute.
    /// [`Navigator.attribution`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/attribution)
    pub fn attribution(&self) -> Attribution {
        self.inner.get("attribution").as_::<Attribution>()
    }
}
impl Navigator {
    /// Getter of the `wakeLock` attribute.
    /// [`Navigator.wakeLock`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/wakeLock)
    pub fn wake_lock(&self) -> WakeLock {
        self.inner.get("wakeLock").as_::<WakeLock>()
    }
}
impl Navigator {
    /// Getter of the `serial` attribute.
    /// [`Navigator.serial`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serial)
    pub fn serial(&self) -> Serial {
        self.inner.get("serial").as_::<Serial>()
    }
}
impl Navigator {
    /// Getter of the `serviceWorker` attribute.
    /// [`Navigator.serviceWorker`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serviceWorker)
    pub fn service_worker(&self) -> ServiceWorkerContainer {
        self.inner
            .get("serviceWorker")
            .as_::<ServiceWorkerContainer>()
    }
}
impl Navigator {
    /// The joinAdInterestGroup method.
    /// [`Navigator.joinAdInterestGroup`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/joinAdInterestGroup)
    pub fn join_ad_interest_group(&self, group: &AuctionAdInterestGroup) -> Promise {
        self.inner
            .call("joinAdInterestGroup", &[group.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The leaveAdInterestGroup method.
    /// [`Navigator.leaveAdInterestGroup`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/leaveAdInterestGroup)
    pub fn leave_ad_interest_group0(&self) -> Promise {
        self.inner
            .call("leaveAdInterestGroup", &[])
            .as_::<Promise>()
    }
    /// The leaveAdInterestGroup method.
    /// [`Navigator.leaveAdInterestGroup`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/leaveAdInterestGroup)
    pub fn leave_ad_interest_group1(&self, group: &AuctionAdInterestGroupKey) -> Promise {
        self.inner
            .call("leaveAdInterestGroup", &[group.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The clearOriginJoinedAdInterestGroups method.
    /// [`Navigator.clearOriginJoinedAdInterestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clearOriginJoinedAdInterestGroups)
    pub fn clear_origin_joined_ad_interest_groups0(&self, owner: &str) -> Promise {
        self.inner
            .call("clearOriginJoinedAdInterestGroups", &[owner.into()])
            .as_::<Promise>()
    }
    /// The clearOriginJoinedAdInterestGroups method.
    /// [`Navigator.clearOriginJoinedAdInterestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clearOriginJoinedAdInterestGroups)
    pub fn clear_origin_joined_ad_interest_groups1(
        &self,
        owner: &str,
        interest_groups_to_keep: &Sequence<String>,
    ) -> Promise {
        self.inner
            .call(
                "clearOriginJoinedAdInterestGroups",
                &[owner.into(), interest_groups_to_keep.into()],
            )
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The runAdAuction method.
    /// [`Navigator.runAdAuction`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/runAdAuction)
    pub fn run_ad_auction(&self, config: &AuctionAdConfig) -> Promise {
        self.inner
            .call("runAdAuction", &[config.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// Getter of the `deprecatedRunAdAuctionEnforcesKAnonymity` attribute.
    /// [`Navigator.deprecatedRunAdAuctionEnforcesKAnonymity`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedRunAdAuctionEnforcesKAnonymity)
    pub fn deprecated_run_ad_auction_enforces_k_anonymity(&self) -> bool {
        self.inner
            .get("deprecatedRunAdAuctionEnforcesKAnonymity")
            .as_::<bool>()
    }
}
impl Navigator {
    /// The canLoadAdAuctionFencedFrame method.
    /// [`Navigator.canLoadAdAuctionFencedFrame`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canLoadAdAuctionFencedFrame)
    pub fn can_load_ad_auction_fenced_frame(&self) -> bool {
        self.inner
            .call("canLoadAdAuctionFencedFrame", &[])
            .as_::<bool>()
    }
}
impl Navigator {
    /// The getInterestGroupAdAuctionData method.
    /// [`Navigator.getInterestGroupAdAuctionData`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getInterestGroupAdAuctionData)
    pub fn get_interest_group_ad_auction_data0(&self) -> Promise {
        self.inner
            .call("getInterestGroupAdAuctionData", &[])
            .as_::<Promise>()
    }
    /// The getInterestGroupAdAuctionData method.
    /// [`Navigator.getInterestGroupAdAuctionData`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getInterestGroupAdAuctionData)
    pub fn get_interest_group_ad_auction_data1(&self, config: &AdAuctionDataConfig) -> Promise {
        self.inner
            .call("getInterestGroupAdAuctionData", &[config.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The createAuctionNonce method.
    /// [`Navigator.createAuctionNonce`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/createAuctionNonce)
    pub fn create_auction_nonce(&self) -> Promise {
        self.inner.call("createAuctionNonce", &[]).as_::<Promise>()
    }
}
impl Navigator {
    /// The updateAdInterestGroups method.
    /// [`Navigator.updateAdInterestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/updateAdInterestGroups)
    pub fn update_ad_interest_groups(&self) -> Undefined {
        self.inner
            .call("updateAdInterestGroups", &[])
            .as_::<Undefined>()
    }
}
impl Navigator {
    /// Getter of the `protectedAudience` attribute.
    /// [`Navigator.protectedAudience`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/protectedAudience)
    pub fn protected_audience(&self) -> ProtectedAudience {
        self.inner
            .get("protectedAudience")
            .as_::<ProtectedAudience>()
    }
}
impl Navigator {
    /// The vibrate method.
    /// [`Navigator.vibrate`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)
    pub fn vibrate(&self, pattern: &Any) -> bool {
        self.inner.call("vibrate", &[pattern.into()]).as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `virtualKeyboard` attribute.
    /// [`Navigator.virtualKeyboard`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/virtualKeyboard)
    pub fn virtual_keyboard(&self) -> VirtualKeyboard {
        self.inner.get("virtualKeyboard").as_::<VirtualKeyboard>()
    }
}
impl Navigator {
    /// Getter of the `bluetooth` attribute.
    /// [`Navigator.bluetooth`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/bluetooth)
    pub fn bluetooth(&self) -> Bluetooth {
        self.inner.get("bluetooth").as_::<Bluetooth>()
    }
}
impl Navigator {
    /// The share method.
    /// [`Navigator.share`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)
    pub fn share0(&self) -> Promise {
        self.inner.call("share", &[]).as_::<Promise>()
    }
    /// The share method.
    /// [`Navigator.share`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)
    pub fn share1(&self, data: &ShareData) -> Promise {
        self.inner.call("share", &[data.into()]).as_::<Promise>()
    }
}
impl Navigator {
    /// The canShare method.
    /// [`Navigator.canShare`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)
    pub fn can_share0(&self) -> bool {
        self.inner.call("canShare", &[]).as_::<bool>()
    }
    /// The canShare method.
    /// [`Navigator.canShare`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)
    pub fn can_share1(&self, data: &ShareData) -> bool {
        self.inner.call("canShare", &[data.into()]).as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `hid` attribute.
    /// [`Navigator.hid`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hid)
    pub fn hid(&self) -> HID {
        self.inner.get("hid").as_::<HID>()
    }
}
impl Navigator {
    /// The requestMIDIAccess method.
    /// [`Navigator.requestMIDIAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)
    pub fn request_midi_access0(&self) -> Promise {
        self.inner.call("requestMIDIAccess", &[]).as_::<Promise>()
    }
    /// The requestMIDIAccess method.
    /// [`Navigator.requestMIDIAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)
    pub fn request_midi_access1(&self, options: &MIDIOptions) -> Promise {
        self.inner
            .call("requestMIDIAccess", &[options.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// Getter of the `usb` attribute.
    /// [`Navigator.usb`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/usb)
    pub fn usb(&self) -> USB {
        self.inner.get("usb").as_::<USB>()
    }
}
impl Navigator {
    /// Getter of the `xr` attribute.
    /// [`Navigator.xr`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/xr)
    pub fn xr(&self) -> XRSystem {
        self.inner.get("xr").as_::<XRSystem>()
    }
}
impl Navigator {
    /// Getter of the `windowControlsOverlay` attribute.
    /// [`Navigator.windowControlsOverlay`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/windowControlsOverlay)
    pub fn window_controls_overlay(&self) -> WindowControlsOverlay {
        self.inner
            .get("windowControlsOverlay")
            .as_::<WindowControlsOverlay>()
    }
}
impl Navigator {
    /// The setAppBadge method.
    /// [`Navigator.setAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/setAppBadge)
    pub fn set_app_badge0(&self) -> Promise {
        self.inner.call("setAppBadge", &[]).as_::<Promise>()
    }
    /// The setAppBadge method.
    /// [`Navigator.setAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/setAppBadge)
    pub fn set_app_badge1(&self, contents: u64) -> Promise {
        self.inner
            .call("setAppBadge", &[contents.into()])
            .as_::<Promise>()
    }
}
impl Navigator {
    /// The clearAppBadge method.
    /// [`Navigator.clearAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clearAppBadge)
    pub fn clear_app_badge(&self) -> Promise {
        self.inner.call("clearAppBadge", &[]).as_::<Promise>()
    }
}
impl Navigator {
    /// Getter of the `deviceMemory` attribute.
    /// [`Navigator.deviceMemory`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deviceMemory)
    pub fn device_memory(&self) -> f64 {
        self.inner.get("deviceMemory").as_::<f64>()
    }
}
impl Navigator {
    /// Getter of the `globalPrivacyControl` attribute.
    /// [`Navigator.globalPrivacyControl`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/globalPrivacyControl)
    pub fn global_privacy_control(&self) -> bool {
        self.inner.get("globalPrivacyControl").as_::<bool>()
    }
}
impl Navigator {
    /// The taintEnabled method.
    /// [`Navigator.taintEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/taintEnabled)
    pub fn taint_enabled(&self) -> bool {
        self.inner.call("taintEnabled", &[]).as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `oscpu` attribute.
    /// [`Navigator.oscpu`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/oscpu)
    pub fn oscpu(&self) -> String {
        self.inner.get("oscpu").as_::<String>()
    }
}
impl Navigator {
    /// Getter of the `language` attribute.
    /// [`Navigator.language`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)
    pub fn language(&self) -> String {
        self.inner.get("language").as_::<String>()
    }
}
impl Navigator {
    /// Getter of the `languages` attribute.
    /// [`Navigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)
    pub fn languages(&self) -> FrozenArray<String> {
        self.inner.get("languages").as_::<FrozenArray<String>>()
    }
}
impl Navigator {
    /// Getter of the `onLine` attribute.
    /// [`Navigator.onLine`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine)
    pub fn on_line(&self) -> bool {
        self.inner.get("onLine").as_::<bool>()
    }
}
impl Navigator {
    /// The registerProtocolHandler method.
    /// [`Navigator.registerProtocolHandler`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler)
    pub fn register_protocol_handler(&self, scheme: &str, url: &str) -> Undefined {
        self.inner
            .call("registerProtocolHandler", &[scheme.into(), url.into()])
            .as_::<Undefined>()
    }
}
impl Navigator {
    /// The unregisterProtocolHandler method.
    /// [`Navigator.unregisterProtocolHandler`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/unregisterProtocolHandler)
    pub fn unregister_protocol_handler(&self, scheme: &str, url: &str) -> Undefined {
        self.inner
            .call("unregisterProtocolHandler", &[scheme.into(), url.into()])
            .as_::<Undefined>()
    }
}
impl Navigator {
    /// Getter of the `cookieEnabled` attribute.
    /// [`Navigator.cookieEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/cookieEnabled)
    pub fn cookie_enabled(&self) -> bool {
        self.inner.get("cookieEnabled").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `plugins` attribute.
    /// [`Navigator.plugins`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins)
    pub fn plugins(&self) -> PluginArray {
        self.inner.get("plugins").as_::<PluginArray>()
    }
}
impl Navigator {
    /// Getter of the `mimeTypes` attribute.
    /// [`Navigator.mimeTypes`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mimeTypes)
    pub fn mime_types(&self) -> MimeTypeArray {
        self.inner.get("mimeTypes").as_::<MimeTypeArray>()
    }
}
impl Navigator {
    /// The javaEnabled method.
    /// [`Navigator.javaEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/javaEnabled)
    pub fn java_enabled(&self) -> bool {
        self.inner.call("javaEnabled", &[]).as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `pdfViewerEnabled` attribute.
    /// [`Navigator.pdfViewerEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/pdfViewerEnabled)
    pub fn pdf_viewer_enabled(&self) -> bool {
        self.inner.get("pdfViewerEnabled").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `hardwareConcurrency` attribute.
    /// [`Navigator.hardwareConcurrency`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency)
    pub fn hardware_concurrency(&self) -> u64 {
        self.inner.get("hardwareConcurrency").as_::<u64>()
    }
}
impl Navigator {
    /// Getter of the `connection` attribute.
    /// [`Navigator.connection`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/connection)
    pub fn connection(&self) -> NetworkInformation {
        self.inner.get("connection").as_::<NetworkInformation>()
    }
}
impl Navigator {
    /// Getter of the `storageBuckets` attribute.
    /// [`Navigator.storageBuckets`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storageBuckets)
    pub fn storage_buckets(&self) -> StorageBucketManager {
        self.inner
            .get("storageBuckets")
            .as_::<StorageBucketManager>()
    }
}
impl Navigator {
    /// Getter of the `storage` attribute.
    /// [`Navigator.storage`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storage)
    pub fn storage(&self) -> StorageManager {
        self.inner.get("storage").as_::<StorageManager>()
    }
}
impl Navigator {
    /// Getter of the `userAgentData` attribute.
    /// [`Navigator.userAgentData`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgentData)
    pub fn user_agent_data(&self) -> NavigatorUAData {
        self.inner.get("userAgentData").as_::<NavigatorUAData>()
    }
}
impl Navigator {
    /// Getter of the `locks` attribute.
    /// [`Navigator.locks`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/locks)
    pub fn locks(&self) -> LockManager {
        self.inner.get("locks").as_::<LockManager>()
    }
}
impl Navigator {
    /// Getter of the `webdriver` attribute.
    /// [`Navigator.webdriver`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/webdriver)
    pub fn webdriver(&self) -> bool {
        self.inner.get("webdriver").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `gpu` attribute.
    /// [`Navigator.gpu`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu)
    pub fn gpu(&self) -> GPU {
        self.inner.get("gpu").as_::<GPU>()
    }
}
impl Navigator {
    /// Getter of the `ml` attribute.
    /// [`Navigator.ml`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/ml)
    pub fn ml(&self) -> ML {
        self.inner.get("ml").as_::<ML>()
    }
}
