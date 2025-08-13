use super::*;




/// The PaymentValidationErrors dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentValidationErrors {
    inner: Any,
}

impl FromVal for PaymentValidationErrors {
    fn from_val(v: &Any) -> Self {
        PaymentValidationErrors { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentValidationErrors {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentValidationErrors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentValidationErrors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentValidationErrors {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentValidationErrors> for Any {
    fn from(s: PaymentValidationErrors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentValidationErrors> for Any {
    fn from(s: &PaymentValidationErrors) -> Any {
        s.inner.clone()
    }
}

impl PaymentValidationErrors {
    /// Getter of the `payer` attribute.
    pub fn payer(&self) -> PayerErrors {
        self.inner.get("payer").as_::<PayerErrors>()
    }

    /// Setter of the `payer` attribute.
    pub fn set_payer(&mut self, value: &PayerErrors) {
        self.inner.set("payer", value);
    }
}
impl PaymentValidationErrors {
    /// Getter of the `shippingAddress` attribute.
    pub fn shipping_address(&self) -> AddressErrors {
        self.inner.get("shippingAddress").as_::<AddressErrors>()
    }

    /// Setter of the `shippingAddress` attribute.
    pub fn set_shipping_address(&mut self, value: &AddressErrors) {
        self.inner.set("shippingAddress", value);
    }
}
impl PaymentValidationErrors {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> JsString {
        self.inner.get("error").as_::<JsString>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &JsString) {
        self.inner.set("error", value);
    }
}
impl PaymentValidationErrors {
    /// Getter of the `paymentMethod` attribute.
    pub fn payment_method(&self) -> Object {
        self.inner.get("paymentMethod").as_::<Object>()
    }

    /// Setter of the `paymentMethod` attribute.
    pub fn set_payment_method(&mut self, value: &Object) {
        self.inner.set("paymentMethod", value);
    }
}
