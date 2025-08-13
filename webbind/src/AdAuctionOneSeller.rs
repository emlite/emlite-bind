use super::*;




/// The AdAuctionOneSeller dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdAuctionOneSeller {
    inner: Any,
}

impl FromVal for AdAuctionOneSeller {
    fn from_val(v: &Any) -> Self {
        AdAuctionOneSeller { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AdAuctionOneSeller {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AdAuctionOneSeller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AdAuctionOneSeller {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AdAuctionOneSeller {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AdAuctionOneSeller> for Any {
    fn from(s: AdAuctionOneSeller) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AdAuctionOneSeller> for Any {
    fn from(s: &AdAuctionOneSeller) -> Any {
        s.inner.clone()
    }
}

impl AdAuctionOneSeller {
    /// Getter of the `seller` attribute.
    pub fn seller(&self) -> JsString {
        self.inner.get("seller").as_::<JsString>()
    }

    /// Setter of the `seller` attribute.
    pub fn set_seller(&mut self, value: &JsString) {
        self.inner.set("seller", value);
    }
}
impl AdAuctionOneSeller {
    /// Getter of the `coordinatorOrigin` attribute.
    pub fn coordinator_origin(&self) -> JsString {
        self.inner.get("coordinatorOrigin").as_::<JsString>()
    }

    /// Setter of the `coordinatorOrigin` attribute.
    pub fn set_coordinator_origin(&mut self, value: &JsString) {
        self.inner.set("coordinatorOrigin", value);
    }
}
