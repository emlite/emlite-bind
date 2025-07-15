use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for PaymentDetailsUpdate {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentDetailsUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PaymentDetailsUpdate {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaymentDetailsUpdate {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PaymentDetailsUpdate> for emlite::Val {
    fn from(s: PaymentDetailsUpdate) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PaymentDetailsUpdate> for emlite::Val {
    fn from(s: &PaymentDetailsUpdate) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for PaymentRequest {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaymentRequest {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PaymentRequest> for emlite::Val {
    fn from(s: PaymentRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PaymentRequest> for emlite::Val {
    fn from(s: &PaymentRequest) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaymentRequest);

impl PaymentRequest {
    pub fn new0(method_data: &Sequence<PaymentMethodData>, details: &Any) -> PaymentRequest {
        Self {
            inner: emlite::Val::global("PaymentRequest")
                .new(&[method_data.into(), details.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(
        method_data: &Sequence<PaymentMethodData>,
        details: &Any,
        options: &Any,
    ) -> PaymentRequest {
        Self {
            inner: emlite::Val::global("PaymentRequest")
                .new(&[method_data.into(), details.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl PaymentRequest {
    pub fn show0(&self) -> Promise {
        self.inner.call("show", &[]).as_::<Promise>()
    }

    pub fn show1(&self, details_promise: &Promise) -> Promise {
        self.inner
            .call("show", &[details_promise.into()])
            .as_::<Promise>()
    }
}
impl PaymentRequest {
    pub fn abort(&self) -> Promise {
        self.inner.call("abort", &[]).as_::<Promise>()
    }
}
impl PaymentRequest {
    pub fn can_make_payment(&self) -> Promise {
        self.inner.call("canMakePayment", &[]).as_::<Promise>()
    }
}
impl PaymentRequest {
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl PaymentRequest {
    pub fn shipping_address(&self) -> ContactAddress {
        self.inner.get("shippingAddress").as_::<ContactAddress>()
    }
}
impl PaymentRequest {
    pub fn shipping_option(&self) -> String {
        self.inner.get("shippingOption").as_::<String>()
    }
}
impl PaymentRequest {
    pub fn shipping_type(&self) -> PaymentShippingType {
        self.inner.get("shippingType").as_::<PaymentShippingType>()
    }
}
impl PaymentRequest {
    pub fn onshippingaddresschange(&self) -> Any {
        self.inner.get("onshippingaddresschange").as_::<Any>()
    }

    pub fn set_onshippingaddresschange(&mut self, value: &Any) {
        self.inner.set("onshippingaddresschange", value);
    }
}
impl PaymentRequest {
    pub fn onshippingoptionchange(&self) -> Any {
        self.inner.get("onshippingoptionchange").as_::<Any>()
    }

    pub fn set_onshippingoptionchange(&mut self, value: &Any) {
        self.inner.set("onshippingoptionchange", value);
    }
}
impl PaymentRequest {
    pub fn onpaymentmethodchange(&self) -> Any {
        self.inner.get("onpaymentmethodchange").as_::<Any>()
    }

    pub fn set_onpaymentmethodchange(&mut self, value: &Any) {
        self.inner.set("onpaymentmethodchange", value);
    }
}
impl PaymentRequest {
    pub fn secure_payment_confirmation_availability() -> Promise {
        emlite::Val::global("PaymentRequest")
            .call("securePaymentConfirmationAvailability", &[])
            .as_::<Promise>()
    }
}
