use super::*;




/// The PurchaseDetails dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PurchaseDetails {
    inner: Any,
}

impl FromVal for PurchaseDetails {
    fn from_val(v: &Any) -> Self {
        PurchaseDetails { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PurchaseDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PurchaseDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PurchaseDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PurchaseDetails {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PurchaseDetails> for Any {
    fn from(s: PurchaseDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PurchaseDetails> for Any {
    fn from(s: &PurchaseDetails) -> Any {
        s.inner.clone()
    }
}

impl PurchaseDetails {
    /// Getter of the `itemId` attribute.
    pub fn item_id(&self) -> JsString {
        self.inner.get("itemId").as_::<JsString>()
    }

    /// Setter of the `itemId` attribute.
    pub fn set_item_id(&mut self, value: &JsString) {
        self.inner.set("itemId", value);
    }
}
impl PurchaseDetails {
    /// Getter of the `purchaseToken` attribute.
    pub fn purchase_token(&self) -> JsString {
        self.inner.get("purchaseToken").as_::<JsString>()
    }

    /// Setter of the `purchaseToken` attribute.
    pub fn set_purchase_token(&mut self, value: &JsString) {
        self.inner.set("purchaseToken", value);
    }
}
