use super::*;

/// The AuctionAdInterestGroup dictionary.
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
    /// Getter of the `priority` attribute.
    pub fn priority(&self) -> f64 {
        self.inner.get("priority").as_::<f64>()
    }

    /// Setter of the `priority` attribute.
    pub fn set_priority(&mut self, value: f64) {
        self.inner.set("priority", value);
    }
}
impl AuctionAdInterestGroup {
    /// Getter of the `prioritySignalsOverrides` attribute.
    pub fn priority_signals_overrides(&self) -> Record<JsString, f64> {
        self.inner
            .get("prioritySignalsOverrides")
            .as_::<Record<JsString, f64>>()
    }

    /// Setter of the `prioritySignalsOverrides` attribute.
    pub fn set_priority_signals_overrides(&mut self, value: Record<JsString, f64>) {
        self.inner.set("prioritySignalsOverrides", value);
    }
}
impl AuctionAdInterestGroup {
    /// Getter of the `lifetimeMs` attribute.
    pub fn lifetime_ms(&self) -> f64 {
        self.inner.get("lifetimeMs").as_::<f64>()
    }

    /// Setter of the `lifetimeMs` attribute.
    pub fn set_lifetime_ms(&mut self, value: f64) {
        self.inner.set("lifetimeMs", value);
    }
}
impl AuctionAdInterestGroup {
    /// Getter of the `additionalBidKey` attribute.
    pub fn additional_bid_key(&self) -> JsString {
        self.inner.get("additionalBidKey").as_::<JsString>()
    }

    /// Setter of the `additionalBidKey` attribute.
    pub fn set_additional_bid_key(&mut self, value: &JsString) {
        self.inner.set("additionalBidKey", value);
    }
}
impl AuctionAdInterestGroup {
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
