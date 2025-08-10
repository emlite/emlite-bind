use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdAuctionPerSellerData {
    inner: Any,
}
impl FromVal for AdAuctionPerSellerData {
    fn from_val(v: &Any) -> Self {
        AdAuctionPerSellerData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AdAuctionPerSellerData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AdAuctionPerSellerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AdAuctionPerSellerData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AdAuctionPerSellerData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AdAuctionPerSellerData> for Any {
    fn from(s: AdAuctionPerSellerData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AdAuctionPerSellerData> for Any {
    fn from(s: &AdAuctionPerSellerData) -> Any {
        s.inner.clone()
    }
}

impl AdAuctionPerSellerData {
    pub fn seller(&self) -> JsString {
        self.inner.get("seller").as_::<JsString>()
    }

    pub fn set_seller(&mut self, value: &JsString) {
        self.inner.set("seller", value);
    }
}
impl AdAuctionPerSellerData {
    pub fn request(&self) -> Uint8Array {
        self.inner.get("request").as_::<Uint8Array>()
    }

    pub fn set_request(&mut self, value: &Uint8Array) {
        self.inner.set("request", value);
    }
}
impl AdAuctionPerSellerData {
    pub fn error(&self) -> JsString {
        self.inner.get("error").as_::<JsString>()
    }

    pub fn set_error(&mut self, value: &JsString) {
        self.inner.set("error", value);
    }
}
