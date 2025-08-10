use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionReportBuyerDebugModeConfig {
    inner: Any,
}
impl FromVal for AuctionReportBuyerDebugModeConfig {
    fn from_val(v: &Any) -> Self {
        AuctionReportBuyerDebugModeConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuctionReportBuyerDebugModeConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuctionReportBuyerDebugModeConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuctionReportBuyerDebugModeConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuctionReportBuyerDebugModeConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuctionReportBuyerDebugModeConfig> for Any {
    fn from(s: AuctionReportBuyerDebugModeConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuctionReportBuyerDebugModeConfig> for Any {
    fn from(s: &AuctionReportBuyerDebugModeConfig) -> Any {
        s.inner.clone()
    }
}

impl AuctionReportBuyerDebugModeConfig {
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl AuctionReportBuyerDebugModeConfig {
    pub fn debug_key(&self) -> i64 {
        self.inner.get("debugKey").as_::<i64>()
    }

    pub fn set_debug_key(&mut self, value: i64) {
        self.inner.set("debugKey", value);
    }
}
