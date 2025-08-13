use super::*;




/// The PaymentDetailsUpdate dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentDetailsUpdate {
    inner: Any,
}

impl FromVal for PaymentDetailsUpdate {
    fn from_val(v: &Any) -> Self {
        PaymentDetailsUpdate { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentDetailsUpdate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentDetailsUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentDetailsUpdate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentDetailsUpdate {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentDetailsUpdate> for Any {
    fn from(s: PaymentDetailsUpdate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentDetailsUpdate> for Any {
    fn from(s: &PaymentDetailsUpdate) -> Any {
        s.inner.clone()
    }
}

impl PaymentDetailsUpdate {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> JsString {
        self.inner.get("error").as_::<JsString>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &JsString) {
        self.inner.set("error", value);
    }
}
impl PaymentDetailsUpdate {
    /// Getter of the `total` attribute.
    pub fn total(&self) -> PaymentItem {
        self.inner.get("total").as_::<PaymentItem>()
    }

    /// Setter of the `total` attribute.
    pub fn set_total(&mut self, value: &PaymentItem) {
        self.inner.set("total", value);
    }
}
impl PaymentDetailsUpdate {
    /// Getter of the `shippingAddressErrors` attribute.
    pub fn shipping_address_errors(&self) -> AddressErrors {
        self.inner.get("shippingAddressErrors").as_::<AddressErrors>()
    }

    /// Setter of the `shippingAddressErrors` attribute.
    pub fn set_shipping_address_errors(&mut self, value: &AddressErrors) {
        self.inner.set("shippingAddressErrors", value);
    }
}
impl PaymentDetailsUpdate {
    /// Getter of the `payerErrors` attribute.
    pub fn payer_errors(&self) -> PayerErrors {
        self.inner.get("payerErrors").as_::<PayerErrors>()
    }

    /// Setter of the `payerErrors` attribute.
    pub fn set_payer_errors(&mut self, value: &PayerErrors) {
        self.inner.set("payerErrors", value);
    }
}
impl PaymentDetailsUpdate {
    /// Getter of the `paymentMethodErrors` attribute.
    pub fn payment_method_errors(&self) -> Object {
        self.inner.get("paymentMethodErrors").as_::<Object>()
    }

    /// Setter of the `paymentMethodErrors` attribute.
    pub fn set_payment_method_errors(&mut self, value: &Object) {
        self.inner.set("paymentMethodErrors", value);
    }
}
