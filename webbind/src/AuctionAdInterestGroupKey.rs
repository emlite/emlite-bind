use super::*;




/// The AuctionAdInterestGroupKey dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionAdInterestGroupKey {
    inner: Any,
}

impl FromVal for AuctionAdInterestGroupKey {
    fn from_val(v: &Any) -> Self {
        AuctionAdInterestGroupKey { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuctionAdInterestGroupKey {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuctionAdInterestGroupKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuctionAdInterestGroupKey {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuctionAdInterestGroupKey {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuctionAdInterestGroupKey> for Any {
    fn from(s: AuctionAdInterestGroupKey) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuctionAdInterestGroupKey> for Any {
    fn from(s: &AuctionAdInterestGroupKey) -> Any {
        s.inner.clone()
    }
}

impl AuctionAdInterestGroupKey {
    /// Getter of the `owner` attribute.
    pub fn owner(&self) -> JsString {
        self.inner.get("owner").as_::<JsString>()
    }

    /// Setter of the `owner` attribute.
    pub fn set_owner(&mut self, value: &JsString) {
        self.inner.set("owner", value);
    }
}
impl AuctionAdInterestGroupKey {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
