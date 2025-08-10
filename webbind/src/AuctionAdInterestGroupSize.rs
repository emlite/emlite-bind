use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuctionAdInterestGroupSize {
    inner: Any,
}
impl FromVal for AuctionAdInterestGroupSize {
    fn from_val(v: &Any) -> Self {
        AuctionAdInterestGroupSize { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuctionAdInterestGroupSize {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuctionAdInterestGroupSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuctionAdInterestGroupSize {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuctionAdInterestGroupSize {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuctionAdInterestGroupSize> for Any {
    fn from(s: AuctionAdInterestGroupSize) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuctionAdInterestGroupSize> for Any {
    fn from(s: &AuctionAdInterestGroupSize) -> Any {
        s.inner.clone()
    }
}

impl AuctionAdInterestGroupSize {
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl AuctionAdInterestGroupSize {
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
