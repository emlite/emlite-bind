use super::*;




/// The PaymentRequestDetailsUpdate dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequestDetailsUpdate {
    inner: Any,
}

impl FromVal for PaymentRequestDetailsUpdate {
    fn from_val(v: &Any) -> Self {
        PaymentRequestDetailsUpdate { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentRequestDetailsUpdate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentRequestDetailsUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentRequestDetailsUpdate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentRequestDetailsUpdate {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentRequestDetailsUpdate> for Any {
    fn from(s: PaymentRequestDetailsUpdate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentRequestDetailsUpdate> for Any {
    fn from(s: &PaymentRequestDetailsUpdate) -> Any {
        s.inner.clone()
    }
}

impl PaymentRequestDetailsUpdate {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> JsString {
        self.inner.get("error").as_::<JsString>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &JsString) {
        self.inner.set("error", value);
    }
}
impl PaymentRequestDetailsUpdate {
    /// Getter of the `total` attribute.
    pub fn total(&self) -> PaymentCurrencyAmount {
        self.inner.get("total").as_::<PaymentCurrencyAmount>()
    }

    /// Setter of the `total` attribute.
    pub fn set_total(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("total", value);
    }
}
impl PaymentRequestDetailsUpdate {
    /// Getter of the `modifiers` attribute.
    pub fn modifiers(&self) -> TypedArray<PaymentDetailsModifier> {
        self.inner.get("modifiers").as_::<TypedArray<PaymentDetailsModifier>>()
    }

    /// Setter of the `modifiers` attribute.
    pub fn set_modifiers(&mut self, value: &TypedArray<PaymentDetailsModifier>) {
        self.inner.set("modifiers", value);
    }
}
impl PaymentRequestDetailsUpdate {
    /// Getter of the `shippingOptions` attribute.
    pub fn shipping_options(&self) -> TypedArray<PaymentShippingOption> {
        self.inner.get("shippingOptions").as_::<TypedArray<PaymentShippingOption>>()
    }

    /// Setter of the `shippingOptions` attribute.
    pub fn set_shipping_options(&mut self, value: &TypedArray<PaymentShippingOption>) {
        self.inner.set("shippingOptions", value);
    }
}
impl PaymentRequestDetailsUpdate {
    /// Getter of the `paymentMethodErrors` attribute.
    pub fn payment_method_errors(&self) -> Object {
        self.inner.get("paymentMethodErrors").as_::<Object>()
    }

    /// Setter of the `paymentMethodErrors` attribute.
    pub fn set_payment_method_errors(&mut self, value: &Object) {
        self.inner.set("paymentMethodErrors", value);
    }
}
impl PaymentRequestDetailsUpdate {
    /// Getter of the `shippingAddressErrors` attribute.
    pub fn shipping_address_errors(&self) -> AddressErrors {
        self.inner.get("shippingAddressErrors").as_::<AddressErrors>()
    }

    /// Setter of the `shippingAddressErrors` attribute.
    pub fn set_shipping_address_errors(&mut self, value: &AddressErrors) {
        self.inner.set("shippingAddressErrors", value);
    }
}
