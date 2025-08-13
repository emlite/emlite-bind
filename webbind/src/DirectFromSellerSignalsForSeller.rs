use super::*;




/// The DirectFromSellerSignalsForSeller dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DirectFromSellerSignalsForSeller {
    inner: Any,
}

impl FromVal for DirectFromSellerSignalsForSeller {
    fn from_val(v: &Any) -> Self {
        DirectFromSellerSignalsForSeller { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DirectFromSellerSignalsForSeller {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DirectFromSellerSignalsForSeller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DirectFromSellerSignalsForSeller {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DirectFromSellerSignalsForSeller {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DirectFromSellerSignalsForSeller> for Any {
    fn from(s: DirectFromSellerSignalsForSeller) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DirectFromSellerSignalsForSeller> for Any {
    fn from(s: &DirectFromSellerSignalsForSeller) -> Any {
        s.inner.clone()
    }
}

impl DirectFromSellerSignalsForSeller {
    /// Getter of the `auctionSignals` attribute.
    pub fn auction_signals(&self) -> Any {
        self.inner.get("auctionSignals").as_::<Any>()
    }

    /// Setter of the `auctionSignals` attribute.
    pub fn set_auction_signals(&mut self, value: &Any) {
        self.inner.set("auctionSignals", value);
    }
}
impl DirectFromSellerSignalsForSeller {
    /// Getter of the `sellerSignals` attribute.
    pub fn seller_signals(&self) -> Any {
        self.inner.get("sellerSignals").as_::<Any>()
    }

    /// Setter of the `sellerSignals` attribute.
    pub fn set_seller_signals(&mut self, value: &Any) {
        self.inner.set("sellerSignals", value);
    }
}
