use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PaymentCompleteDetails {
    inner: emlite::Val,
}
impl FromVal for PaymentCompleteDetails {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentCompleteDetails { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentCompleteDetails {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentCompleteDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentCompleteDetails> for emlite::Val {
    fn from(s: PaymentCompleteDetails) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentCompleteDetails {
    pub fn data(&self) -> jsbind::Object {
        self.inner.get("data").as_::<jsbind::Object>()
    }

    pub fn set_data(&mut self, value: jsbind::Object) {
        self.inner.set("data", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PaymentValidationErrors {
    inner: emlite::Val,
}
impl FromVal for PaymentValidationErrors {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentValidationErrors { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentValidationErrors {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentValidationErrors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentValidationErrors> for emlite::Val {
    fn from(s: PaymentValidationErrors) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentValidationErrors {
    pub fn payer(&self) -> jsbind::Any {
        self.inner.get("payer").as_::<jsbind::Any>()
    }

    pub fn set_payer(&mut self, value: jsbind::Any) {
        self.inner.set("payer", value);
    }
}
impl PaymentValidationErrors {
    pub fn shipping_address(&self) -> jsbind::Any {
        self.inner.get("shippingAddress").as_::<jsbind::Any>()
    }

    pub fn set_shipping_address(&mut self, value: jsbind::Any) {
        self.inner.set("shippingAddress", value);
    }
}
impl PaymentValidationErrors {
    pub fn error(&self) -> jsbind::DOMString {
        self.inner.get("error").as_::<jsbind::DOMString>()
    }

    pub fn set_error(&mut self, value: jsbind::DOMString) {
        self.inner.set("error", value);
    }
}
impl PaymentValidationErrors {
    pub fn payment_method(&self) -> jsbind::Object {
        self.inner.get("paymentMethod").as_::<jsbind::Object>()
    }

    pub fn set_payment_method(&mut self, value: jsbind::Object) {
        self.inner.set("paymentMethod", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PaymentResponse {
    inner: EventTarget,
}
impl FromVal for PaymentResponse {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentResponse {
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
impl From<PaymentResponse> for emlite::Val {
    fn from(s: PaymentResponse) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentResponse {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl PaymentResponse {
    pub fn request_id(&self) -> jsbind::DOMString {
        self.inner.get("requestId").as_::<jsbind::DOMString>()
    }
}
impl PaymentResponse {
    pub fn method_name(&self) -> jsbind::DOMString {
        self.inner.get("methodName").as_::<jsbind::DOMString>()
    }
}
impl PaymentResponse {
    pub fn details(&self) -> jsbind::Object {
        self.inner.get("details").as_::<jsbind::Object>()
    }
}
impl PaymentResponse {
    pub fn shipping_address(&self) -> ContactAddress {
        self.inner.get("shippingAddress").as_::<ContactAddress>()
    }
}
impl PaymentResponse {
    pub fn shipping_option(&self) -> jsbind::DOMString {
        self.inner.get("shippingOption").as_::<jsbind::DOMString>()
    }
}
impl PaymentResponse {
    pub fn payer_name(&self) -> jsbind::DOMString {
        self.inner.get("payerName").as_::<jsbind::DOMString>()
    }
}
impl PaymentResponse {
    pub fn payer_email(&self) -> jsbind::DOMString {
        self.inner.get("payerEmail").as_::<jsbind::DOMString>()
    }
}
impl PaymentResponse {
    pub fn payer_phone(&self) -> jsbind::DOMString {
        self.inner.get("payerPhone").as_::<jsbind::DOMString>()
    }
}
impl PaymentResponse {
    pub fn complete0(&self) -> jsbind::Promise {
        self.inner.call("complete", &[]).as_::<jsbind::Promise>()
    }

    pub fn complete1(&self, result: PaymentComplete) -> jsbind::Promise {
        self.inner
            .call("complete", &[result.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn complete2(
        &self,
        result: PaymentComplete,
        details: PaymentCompleteDetails,
    ) -> jsbind::Promise {
        self.inner
            .call("complete", &[result.into(), details.into()])
            .as_::<jsbind::Promise>()
    }
}
impl PaymentResponse {
    pub fn retry0(&self) -> jsbind::Promise {
        self.inner.call("retry", &[]).as_::<jsbind::Promise>()
    }

    pub fn retry1(&self, error_fields: PaymentValidationErrors) -> jsbind::Promise {
        self.inner
            .call("retry", &[error_fields.into()])
            .as_::<jsbind::Promise>()
    }
}
impl PaymentResponse {
    pub fn onpayerdetailchange(&self) -> jsbind::Any {
        self.inner.get("onpayerdetailchange").as_::<jsbind::Any>()
    }

    pub fn set_onpayerdetailchange(&mut self, value: jsbind::Any) {
        self.inner.set("onpayerdetailchange", value);
    }
}
