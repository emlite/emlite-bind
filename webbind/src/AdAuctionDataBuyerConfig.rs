use super::*;




/// The AdAuctionDataBuyerConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AdAuctionDataBuyerConfig {
    inner: Any,
}

impl FromVal for AdAuctionDataBuyerConfig {
    fn from_val(v: &Any) -> Self {
        AdAuctionDataBuyerConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AdAuctionDataBuyerConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AdAuctionDataBuyerConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AdAuctionDataBuyerConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AdAuctionDataBuyerConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AdAuctionDataBuyerConfig> for Any {
    fn from(s: AdAuctionDataBuyerConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AdAuctionDataBuyerConfig> for Any {
    fn from(s: &AdAuctionDataBuyerConfig) -> Any {
        s.inner.clone()
    }
}

impl AdAuctionDataBuyerConfig {
    /// Getter of the `targetSize` attribute.
    pub fn target_size(&self) -> u32 {
        self.inner.get("targetSize").as_::<u32>()
    }

    /// Setter of the `targetSize` attribute.
    pub fn set_target_size(&mut self, value: u32) {
        self.inner.set("targetSize", value);
    }
}
