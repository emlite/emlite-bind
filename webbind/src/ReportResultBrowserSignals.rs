use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReportResultBrowserSignals {
    inner: Any,
}
impl FromVal for ReportResultBrowserSignals {
    fn from_val(v: &Any) -> Self {
        ReportResultBrowserSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReportResultBrowserSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReportResultBrowserSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReportResultBrowserSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReportResultBrowserSignals {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReportResultBrowserSignals> for Any {
    fn from(s: ReportResultBrowserSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReportResultBrowserSignals> for Any {
    fn from(s: &ReportResultBrowserSignals) -> Any {
        s.inner.clone()
    }
}

impl ReportResultBrowserSignals {
    pub fn desirability(&self) -> f64 {
        self.inner.get("desirability").as_::<f64>()
    }

    pub fn set_desirability(&mut self, value: f64) {
        self.inner.set("desirability", value);
    }
}
impl ReportResultBrowserSignals {
    pub fn top_level_seller_signals(&self) -> JsString {
        self.inner.get("topLevelSellerSignals").as_::<JsString>()
    }

    pub fn set_top_level_seller_signals(&mut self, value: &JsString) {
        self.inner.set("topLevelSellerSignals", value);
    }
}
impl ReportResultBrowserSignals {
    pub fn modified_bid(&self) -> f64 {
        self.inner.get("modifiedBid").as_::<f64>()
    }

    pub fn set_modified_bid(&mut self, value: f64) {
        self.inner.set("modifiedBid", value);
    }
}
impl ReportResultBrowserSignals {
    pub fn data_version(&self) -> u32 {
        self.inner.get("dataVersion").as_::<u32>()
    }

    pub fn set_data_version(&mut self, value: u32) {
        self.inner.set("dataVersion", value);
    }
}
