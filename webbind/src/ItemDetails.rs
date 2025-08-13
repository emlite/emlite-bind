use super::*;




/// The ItemDetails dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ItemDetails {
    inner: Any,
}

impl FromVal for ItemDetails {
    fn from_val(v: &Any) -> Self {
        ItemDetails { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ItemDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ItemDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ItemDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ItemDetails {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ItemDetails> for Any {
    fn from(s: ItemDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ItemDetails> for Any {
    fn from(s: &ItemDetails) -> Any {
        s.inner.clone()
    }
}

impl ItemDetails {
    /// Getter of the `itemId` attribute.
    pub fn item_id(&self) -> JsString {
        self.inner.get("itemId").as_::<JsString>()
    }

    /// Setter of the `itemId` attribute.
    pub fn set_item_id(&mut self, value: &JsString) {
        self.inner.set("itemId", value);
    }
}
impl ItemDetails {
    /// Getter of the `title` attribute.
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl ItemDetails {
    /// Getter of the `price` attribute.
    pub fn price(&self) -> PaymentCurrencyAmount {
        self.inner.get("price").as_::<PaymentCurrencyAmount>()
    }

    /// Setter of the `price` attribute.
    pub fn set_price(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("price", value);
    }
}
impl ItemDetails {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> ItemType {
        self.inner.get("type").as_::<ItemType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &ItemType) {
        self.inner.set("type", value);
    }
}
impl ItemDetails {
    /// Getter of the `description` attribute.
    pub fn description(&self) -> JsString {
        self.inner.get("description").as_::<JsString>()
    }

    /// Setter of the `description` attribute.
    pub fn set_description(&mut self, value: &JsString) {
        self.inner.set("description", value);
    }
}
impl ItemDetails {
    /// Getter of the `iconURLs` attribute.
    pub fn icon_ur_ls(&self) -> TypedArray<JsString> {
        self.inner.get("iconURLs").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `iconURLs` attribute.
    pub fn set_icon_ur_ls(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("iconURLs", value);
    }
}
impl ItemDetails {
    /// Getter of the `subscriptionPeriod` attribute.
    pub fn subscription_period(&self) -> JsString {
        self.inner.get("subscriptionPeriod").as_::<JsString>()
    }

    /// Setter of the `subscriptionPeriod` attribute.
    pub fn set_subscription_period(&mut self, value: &JsString) {
        self.inner.set("subscriptionPeriod", value);
    }
}
impl ItemDetails {
    /// Getter of the `freeTrialPeriod` attribute.
    pub fn free_trial_period(&self) -> JsString {
        self.inner.get("freeTrialPeriod").as_::<JsString>()
    }

    /// Setter of the `freeTrialPeriod` attribute.
    pub fn set_free_trial_period(&mut self, value: &JsString) {
        self.inner.set("freeTrialPeriod", value);
    }
}
impl ItemDetails {
    /// Getter of the `introductoryPrice` attribute.
    pub fn introductory_price(&self) -> PaymentCurrencyAmount {
        self.inner.get("introductoryPrice").as_::<PaymentCurrencyAmount>()
    }

    /// Setter of the `introductoryPrice` attribute.
    pub fn set_introductory_price(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("introductoryPrice", value);
    }
}
impl ItemDetails {
    /// Getter of the `introductoryPricePeriod` attribute.
    pub fn introductory_price_period(&self) -> JsString {
        self.inner.get("introductoryPricePeriod").as_::<JsString>()
    }

    /// Setter of the `introductoryPricePeriod` attribute.
    pub fn set_introductory_price_period(&mut self, value: &JsString) {
        self.inner.set("introductoryPricePeriod", value);
    }
}
impl ItemDetails {
    /// Getter of the `introductoryPriceCycles` attribute.
    pub fn introductory_price_cycles(&self) -> u64 {
        self.inner.get("introductoryPriceCycles").as_::<u64>()
    }

    /// Setter of the `introductoryPriceCycles` attribute.
    pub fn set_introductory_price_cycles(&mut self, value: u64) {
        self.inner.set("introductoryPriceCycles", value);
    }
}
