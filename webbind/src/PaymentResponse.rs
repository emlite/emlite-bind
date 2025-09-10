use super::*;

/// The PaymentResponse class.
/// [`PaymentResponse`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentResponse {
    inner: EventTarget,
}

impl FromVal for PaymentResponse {
    fn from_val(v: &Any) -> Self {
        PaymentResponse {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaymentResponse {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaymentResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaymentResponse {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaymentResponse {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PaymentResponse> for Any {
    fn from(s: PaymentResponse) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaymentResponse> for Any {
    fn from(s: &PaymentResponse) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PaymentResponse);

impl PaymentResponse {
    /// The toJSON method.
    /// [`PaymentResponse.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl PaymentResponse {
    /// Getter of the `requestId` attribute.
    /// [`PaymentResponse.requestId`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/requestId)
    pub fn request_id(&self) -> JsString {
        self.inner.get("requestId").as_::<JsString>()
    }
}
impl PaymentResponse {
    /// Getter of the `methodName` attribute.
    /// [`PaymentResponse.methodName`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/methodName)
    pub fn method_name(&self) -> JsString {
        self.inner.get("methodName").as_::<JsString>()
    }
}
impl PaymentResponse {
    /// Getter of the `details` attribute.
    /// [`PaymentResponse.details`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/details)
    pub fn details(&self) -> Object {
        self.inner.get("details").as_::<Object>()
    }
}
impl PaymentResponse {
    /// Getter of the `shippingAddress` attribute.
    /// [`PaymentResponse.shippingAddress`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingAddress)
    pub fn shipping_address(&self) -> ContactAddress {
        self.inner.get("shippingAddress").as_::<ContactAddress>()
    }
}
impl PaymentResponse {
    /// Getter of the `shippingOption` attribute.
    /// [`PaymentResponse.shippingOption`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingOption)
    pub fn shipping_option(&self) -> JsString {
        self.inner.get("shippingOption").as_::<JsString>()
    }
}
impl PaymentResponse {
    /// Getter of the `payerName` attribute.
    /// [`PaymentResponse.payerName`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerName)
    pub fn payer_name(&self) -> JsString {
        self.inner.get("payerName").as_::<JsString>()
    }
}
impl PaymentResponse {
    /// Getter of the `payerEmail` attribute.
    /// [`PaymentResponse.payerEmail`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerEmail)
    pub fn payer_email(&self) -> JsString {
        self.inner.get("payerEmail").as_::<JsString>()
    }
}
impl PaymentResponse {
    /// Getter of the `payerPhone` attribute.
    /// [`PaymentResponse.payerPhone`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerPhone)
    pub fn payer_phone(&self) -> JsString {
        self.inner.get("payerPhone").as_::<JsString>()
    }
}
impl PaymentResponse {
    /// The complete method.
    /// [`PaymentResponse.complete`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)
    pub fn complete0(&self) -> Promise<Undefined> {
        self.inner.call("complete", &[]).as_::<Promise<Undefined>>()
    }
    /// The complete method.
    /// [`PaymentResponse.complete`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)
    pub fn complete1(&self, result: &PaymentComplete) -> Promise<Undefined> {
        self.inner
            .call("complete", &[result.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The complete method.
    /// [`PaymentResponse.complete`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)
    pub fn complete2(
        &self,
        result: &PaymentComplete,
        details: &PaymentCompleteDetails,
    ) -> Promise<Undefined> {
        self.inner
            .call("complete", &[result.into(), details.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl PaymentResponse {
    /// The retry method.
    /// [`PaymentResponse.retry`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/retry)
    pub fn retry0(&self) -> Promise<Undefined> {
        self.inner.call("retry", &[]).as_::<Promise<Undefined>>()
    }
    /// The retry method.
    /// [`PaymentResponse.retry`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/retry)
    pub fn retry1(&self, error_fields: &PaymentValidationErrors) -> Promise<Undefined> {
        self.inner
            .call("retry", &[error_fields.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl PaymentResponse {
    /// Getter of the `onpayerdetailchange` attribute.
    /// [`PaymentResponse.onpayerdetailchange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/onpayerdetailchange)
    pub fn onpayerdetailchange(&self) -> Any {
        self.inner.get("onpayerdetailchange").as_::<Any>()
    }

    /// Setter of the `onpayerdetailchange` attribute.
    /// [`PaymentResponse.onpayerdetailchange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/onpayerdetailchange)
    pub fn set_onpayerdetailchange(&mut self, value: &Any) {
        self.inner.set("onpayerdetailchange", value);
    }
}
