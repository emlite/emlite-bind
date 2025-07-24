use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentCompleteDetails {
    inner: Any,
}
impl FromVal for PaymentCompleteDetails {
    fn from_val(v: &Any) -> Self {
        PaymentCompleteDetails { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentCompleteDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentCompleteDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentCompleteDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentCompleteDetails {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentCompleteDetails> for Any {
    fn from(s: PaymentCompleteDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentCompleteDetails> for Any {
    fn from(s: &PaymentCompleteDetails) -> Any {
        s.inner.clone()
    }
}

impl PaymentCompleteDetails {
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
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
    pub fn payer(&self) -> Any {
        self.inner.get("payer").as_::<Any>()
    }

    pub fn set_payer(&mut self, value: &Any) {
        self.inner.set("payer", value);
    }
}
impl PaymentValidationErrors {
    pub fn shipping_address(&self) -> Any {
        self.inner.get("shippingAddress").as_::<Any>()
    }

    pub fn set_shipping_address(&mut self, value: &Any) {
        self.inner.set("shippingAddress", value);
    }
}
impl PaymentValidationErrors {
    pub fn error(&self) -> DOMString {
        self.inner.get("error").as_::<DOMString>()
    }

    pub fn set_error(&mut self, value: &DOMString) {
        self.inner.set("error", value);
    }
}
impl PaymentValidationErrors {
    pub fn payment_method(&self) -> Object {
        self.inner.get("paymentMethod").as_::<Object>()
    }

    pub fn set_payment_method(&mut self, value: &Object) {
        self.inner.set("paymentMethod", value);
    }
}
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
    pub fn request_id(&self) -> DOMString {
        self.inner.get("requestId").as_::<DOMString>()
    }
}
impl PaymentResponse {
    /// Getter of the `methodName` attribute.
    /// [`PaymentResponse.methodName`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/methodName)
    pub fn method_name(&self) -> DOMString {
        self.inner.get("methodName").as_::<DOMString>()
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
    pub fn shipping_option(&self) -> DOMString {
        self.inner.get("shippingOption").as_::<DOMString>()
    }
}
impl PaymentResponse {
    /// Getter of the `payerName` attribute.
    /// [`PaymentResponse.payerName`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerName)
    pub fn payer_name(&self) -> DOMString {
        self.inner.get("payerName").as_::<DOMString>()
    }
}
impl PaymentResponse {
    /// Getter of the `payerEmail` attribute.
    /// [`PaymentResponse.payerEmail`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerEmail)
    pub fn payer_email(&self) -> DOMString {
        self.inner.get("payerEmail").as_::<DOMString>()
    }
}
impl PaymentResponse {
    /// Getter of the `payerPhone` attribute.
    /// [`PaymentResponse.payerPhone`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerPhone)
    pub fn payer_phone(&self) -> DOMString {
        self.inner.get("payerPhone").as_::<DOMString>()
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
