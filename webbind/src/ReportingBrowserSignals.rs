use super::*;




/// The ReportingBrowserSignals dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReportingBrowserSignals {
    inner: Any,
}

impl FromVal for ReportingBrowserSignals {
    fn from_val(v: &Any) -> Self {
        ReportingBrowserSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReportingBrowserSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReportingBrowserSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReportingBrowserSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReportingBrowserSignals {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ReportingBrowserSignals> for Any {
    fn from(s: ReportingBrowserSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReportingBrowserSignals> for Any {
    fn from(s: &ReportingBrowserSignals) -> Any {
        s.inner.clone()
    }
}

impl ReportingBrowserSignals {
    /// Getter of the `topWindowHostname` attribute.
    pub fn top_window_hostname(&self) -> JsString {
        self.inner.get("topWindowHostname").as_::<JsString>()
    }

    /// Setter of the `topWindowHostname` attribute.
    pub fn set_top_window_hostname(&mut self, value: &JsString) {
        self.inner.set("topWindowHostname", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `interestGroupOwner` attribute.
    pub fn interest_group_owner(&self) -> JsString {
        self.inner.get("interestGroupOwner").as_::<JsString>()
    }

    /// Setter of the `interestGroupOwner` attribute.
    pub fn set_interest_group_owner(&mut self, value: &JsString) {
        self.inner.set("interestGroupOwner", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `renderURL` attribute.
    pub fn render_url(&self) -> JsString {
        self.inner.get("renderURL").as_::<JsString>()
    }

    /// Setter of the `renderURL` attribute.
    pub fn set_render_url(&mut self, value: &JsString) {
        self.inner.set("renderURL", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `bid` attribute.
    pub fn bid(&self) -> f64 {
        self.inner.get("bid").as_::<f64>()
    }

    /// Setter of the `bid` attribute.
    pub fn set_bid(&mut self, value: f64) {
        self.inner.set("bid", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `highestScoringOtherBid` attribute.
    pub fn highest_scoring_other_bid(&self) -> f64 {
        self.inner.get("highestScoringOtherBid").as_::<f64>()
    }

    /// Setter of the `highestScoringOtherBid` attribute.
    pub fn set_highest_scoring_other_bid(&mut self, value: f64) {
        self.inner.set("highestScoringOtherBid", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `bidCurrency` attribute.
    pub fn bid_currency(&self) -> JsString {
        self.inner.get("bidCurrency").as_::<JsString>()
    }

    /// Setter of the `bidCurrency` attribute.
    pub fn set_bid_currency(&mut self, value: &JsString) {
        self.inner.set("bidCurrency", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `highestScoringOtherBidCurrency` attribute.
    pub fn highest_scoring_other_bid_currency(&self) -> JsString {
        self.inner.get("highestScoringOtherBidCurrency").as_::<JsString>()
    }

    /// Setter of the `highestScoringOtherBidCurrency` attribute.
    pub fn set_highest_scoring_other_bid_currency(&mut self, value: &JsString) {
        self.inner.set("highestScoringOtherBidCurrency", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `topLevelSeller` attribute.
    pub fn top_level_seller(&self) -> JsString {
        self.inner.get("topLevelSeller").as_::<JsString>()
    }

    /// Setter of the `topLevelSeller` attribute.
    pub fn set_top_level_seller(&mut self, value: &JsString) {
        self.inner.set("topLevelSeller", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `componentSeller` attribute.
    pub fn component_seller(&self) -> JsString {
        self.inner.get("componentSeller").as_::<JsString>()
    }

    /// Setter of the `componentSeller` attribute.
    pub fn set_component_seller(&mut self, value: &JsString) {
        self.inner.set("componentSeller", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `buyerAndSellerReportingId` attribute.
    pub fn buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner.get("buyerAndSellerReportingId").as_::<JsString>()
    }

    /// Setter of the `buyerAndSellerReportingId` attribute.
    pub fn set_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("buyerAndSellerReportingId", value);
    }
}
impl ReportingBrowserSignals {
    /// Getter of the `selectedBuyerAndSellerReportingId` attribute.
    pub fn selected_buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner.get("selectedBuyerAndSellerReportingId").as_::<JsString>()
    }

    /// Setter of the `selectedBuyerAndSellerReportingId` attribute.
    pub fn set_selected_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("selectedBuyerAndSellerReportingId", value);
    }
}
