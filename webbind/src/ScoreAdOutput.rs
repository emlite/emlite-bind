use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScoreAdOutput {
    inner: Any,
}
impl FromVal for ScoreAdOutput {
    fn from_val(v: &Any) -> Self {
        ScoreAdOutput { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ScoreAdOutput {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScoreAdOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ScoreAdOutput {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ScoreAdOutput {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ScoreAdOutput> for Any {
    fn from(s: ScoreAdOutput) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ScoreAdOutput> for Any {
    fn from(s: &ScoreAdOutput) -> Any {
        s.inner.clone()
    }
}

impl ScoreAdOutput {
    pub fn desirability(&self) -> f64 {
        self.inner.get("desirability").as_::<f64>()
    }

    pub fn set_desirability(&mut self, value: f64) {
        self.inner.set("desirability", value);
    }
}
impl ScoreAdOutput {
    pub fn bid(&self) -> f64 {
        self.inner.get("bid").as_::<f64>()
    }

    pub fn set_bid(&mut self, value: f64) {
        self.inner.set("bid", value);
    }
}
impl ScoreAdOutput {
    pub fn bid_currency(&self) -> JsString {
        self.inner.get("bidCurrency").as_::<JsString>()
    }

    pub fn set_bid_currency(&mut self, value: &JsString) {
        self.inner.set("bidCurrency", value);
    }
}
impl ScoreAdOutput {
    pub fn incoming_bid_in_seller_currency(&self) -> f64 {
        self.inner.get("incomingBidInSellerCurrency").as_::<f64>()
    }

    pub fn set_incoming_bid_in_seller_currency(&mut self, value: f64) {
        self.inner.set("incomingBidInSellerCurrency", value);
    }
}
impl ScoreAdOutput {
    pub fn allow_component_auction(&self) -> bool {
        self.inner.get("allowComponentAuction").as_::<bool>()
    }

    pub fn set_allow_component_auction(&mut self, value: bool) {
        self.inner.set("allowComponentAuction", value);
    }
}
