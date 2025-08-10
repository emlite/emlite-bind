use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReportWinBrowserSignals {
    inner: Any,
}
impl FromVal for ReportWinBrowserSignals {
    fn from_val(v: &Any) -> Self {
        ReportWinBrowserSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReportWinBrowserSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReportWinBrowserSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReportWinBrowserSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReportWinBrowserSignals {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReportWinBrowserSignals> for Any {
    fn from(s: ReportWinBrowserSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReportWinBrowserSignals> for Any {
    fn from(s: &ReportWinBrowserSignals) -> Any {
        s.inner.clone()
    }
}

impl ReportWinBrowserSignals {
    pub fn ad_cost(&self) -> f64 {
        self.inner.get("adCost").as_::<f64>()
    }

    pub fn set_ad_cost(&mut self, value: f64) {
        self.inner.set("adCost", value);
    }
}
impl ReportWinBrowserSignals {
    pub fn seller(&self) -> JsString {
        self.inner.get("seller").as_::<JsString>()
    }

    pub fn set_seller(&mut self, value: &JsString) {
        self.inner.set("seller", value);
    }
}
impl ReportWinBrowserSignals {
    pub fn made_highest_scoring_other_bid(&self) -> bool {
        self.inner.get("madeHighestScoringOtherBid").as_::<bool>()
    }

    pub fn set_made_highest_scoring_other_bid(&mut self, value: bool) {
        self.inner.set("madeHighestScoringOtherBid", value);
    }
}
impl ReportWinBrowserSignals {
    pub fn interest_group_name(&self) -> JsString {
        self.inner.get("interestGroupName").as_::<JsString>()
    }

    pub fn set_interest_group_name(&mut self, value: &JsString) {
        self.inner.set("interestGroupName", value);
    }
}
impl ReportWinBrowserSignals {
    pub fn buyer_reporting_id(&self) -> JsString {
        self.inner.get("buyerReportingId").as_::<JsString>()
    }

    pub fn set_buyer_reporting_id(&mut self, value: &JsString) {
        self.inner.set("buyerReportingId", value);
    }
}
impl ReportWinBrowserSignals {
    pub fn modeling_signals(&self) -> u16 {
        self.inner.get("modelingSignals").as_::<u16>()
    }

    pub fn set_modeling_signals(&mut self, value: u16) {
        self.inner.set("modelingSignals", value);
    }
}
impl ReportWinBrowserSignals {
    pub fn data_version(&self) -> u32 {
        self.inner.get("dataVersion").as_::<u32>()
    }

    pub fn set_data_version(&mut self, value: u32) {
        self.inner.set("dataVersion", value);
    }
}
impl ReportWinBrowserSignals {
    pub fn k_anon_status(&self) -> KAnonStatus {
        self.inner.get("kAnonStatus").as_::<KAnonStatus>()
    }

    pub fn set_k_anon_status(&mut self, value: &KAnonStatus) {
        self.inner.set("kAnonStatus", value);
    }
}
