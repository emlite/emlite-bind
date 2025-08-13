use super::*;




/// The PaymentDetailsBase dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentDetailsBase {
    inner: Any,
}

impl FromVal for PaymentDetailsBase {
    fn from_val(v: &Any) -> Self {
        PaymentDetailsBase { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentDetailsBase {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentDetailsBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentDetailsBase {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentDetailsBase {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentDetailsBase> for Any {
    fn from(s: PaymentDetailsBase) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentDetailsBase> for Any {
    fn from(s: &PaymentDetailsBase) -> Any {
        s.inner.clone()
    }
}

impl PaymentDetailsBase {
    /// Getter of the `displayItems` attribute.
    pub fn display_items(&self) -> TypedArray<PaymentItem> {
        self.inner.get("displayItems").as_::<TypedArray<PaymentItem>>()
    }

    /// Setter of the `displayItems` attribute.
    pub fn set_display_items(&mut self, value: &TypedArray<PaymentItem>) {
        self.inner.set("displayItems", value);
    }
}
impl PaymentDetailsBase {
    /// Getter of the `shippingOptions` attribute.
    pub fn shipping_options(&self) -> TypedArray<PaymentShippingOption> {
        self.inner.get("shippingOptions").as_::<TypedArray<PaymentShippingOption>>()
    }

    /// Setter of the `shippingOptions` attribute.
    pub fn set_shipping_options(&mut self, value: &TypedArray<PaymentShippingOption>) {
        self.inner.set("shippingOptions", value);
    }
}
impl PaymentDetailsBase {
    /// Getter of the `modifiers` attribute.
    pub fn modifiers(&self) -> TypedArray<PaymentDetailsModifier> {
        self.inner.get("modifiers").as_::<TypedArray<PaymentDetailsModifier>>()
    }

    /// Setter of the `modifiers` attribute.
    pub fn set_modifiers(&mut self, value: &TypedArray<PaymentDetailsModifier>) {
        self.inner.set("modifiers", value);
    }
}
