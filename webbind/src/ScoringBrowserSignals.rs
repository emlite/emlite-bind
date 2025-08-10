use super::*;

/// The ScoringBrowserSignals dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScoringBrowserSignals {
    inner: Any,
}

impl FromVal for ScoringBrowserSignals {
    fn from_val(v: &Any) -> Self {
        ScoringBrowserSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ScoringBrowserSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ScoringBrowserSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ScoringBrowserSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ScoringBrowserSignals {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ScoringBrowserSignals> for Any {
    fn from(s: ScoringBrowserSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ScoringBrowserSignals> for Any {
    fn from(s: &ScoringBrowserSignals) -> Any {
        s.inner.clone()
    }
}

impl ScoringBrowserSignals {
    /// Getter of the `topWindowHostname` attribute.
    pub fn top_window_hostname(&self) -> JsString {
        self.inner.get("topWindowHostname").as_::<JsString>()
    }

    /// Setter of the `topWindowHostname` attribute.
    pub fn set_top_window_hostname(&mut self, value: &JsString) {
        self.inner.set("topWindowHostname", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `interestGroupOwner` attribute.
    pub fn interest_group_owner(&self) -> JsString {
        self.inner.get("interestGroupOwner").as_::<JsString>()
    }

    /// Setter of the `interestGroupOwner` attribute.
    pub fn set_interest_group_owner(&mut self, value: &JsString) {
        self.inner.set("interestGroupOwner", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `renderURL` attribute.
    pub fn render_url(&self) -> JsString {
        self.inner.get("renderURL").as_::<JsString>()
    }

    /// Setter of the `renderURL` attribute.
    pub fn set_render_url(&mut self, value: &JsString) {
        self.inner.set("renderURL", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `biddingDurationMsec` attribute.
    pub fn bidding_duration_msec(&self) -> u32 {
        self.inner.get("biddingDurationMsec").as_::<u32>()
    }

    /// Setter of the `biddingDurationMsec` attribute.
    pub fn set_bidding_duration_msec(&mut self, value: u32) {
        self.inner.set("biddingDurationMsec", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `bidCurrency` attribute.
    pub fn bid_currency(&self) -> JsString {
        self.inner.get("bidCurrency").as_::<JsString>()
    }

    /// Setter of the `bidCurrency` attribute.
    pub fn set_bid_currency(&mut self, value: &JsString) {
        self.inner.set("bidCurrency", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `renderSize` attribute.
    pub fn render_size(&self) -> Record<JsString, JsString> {
        self.inner
            .get("renderSize")
            .as_::<Record<JsString, JsString>>()
    }

    /// Setter of the `renderSize` attribute.
    pub fn set_render_size(&mut self, value: &Record<JsString, JsString>) {
        self.inner.set("renderSize", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `dataVersion` attribute.
    pub fn data_version(&self) -> u32 {
        self.inner.get("dataVersion").as_::<u32>()
    }

    /// Setter of the `dataVersion` attribute.
    pub fn set_data_version(&mut self, value: u32) {
        self.inner.set("dataVersion", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `crossOriginDataVersion` attribute.
    pub fn cross_origin_data_version(&self) -> u32 {
        self.inner.get("crossOriginDataVersion").as_::<u32>()
    }

    /// Setter of the `crossOriginDataVersion` attribute.
    pub fn set_cross_origin_data_version(&mut self, value: u32) {
        self.inner.set("crossOriginDataVersion", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `adComponents` attribute.
    pub fn ad_components(&self) -> TypedArray<JsString> {
        self.inner.get("adComponents").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `adComponents` attribute.
    pub fn set_ad_components(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("adComponents", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `forDebuggingOnlyInCooldownOrLockout` attribute.
    pub fn for_debugging_only_in_cooldown_or_lockout(&self) -> bool {
        self.inner
            .get("forDebuggingOnlyInCooldownOrLockout")
            .as_::<bool>()
    }

    /// Setter of the `forDebuggingOnlyInCooldownOrLockout` attribute.
    pub fn set_for_debugging_only_in_cooldown_or_lockout(&mut self, value: bool) {
        self.inner.set("forDebuggingOnlyInCooldownOrLockout", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `creativeScanningMetadata` attribute.
    pub fn creative_scanning_metadata(&self) -> JsString {
        self.inner.get("creativeScanningMetadata").as_::<JsString>()
    }

    /// Setter of the `creativeScanningMetadata` attribute.
    pub fn set_creative_scanning_metadata(&mut self, value: &JsString) {
        self.inner.set("creativeScanningMetadata", value);
    }
}
impl ScoringBrowserSignals {
    /// Getter of the `adComponentsCreativeScanningMetadata` attribute.
    pub fn ad_components_creative_scanning_metadata(&self) -> TypedArray<JsString> {
        self.inner
            .get("adComponentsCreativeScanningMetadata")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `adComponentsCreativeScanningMetadata` attribute.
    pub fn set_ad_components_creative_scanning_metadata(&mut self, value: &TypedArray<JsString>) {
        self.inner
            .set("adComponentsCreativeScanningMetadata", value);
    }
}
