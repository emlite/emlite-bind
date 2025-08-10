use super::*;

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
    pub fn top_window_hostname(&self) -> JsString {
        self.inner.get("topWindowHostname").as_::<JsString>()
    }

    pub fn set_top_window_hostname(&mut self, value: &JsString) {
        self.inner.set("topWindowHostname", value);
    }
}
impl ReportingBrowserSignals {
    pub fn interest_group_owner(&self) -> JsString {
        self.inner.get("interestGroupOwner").as_::<JsString>()
    }

    pub fn set_interest_group_owner(&mut self, value: &JsString) {
        self.inner.set("interestGroupOwner", value);
    }
}
impl ReportingBrowserSignals {
    pub fn render_url(&self) -> JsString {
        self.inner.get("renderURL").as_::<JsString>()
    }

    pub fn set_render_url(&mut self, value: &JsString) {
        self.inner.set("renderURL", value);
    }
}
impl ReportingBrowserSignals {
    pub fn bid(&self) -> f64 {
        self.inner.get("bid").as_::<f64>()
    }

    pub fn set_bid(&mut self, value: f64) {
        self.inner.set("bid", value);
    }
}
impl ReportingBrowserSignals {
    pub fn highest_scoring_other_bid(&self) -> f64 {
        self.inner.get("highestScoringOtherBid").as_::<f64>()
    }

    pub fn set_highest_scoring_other_bid(&mut self, value: f64) {
        self.inner.set("highestScoringOtherBid", value);
    }
}
impl ReportingBrowserSignals {
    pub fn bid_currency(&self) -> JsString {
        self.inner.get("bidCurrency").as_::<JsString>()
    }

    pub fn set_bid_currency(&mut self, value: &JsString) {
        self.inner.set("bidCurrency", value);
    }
}
impl ReportingBrowserSignals {
    pub fn highest_scoring_other_bid_currency(&self) -> JsString {
        self.inner
            .get("highestScoringOtherBidCurrency")
            .as_::<JsString>()
    }

    pub fn set_highest_scoring_other_bid_currency(&mut self, value: &JsString) {
        self.inner.set("highestScoringOtherBidCurrency", value);
    }
}
impl ReportingBrowserSignals {
    pub fn top_level_seller(&self) -> JsString {
        self.inner.get("topLevelSeller").as_::<JsString>()
    }

    pub fn set_top_level_seller(&mut self, value: &JsString) {
        self.inner.set("topLevelSeller", value);
    }
}
impl ReportingBrowserSignals {
    pub fn component_seller(&self) -> JsString {
        self.inner.get("componentSeller").as_::<JsString>()
    }

    pub fn set_component_seller(&mut self, value: &JsString) {
        self.inner.set("componentSeller", value);
    }
}
impl ReportingBrowserSignals {
    pub fn buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner
            .get("buyerAndSellerReportingId")
            .as_::<JsString>()
    }

    pub fn set_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("buyerAndSellerReportingId", value);
    }
}
impl ReportingBrowserSignals {
    pub fn selected_buyer_and_seller_reporting_id(&self) -> JsString {
        self.inner
            .get("selectedBuyerAndSellerReportingId")
            .as_::<JsString>()
    }

    pub fn set_selected_buyer_and_seller_reporting_id(&mut self, value: &JsString) {
        self.inner.set("selectedBuyerAndSellerReportingId", value);
    }
}
