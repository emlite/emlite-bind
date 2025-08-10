use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CollectedClientAdditionalPaymentData {
    inner: Any,
}
impl FromVal for CollectedClientAdditionalPaymentData {
    fn from_val(v: &Any) -> Self {
        CollectedClientAdditionalPaymentData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CollectedClientAdditionalPaymentData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CollectedClientAdditionalPaymentData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CollectedClientAdditionalPaymentData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CollectedClientAdditionalPaymentData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CollectedClientAdditionalPaymentData> for Any {
    fn from(s: CollectedClientAdditionalPaymentData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CollectedClientAdditionalPaymentData> for Any {
    fn from(s: &CollectedClientAdditionalPaymentData) -> Any {
        s.inner.clone()
    }
}

impl CollectedClientAdditionalPaymentData {
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl CollectedClientAdditionalPaymentData {
    pub fn top_origin(&self) -> JsString {
        self.inner.get("topOrigin").as_::<JsString>()
    }

    pub fn set_top_origin(&mut self, value: &JsString) {
        self.inner.set("topOrigin", value);
    }
}
impl CollectedClientAdditionalPaymentData {
    pub fn payee_name(&self) -> JsString {
        self.inner.get("payeeName").as_::<JsString>()
    }

    pub fn set_payee_name(&mut self, value: &JsString) {
        self.inner.set("payeeName", value);
    }
}
impl CollectedClientAdditionalPaymentData {
    pub fn payee_origin(&self) -> JsString {
        self.inner.get("payeeOrigin").as_::<JsString>()
    }

    pub fn set_payee_origin(&mut self, value: &JsString) {
        self.inner.set("payeeOrigin", value);
    }
}
impl CollectedClientAdditionalPaymentData {
    pub fn payment_entities_logos(&self) -> TypedArray<PaymentEntityLogo> {
        self.inner
            .get("paymentEntitiesLogos")
            .as_::<TypedArray<PaymentEntityLogo>>()
    }

    pub fn set_payment_entities_logos(&mut self, value: &TypedArray<PaymentEntityLogo>) {
        self.inner.set("paymentEntitiesLogos", value);
    }
}
impl CollectedClientAdditionalPaymentData {
    pub fn total(&self) -> PaymentCurrencyAmount {
        self.inner.get("total").as_::<PaymentCurrencyAmount>()
    }

    pub fn set_total(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("total", value);
    }
}
impl CollectedClientAdditionalPaymentData {
    pub fn instrument(&self) -> PaymentCredentialInstrument {
        self.inner
            .get("instrument")
            .as_::<PaymentCredentialInstrument>()
    }

    pub fn set_instrument(&mut self, value: &PaymentCredentialInstrument) {
        self.inner.set("instrument", value);
    }
}
impl CollectedClientAdditionalPaymentData {
    pub fn browser_bound_public_key(&self) -> JsString {
        self.inner.get("browserBoundPublicKey").as_::<JsString>()
    }

    pub fn set_browser_bound_public_key(&mut self, value: &JsString) {
        self.inner.set("browserBoundPublicKey", value);
    }
}
