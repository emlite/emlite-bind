use super::*;




/// The AuctionRealTimeReportingConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionRealTimeReportingConfig {
    inner: Any,
}

impl FromVal for AuctionRealTimeReportingConfig {
    fn from_val(v: &Any) -> Self {
        AuctionRealTimeReportingConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuctionRealTimeReportingConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuctionRealTimeReportingConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuctionRealTimeReportingConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuctionRealTimeReportingConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuctionRealTimeReportingConfig> for Any {
    fn from(s: AuctionRealTimeReportingConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuctionRealTimeReportingConfig> for Any {
    fn from(s: &AuctionRealTimeReportingConfig) -> Any {
        s.inner.clone()
    }
}

impl AuctionRealTimeReportingConfig {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
