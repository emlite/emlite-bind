use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for PaymentCompleteDetails {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaymentCompleteDetails {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&PaymentCompleteDetails> for emlite::Val {
    fn from(s: &PaymentCompleteDetails) -> emlite::Val {
        s.inner.clone()
    }
}

impl PaymentCompleteDetails {
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    pub fn set_data(&mut self, value: Object) {
        self.inner.set("data", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for PaymentValidationErrors {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaymentValidationErrors {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&PaymentValidationErrors> for emlite::Val {
    fn from(s: &PaymentValidationErrors) -> emlite::Val {
        s.inner.clone()
    }
}

impl PaymentValidationErrors {
    pub fn payer(&self) -> Any {
        self.inner.get("payer").as_::<Any>()
    }

    pub fn set_payer(&mut self, value: Any) {
        self.inner.set("payer", value);
    }
}
impl PaymentValidationErrors {
    pub fn shipping_address(&self) -> Any {
        self.inner.get("shippingAddress").as_::<Any>()
    }

    pub fn set_shipping_address(&mut self, value: Any) {
        self.inner.set("shippingAddress", value);
    }
}
impl PaymentValidationErrors {
    pub fn error(&self) -> DOMString {
        self.inner.get("error").as_::<DOMString>()
    }

    pub fn set_error(&mut self, value: DOMString) {
        self.inner.set("error", value);
    }
}
impl PaymentValidationErrors {
    pub fn payment_method(&self) -> Object {
        self.inner.get("paymentMethod").as_::<Object>()
    }

    pub fn set_payment_method(&mut self, value: Object) {
        self.inner.set("paymentMethod", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for PaymentResponse {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PaymentResponse {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&PaymentResponse> for emlite::Val {
    fn from(s: &PaymentResponse) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaymentResponse);

impl PaymentResponse {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl PaymentResponse {
    pub fn request_id(&self) -> DOMString {
        self.inner.get("requestId").as_::<DOMString>()
    }
}
impl PaymentResponse {
    pub fn method_name(&self) -> DOMString {
        self.inner.get("methodName").as_::<DOMString>()
    }
}
impl PaymentResponse {
    pub fn details(&self) -> Object {
        self.inner.get("details").as_::<Object>()
    }
}
impl PaymentResponse {
    pub fn shipping_address(&self) -> ContactAddress {
        self.inner.get("shippingAddress").as_::<ContactAddress>()
    }
}
impl PaymentResponse {
    pub fn shipping_option(&self) -> DOMString {
        self.inner.get("shippingOption").as_::<DOMString>()
    }
}
impl PaymentResponse {
    pub fn payer_name(&self) -> DOMString {
        self.inner.get("payerName").as_::<DOMString>()
    }
}
impl PaymentResponse {
    pub fn payer_email(&self) -> DOMString {
        self.inner.get("payerEmail").as_::<DOMString>()
    }
}
impl PaymentResponse {
    pub fn payer_phone(&self) -> DOMString {
        self.inner.get("payerPhone").as_::<DOMString>()
    }
}
impl PaymentResponse {
    pub fn complete0(&self) -> Promise {
        self.inner.call("complete", &[]).as_::<Promise>()
    }

    pub fn complete1(&self, result: PaymentComplete) -> Promise {
        self.inner
            .call("complete", &[result.into()])
            .as_::<Promise>()
    }

    pub fn complete2(&self, result: PaymentComplete, details: PaymentCompleteDetails) -> Promise {
        self.inner
            .call("complete", &[result.into(), details.into()])
            .as_::<Promise>()
    }
}
impl PaymentResponse {
    pub fn retry0(&self) -> Promise {
        self.inner.call("retry", &[]).as_::<Promise>()
    }

    pub fn retry1(&self, error_fields: PaymentValidationErrors) -> Promise {
        self.inner
            .call("retry", &[error_fields.into()])
            .as_::<Promise>()
    }
}
impl PaymentResponse {
    pub fn onpayerdetailchange(&self) -> Any {
        self.inner.get("onpayerdetailchange").as_::<Any>()
    }

    pub fn set_onpayerdetailchange(&mut self, value: Any) {
        self.inner.set("onpayerdetailchange", value);
    }
}
