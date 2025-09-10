use super::*;

/// The AdAuctionDataConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdAuctionDataConfig {
    inner: Any,
}

impl FromVal for AdAuctionDataConfig {
    fn from_val(v: &Any) -> Self {
        AdAuctionDataConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AdAuctionDataConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AdAuctionDataConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AdAuctionDataConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AdAuctionDataConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AdAuctionDataConfig> for Any {
    fn from(s: AdAuctionDataConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AdAuctionDataConfig> for Any {
    fn from(s: &AdAuctionDataConfig) -> Any {
        s.inner.clone()
    }
}

impl AdAuctionDataConfig {
    /// Getter of the `seller` attribute.
    pub fn seller(&self) -> JsString {
        self.inner.get("seller").as_::<JsString>()
    }

    /// Setter of the `seller` attribute.
    pub fn set_seller(&mut self, value: &JsString) {
        self.inner.set("seller", value);
    }
}
impl AdAuctionDataConfig {
    /// Getter of the `coordinatorOrigin` attribute.
    pub fn coordinator_origin(&self) -> JsString {
        self.inner.get("coordinatorOrigin").as_::<JsString>()
    }

    /// Setter of the `coordinatorOrigin` attribute.
    pub fn set_coordinator_origin(&mut self, value: &JsString) {
        self.inner.set("coordinatorOrigin", value);
    }
}
impl AdAuctionDataConfig {
    /// Getter of the `sellers` attribute.
    pub fn sellers(&self) -> TypedArray<AdAuctionOneSeller> {
        self.inner
            .get("sellers")
            .as_::<TypedArray<AdAuctionOneSeller>>()
    }

    /// Setter of the `sellers` attribute.
    pub fn set_sellers(&mut self, value: &TypedArray<AdAuctionOneSeller>) {
        self.inner.set("sellers", value);
    }
}
impl AdAuctionDataConfig {
    /// Getter of the `requestSize` attribute.
    pub fn request_size(&self) -> u32 {
        self.inner.get("requestSize").as_::<u32>()
    }

    /// Setter of the `requestSize` attribute.
    pub fn set_request_size(&mut self, value: u32) {
        self.inner.set("requestSize", value);
    }
}
impl AdAuctionDataConfig {
    /// Getter of the `perBuyerConfig` attribute.
    pub fn per_buyer_config(&self) -> Record<JsString, AdAuctionDataBuyerConfig> {
        self.inner
            .get("perBuyerConfig")
            .as_::<Record<JsString, AdAuctionDataBuyerConfig>>()
    }

    /// Setter of the `perBuyerConfig` attribute.
    pub fn set_per_buyer_config(&mut self, value: &Record<JsString, AdAuctionDataBuyerConfig>) {
        self.inner.set("perBuyerConfig", value);
    }
}
