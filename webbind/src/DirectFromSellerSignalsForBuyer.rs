use super::*;

/// The DirectFromSellerSignalsForBuyer dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DirectFromSellerSignalsForBuyer {
    inner: Any,
}

impl FromVal for DirectFromSellerSignalsForBuyer {
    fn from_val(v: &Any) -> Self {
        DirectFromSellerSignalsForBuyer { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DirectFromSellerSignalsForBuyer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DirectFromSellerSignalsForBuyer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DirectFromSellerSignalsForBuyer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DirectFromSellerSignalsForBuyer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DirectFromSellerSignalsForBuyer> for Any {
    fn from(s: DirectFromSellerSignalsForBuyer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DirectFromSellerSignalsForBuyer> for Any {
    fn from(s: &DirectFromSellerSignalsForBuyer) -> Any {
        s.inner.clone()
    }
}

impl DirectFromSellerSignalsForBuyer {
    /// Getter of the `auctionSignals` attribute.
    pub fn auction_signals(&self) -> Any {
        self.inner.get("auctionSignals").as_::<Any>()
    }

    /// Setter of the `auctionSignals` attribute.
    pub fn set_auction_signals(&mut self, value: &Any) {
        self.inner.set("auctionSignals", value);
    }
}
impl DirectFromSellerSignalsForBuyer {
    /// Getter of the `perBuyerSignals` attribute.
    pub fn per_buyer_signals(&self) -> Any {
        self.inner.get("perBuyerSignals").as_::<Any>()
    }

    /// Setter of the `perBuyerSignals` attribute.
    pub fn set_per_buyer_signals(&mut self, value: &Any) {
        self.inner.set("perBuyerSignals", value);
    }
}
