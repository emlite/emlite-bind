use super::*;

/// The PaymentRequestEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequestEventInit {
    inner: Any,
}

impl FromVal for PaymentRequestEventInit {
    fn from_val(v: &Any) -> Self {
        PaymentRequestEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentRequestEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentRequestEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentRequestEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentRequestEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PaymentRequestEventInit> for Any {
    fn from(s: PaymentRequestEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentRequestEventInit> for Any {
    fn from(s: &PaymentRequestEventInit) -> Any {
        s.inner.clone()
    }
}

impl PaymentRequestEventInit {
    /// Getter of the `topOrigin` attribute.
    pub fn top_origin(&self) -> JsString {
        self.inner.get("topOrigin").as_::<JsString>()
    }

    /// Setter of the `topOrigin` attribute.
    pub fn set_top_origin(&mut self, value: &JsString) {
        self.inner.set("topOrigin", value);
    }
}
impl PaymentRequestEventInit {
    /// Getter of the `paymentRequestOrigin` attribute.
    pub fn payment_request_origin(&self) -> JsString {
        self.inner.get("paymentRequestOrigin").as_::<JsString>()
    }

    /// Setter of the `paymentRequestOrigin` attribute.
    pub fn set_payment_request_origin(&mut self, value: &JsString) {
        self.inner.set("paymentRequestOrigin", value);
    }
}
impl PaymentRequestEventInit {
    /// Getter of the `paymentRequestId` attribute.
    pub fn payment_request_id(&self) -> JsString {
        self.inner.get("paymentRequestId").as_::<JsString>()
    }

    /// Setter of the `paymentRequestId` attribute.
    pub fn set_payment_request_id(&mut self, value: &JsString) {
        self.inner.set("paymentRequestId", value);
    }
}
impl PaymentRequestEventInit {
    /// Getter of the `methodData` attribute.
    pub fn method_data(&self) -> TypedArray<PaymentMethodData> {
        self.inner
            .get("methodData")
            .as_::<TypedArray<PaymentMethodData>>()
    }

    /// Setter of the `methodData` attribute.
    pub fn set_method_data(&mut self, value: &TypedArray<PaymentMethodData>) {
        self.inner.set("methodData", value);
    }
}
impl PaymentRequestEventInit {
    /// Getter of the `total` attribute.
    pub fn total(&self) -> PaymentCurrencyAmount {
        self.inner.get("total").as_::<PaymentCurrencyAmount>()
    }

    /// Setter of the `total` attribute.
    pub fn set_total(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("total", value);
    }
}
impl PaymentRequestEventInit {
    /// Getter of the `modifiers` attribute.
    pub fn modifiers(&self) -> TypedArray<PaymentDetailsModifier> {
        self.inner
            .get("modifiers")
            .as_::<TypedArray<PaymentDetailsModifier>>()
    }

    /// Setter of the `modifiers` attribute.
    pub fn set_modifiers(&mut self, value: &TypedArray<PaymentDetailsModifier>) {
        self.inner.set("modifiers", value);
    }
}
impl PaymentRequestEventInit {
    /// Getter of the `paymentOptions` attribute.
    pub fn payment_options(&self) -> PaymentOptions {
        self.inner.get("paymentOptions").as_::<PaymentOptions>()
    }

    /// Setter of the `paymentOptions` attribute.
    pub fn set_payment_options(&mut self, value: &PaymentOptions) {
        self.inner.set("paymentOptions", value);
    }
}
impl PaymentRequestEventInit {
    /// Getter of the `shippingOptions` attribute.
    pub fn shipping_options(&self) -> TypedArray<PaymentShippingOption> {
        self.inner
            .get("shippingOptions")
            .as_::<TypedArray<PaymentShippingOption>>()
    }

    /// Setter of the `shippingOptions` attribute.
    pub fn set_shipping_options(&mut self, value: &TypedArray<PaymentShippingOption>) {
        self.inner.set("shippingOptions", value);
    }
}
