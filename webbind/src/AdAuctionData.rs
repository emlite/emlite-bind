use super::*;

/// The AdAuctionData dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdAuctionData {
    inner: Any,
}

impl FromVal for AdAuctionData {
    fn from_val(v: &Any) -> Self {
        AdAuctionData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AdAuctionData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AdAuctionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AdAuctionData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AdAuctionData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AdAuctionData> for Any {
    fn from(s: AdAuctionData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AdAuctionData> for Any {
    fn from(s: &AdAuctionData) -> Any {
        s.inner.clone()
    }
}

impl AdAuctionData {
    /// Getter of the `requestId` attribute.
    pub fn request_id(&self) -> JsString {
        self.inner.get("requestId").as_::<JsString>()
    }

    /// Setter of the `requestId` attribute.
    pub fn set_request_id(&mut self, value: &JsString) {
        self.inner.set("requestId", value);
    }
}
impl AdAuctionData {
    /// Getter of the `request` attribute.
    pub fn request(&self) -> Uint8Array {
        self.inner.get("request").as_::<Uint8Array>()
    }

    /// Setter of the `request` attribute.
    pub fn set_request(&mut self, value: &Uint8Array) {
        self.inner.set("request", value);
    }
}
impl AdAuctionData {
    /// Getter of the `requests` attribute.
    pub fn requests(&self) -> TypedArray<AdAuctionPerSellerData> {
        self.inner
            .get("requests")
            .as_::<TypedArray<AdAuctionPerSellerData>>()
    }

    /// Setter of the `requests` attribute.
    pub fn set_requests(&mut self, value: &TypedArray<AdAuctionPerSellerData>) {
        self.inner.set("requests", value);
    }
}
