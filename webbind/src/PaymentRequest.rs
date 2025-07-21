use super::*;

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
    pub fn error(&self) -> String {
        self.inner.get("error").as_::<String>()
    }

    pub fn set_error(&mut self, value: &str) {
        self.inner.set("error", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn total(&self) -> Any {
        self.inner.get("total").as_::<Any>()
    }

    pub fn set_total(&mut self, value: &Any) {
        self.inner.set("total", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn shipping_address_errors(&self) -> Any {
        self.inner.get("shippingAddressErrors").as_::<Any>()
    }

    pub fn set_shipping_address_errors(&mut self, value: &Any) {
        self.inner.set("shippingAddressErrors", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn payer_errors(&self) -> Any {
        self.inner.get("payerErrors").as_::<Any>()
    }

    pub fn set_payer_errors(&mut self, value: &Any) {
        self.inner.set("payerErrors", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn payment_method_errors(&self) -> Object {
        self.inner.get("paymentMethodErrors").as_::<Object>()
    }

    pub fn set_payment_method_errors(&mut self, value: &Object) {
        self.inner.set("paymentMethodErrors", value);
    }
}
/// The PaymentRequest class.
/// [`PaymentRequest`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequest {
    inner: EventTarget,
}
impl FromVal for PaymentRequest {
    fn from_val(v: &Any) -> Self {
        PaymentRequest {
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
impl core::ops::Deref for PaymentRequest {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentRequest {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentRequest> for Any {
    fn from(s: PaymentRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentRequest> for Any {
    fn from(s: &PaymentRequest) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaymentRequest);

impl PaymentRequest {
    /// The `new PaymentRequest(..)` constructor, creating a new PaymentRequest instance
    pub fn new0(method_data: &Sequence<PaymentMethodData>, details: &Any) -> PaymentRequest {
        Self {
            inner: Any::global("PaymentRequest")
                .new(&[method_data.into(), details.into()])
                .as_::<EventTarget>(),
        }
    }

    /// The `new PaymentRequest(..)` constructor, creating a new PaymentRequest instance
    pub fn new1(
        method_data: &Sequence<PaymentMethodData>,
        details: &Any,
        options: &Any,
    ) -> PaymentRequest {
        Self {
            inner: Any::global("PaymentRequest")
                .new(&[method_data.into(), details.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl PaymentRequest {
    /// The show method.
    /// [`PaymentRequest.show`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/show)
    pub fn show0(&self) -> Promise<PaymentResponse> {
        self.inner
            .call("show", &[])
            .as_::<Promise<PaymentResponse>>()
    }
    /// The show method.
    /// [`PaymentRequest.show`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/show)
    pub fn show1(
        &self,
        details_promise: &Promise<PaymentDetailsUpdate>,
    ) -> Promise<PaymentResponse> {
        self.inner
            .call("show", &[details_promise.into()])
            .as_::<Promise<PaymentResponse>>()
    }
}
impl PaymentRequest {
    /// The abort method.
    /// [`PaymentRequest.abort`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/abort)
    pub fn abort(&self) -> Promise<Undefined> {
        self.inner.call("abort", &[]).as_::<Promise<Undefined>>()
    }
}
impl PaymentRequest {
    /// The canMakePayment method.
    /// [`PaymentRequest.canMakePayment`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/canMakePayment)
    pub fn can_make_payment(&self) -> Promise<bool> {
        self.inner
            .call("canMakePayment", &[])
            .as_::<Promise<bool>>()
    }
}
impl PaymentRequest {
    /// Getter of the `id` attribute.
    /// [`PaymentRequest.id`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/id)
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl PaymentRequest {
    /// Getter of the `shippingAddress` attribute.
    /// [`PaymentRequest.shippingAddress`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/shippingAddress)
    pub fn shipping_address(&self) -> ContactAddress {
        self.inner.get("shippingAddress").as_::<ContactAddress>()
    }
}
impl PaymentRequest {
    /// Getter of the `shippingOption` attribute.
    /// [`PaymentRequest.shippingOption`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/shippingOption)
    pub fn shipping_option(&self) -> String {
        self.inner.get("shippingOption").as_::<String>()
    }
}
impl PaymentRequest {
    /// Getter of the `shippingType` attribute.
    /// [`PaymentRequest.shippingType`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/shippingType)
    pub fn shipping_type(&self) -> PaymentShippingType {
        self.inner.get("shippingType").as_::<PaymentShippingType>()
    }
}
impl PaymentRequest {
    /// Getter of the `onshippingaddresschange` attribute.
    /// [`PaymentRequest.onshippingaddresschange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/onshippingaddresschange)
    pub fn onshippingaddresschange(&self) -> Any {
        self.inner.get("onshippingaddresschange").as_::<Any>()
    }

    /// Setter of the `onshippingaddresschange` attribute.
    /// [`PaymentRequest.onshippingaddresschange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/onshippingaddresschange)
    pub fn set_onshippingaddresschange(&mut self, value: &Any) {
        self.inner.set("onshippingaddresschange", value);
    }
}
impl PaymentRequest {
    /// Getter of the `onshippingoptionchange` attribute.
    /// [`PaymentRequest.onshippingoptionchange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/onshippingoptionchange)
    pub fn onshippingoptionchange(&self) -> Any {
        self.inner.get("onshippingoptionchange").as_::<Any>()
    }

    /// Setter of the `onshippingoptionchange` attribute.
    /// [`PaymentRequest.onshippingoptionchange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/onshippingoptionchange)
    pub fn set_onshippingoptionchange(&mut self, value: &Any) {
        self.inner.set("onshippingoptionchange", value);
    }
}
impl PaymentRequest {
    /// Getter of the `onpaymentmethodchange` attribute.
    /// [`PaymentRequest.onpaymentmethodchange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/onpaymentmethodchange)
    pub fn onpaymentmethodchange(&self) -> Any {
        self.inner.get("onpaymentmethodchange").as_::<Any>()
    }

    /// Setter of the `onpaymentmethodchange` attribute.
    /// [`PaymentRequest.onpaymentmethodchange`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/onpaymentmethodchange)
    pub fn set_onpaymentmethodchange(&mut self, value: &Any) {
        self.inner.set("onpaymentmethodchange", value);
    }
}
impl PaymentRequest {
    /// The securePaymentConfirmationAvailability method.
    /// [`PaymentRequest.securePaymentConfirmationAvailability`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequest/securePaymentConfirmationAvailability)
    pub fn secure_payment_confirmation_availability()
    -> Promise<SecurePaymentConfirmationAvailability> {
        Any::global("PaymentRequest")
            .call("securePaymentConfirmationAvailability", &[])
            .as_::<Promise<SecurePaymentConfirmationAvailability>>()
    }
}
