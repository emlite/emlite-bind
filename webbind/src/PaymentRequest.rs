use super::*;

#[derive(Clone, Debug)]
pub struct PaymentDetailsUpdate {
    inner: emlite::Val,
}
impl FromVal for PaymentDetailsUpdate {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentDetailsUpdate { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PaymentDetailsUpdate {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaymentDetailsUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentDetailsUpdate> for emlite::Val {
    fn from(s: PaymentDetailsUpdate) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentDetailsUpdate {
    pub fn error(&self) -> jsbind::DOMString {
        self.inner.get("error").as_::<jsbind::DOMString>()
    }

    pub fn set_error(&mut self, value: jsbind::DOMString) {
        self.inner.set("error", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn total(&self) -> jsbind::Any {
        self.inner.get("total").as_::<jsbind::Any>()
    }

    pub fn set_total(&mut self, value: jsbind::Any) {
        self.inner.set("total", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn shipping_address_errors(&self) -> jsbind::Any {
        self.inner.get("shippingAddressErrors").as_::<jsbind::Any>()
    }

    pub fn set_shipping_address_errors(&mut self, value: jsbind::Any) {
        self.inner.set("shippingAddressErrors", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn payer_errors(&self) -> jsbind::Any {
        self.inner.get("payerErrors").as_::<jsbind::Any>()
    }

    pub fn set_payer_errors(&mut self, value: jsbind::Any) {
        self.inner.set("payerErrors", value);
    }
}
impl PaymentDetailsUpdate {
    pub fn payment_method_errors(&self) -> jsbind::Object {
        self.inner
            .get("paymentMethodErrors")
            .as_::<jsbind::Object>()
    }

    pub fn set_payment_method_errors(&mut self, value: jsbind::Object) {
        self.inner.set("paymentMethodErrors", value);
    }
}
#[derive(Clone, Debug)]
pub struct PaymentRequest {
    inner: EventTarget,
}
impl FromVal for PaymentRequest {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentRequest {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PaymentRequest {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaymentRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentRequest> for emlite::Val {
    fn from(s: PaymentRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentRequest {
    pub fn new0(
        method_data: jsbind::Sequence<PaymentMethodData>,
        details: jsbind::Any,
    ) -> PaymentRequest {
        Self {
            inner: emlite::Val::global("PaymentRequest")
                .new(&[method_data.into(), details.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(
        method_data: jsbind::Sequence<PaymentMethodData>,
        details: jsbind::Any,
        options: jsbind::Any,
    ) -> PaymentRequest {
        Self {
            inner: emlite::Val::global("PaymentRequest")
                .new(&[method_data.into(), details.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl PaymentRequest {
    pub fn show0(&self) -> jsbind::Promise {
        self.inner.call("show", &[]).as_::<jsbind::Promise>()
    }

    pub fn show1(&self, details_promise: jsbind::Promise) -> jsbind::Promise {
        self.inner
            .call("show", &[details_promise.into()])
            .as_::<jsbind::Promise>()
    }
}
impl PaymentRequest {
    pub fn abort(&self) -> jsbind::Promise {
        self.inner.call("abort", &[]).as_::<jsbind::Promise>()
    }
}
impl PaymentRequest {
    pub fn can_make_payment(&self) -> jsbind::Promise {
        self.inner
            .call("canMakePayment", &[])
            .as_::<jsbind::Promise>()
    }
}
impl PaymentRequest {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl PaymentRequest {
    pub fn shipping_address(&self) -> ContactAddress {
        self.inner.get("shippingAddress").as_::<ContactAddress>()
    }
}
impl PaymentRequest {
    pub fn shipping_option(&self) -> jsbind::DOMString {
        self.inner.get("shippingOption").as_::<jsbind::DOMString>()
    }
}
impl PaymentRequest {
    pub fn shipping_type(&self) -> PaymentShippingType {
        self.inner.get("shippingType").as_::<PaymentShippingType>()
    }
}
impl PaymentRequest {
    pub fn onshippingaddresschange(&self) -> jsbind::Any {
        self.inner
            .get("onshippingaddresschange")
            .as_::<jsbind::Any>()
    }

    pub fn set_onshippingaddresschange(&mut self, value: jsbind::Any) {
        self.inner.set("onshippingaddresschange", value);
    }
}
impl PaymentRequest {
    pub fn onshippingoptionchange(&self) -> jsbind::Any {
        self.inner
            .get("onshippingoptionchange")
            .as_::<jsbind::Any>()
    }

    pub fn set_onshippingoptionchange(&mut self, value: jsbind::Any) {
        self.inner.set("onshippingoptionchange", value);
    }
}
impl PaymentRequest {
    pub fn onpaymentmethodchange(&self) -> jsbind::Any {
        self.inner.get("onpaymentmethodchange").as_::<jsbind::Any>()
    }

    pub fn set_onpaymentmethodchange(&mut self, value: jsbind::Any) {
        self.inner.set("onpaymentmethodchange", value);
    }
}
impl PaymentRequest {
    pub fn secure_payment_confirmation_availability() -> jsbind::Promise {
        emlite::Val::global("paymentrequest")
            .call("securePaymentConfirmationAvailability", &[])
            .as_::<jsbind::Promise>()
    }
}
