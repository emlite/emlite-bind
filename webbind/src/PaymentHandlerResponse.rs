use super::*;




/// The PaymentHandlerResponse dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentHandlerResponse {
    inner: Any,
}

impl FromVal for PaymentHandlerResponse {
    fn from_val(v: &Any) -> Self {
        PaymentHandlerResponse { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentHandlerResponse {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentHandlerResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentHandlerResponse {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentHandlerResponse {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaymentHandlerResponse> for Any {
    fn from(s: PaymentHandlerResponse) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentHandlerResponse> for Any {
    fn from(s: &PaymentHandlerResponse) -> Any {
        s.inner.clone()
    }
}

impl PaymentHandlerResponse {
    /// Getter of the `methodName` attribute.
    pub fn method_name(&self) -> JsString {
        self.inner.get("methodName").as_::<JsString>()
    }

    /// Setter of the `methodName` attribute.
    pub fn set_method_name(&mut self, value: &JsString) {
        self.inner.set("methodName", value);
    }
}
impl PaymentHandlerResponse {
    /// Getter of the `details` attribute.
    pub fn details(&self) -> Object {
        self.inner.get("details").as_::<Object>()
    }

    /// Setter of the `details` attribute.
    pub fn set_details(&mut self, value: &Object) {
        self.inner.set("details", value);
    }
}
impl PaymentHandlerResponse {
    /// Getter of the `payerName` attribute.
    pub fn payer_name(&self) -> JsString {
        self.inner.get("payerName").as_::<JsString>()
    }

    /// Setter of the `payerName` attribute.
    pub fn set_payer_name(&mut self, value: &JsString) {
        self.inner.set("payerName", value);
    }
}
impl PaymentHandlerResponse {
    /// Getter of the `payerEmail` attribute.
    pub fn payer_email(&self) -> JsString {
        self.inner.get("payerEmail").as_::<JsString>()
    }

    /// Setter of the `payerEmail` attribute.
    pub fn set_payer_email(&mut self, value: &JsString) {
        self.inner.set("payerEmail", value);
    }
}
impl PaymentHandlerResponse {
    /// Getter of the `payerPhone` attribute.
    pub fn payer_phone(&self) -> JsString {
        self.inner.get("payerPhone").as_::<JsString>()
    }

    /// Setter of the `payerPhone` attribute.
    pub fn set_payer_phone(&mut self, value: &JsString) {
        self.inner.set("payerPhone", value);
    }
}
impl PaymentHandlerResponse {
    /// Getter of the `shippingAddress` attribute.
    pub fn shipping_address(&self) -> AddressInit {
        self.inner.get("shippingAddress").as_::<AddressInit>()
    }

    /// Setter of the `shippingAddress` attribute.
    pub fn set_shipping_address(&mut self, value: &AddressInit) {
        self.inner.set("shippingAddress", value);
    }
}
impl PaymentHandlerResponse {
    /// Getter of the `shippingOption` attribute.
    pub fn shipping_option(&self) -> JsString {
        self.inner.get("shippingOption").as_::<JsString>()
    }

    /// Setter of the `shippingOption` attribute.
    pub fn set_shipping_option(&mut self, value: &JsString) {
        self.inner.set("shippingOption", value);
    }
}
