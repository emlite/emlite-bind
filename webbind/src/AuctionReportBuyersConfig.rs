use super::*;

/// The AuctionReportBuyersConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionReportBuyersConfig {
    inner: Any,
}

impl FromVal for AuctionReportBuyersConfig {
    fn from_val(v: &Any) -> Self {
        AuctionReportBuyersConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuctionReportBuyersConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuctionReportBuyersConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuctionReportBuyersConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuctionReportBuyersConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuctionReportBuyersConfig> for Any {
    fn from(s: AuctionReportBuyersConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuctionReportBuyersConfig> for Any {
    fn from(s: &AuctionReportBuyersConfig) -> Any {
        s.inner.clone()
    }
}

impl AuctionReportBuyersConfig {
    /// Getter of the `bucket` attribute.
    pub fn bucket(&self) -> i64 {
        self.inner.get("bucket").as_::<i64>()
    }

    /// Setter of the `bucket` attribute.
    pub fn set_bucket(&mut self, value: i64) {
        self.inner.set("bucket", value);
    }
}
impl AuctionReportBuyersConfig {
    /// Getter of the `scale` attribute.
    pub fn scale(&self) -> f64 {
        self.inner.get("scale").as_::<f64>()
    }

    /// Setter of the `scale` attribute.
    pub fn set_scale(&mut self, value: f64) {
        self.inner.set("scale", value);
    }
}
