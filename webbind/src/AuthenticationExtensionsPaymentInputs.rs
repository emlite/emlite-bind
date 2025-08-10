use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPaymentInputs {
    inner: Any,
}
impl FromVal for AuthenticationExtensionsPaymentInputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPaymentInputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuthenticationExtensionsPaymentInputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticationExtensionsPaymentInputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuthenticationExtensionsPaymentInputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuthenticationExtensionsPaymentInputs {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuthenticationExtensionsPaymentInputs> for Any {
    fn from(s: AuthenticationExtensionsPaymentInputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuthenticationExtensionsPaymentInputs> for Any {
    fn from(s: &AuthenticationExtensionsPaymentInputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPaymentInputs {
    pub fn is_payment(&self) -> bool {
        self.inner.get("isPayment").as_::<bool>()
    }

    pub fn set_is_payment(&mut self, value: bool) {
        self.inner.set("isPayment", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn browser_bound_pub_key_cred_params(&self) -> TypedArray<PublicKeyCredentialParameters> {
        self.inner
            .get("browserBoundPubKeyCredParams")
            .as_::<TypedArray<PublicKeyCredentialParameters>>()
    }

    pub fn set_browser_bound_pub_key_cred_params(
        &mut self,
        value: &TypedArray<PublicKeyCredentialParameters>,
    ) {
        self.inner.set("browserBoundPubKeyCredParams", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn top_origin(&self) -> JsString {
        self.inner.get("topOrigin").as_::<JsString>()
    }

    pub fn set_top_origin(&mut self, value: &JsString) {
        self.inner.set("topOrigin", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn payee_name(&self) -> JsString {
        self.inner.get("payeeName").as_::<JsString>()
    }

    pub fn set_payee_name(&mut self, value: &JsString) {
        self.inner.set("payeeName", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn payee_origin(&self) -> JsString {
        self.inner.get("payeeOrigin").as_::<JsString>()
    }

    pub fn set_payee_origin(&mut self, value: &JsString) {
        self.inner.set("payeeOrigin", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn payment_entities_logos(&self) -> TypedArray<PaymentEntityLogo> {
        self.inner
            .get("paymentEntitiesLogos")
            .as_::<TypedArray<PaymentEntityLogo>>()
    }

    pub fn set_payment_entities_logos(&mut self, value: &TypedArray<PaymentEntityLogo>) {
        self.inner.set("paymentEntitiesLogos", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn total(&self) -> PaymentCurrencyAmount {
        self.inner.get("total").as_::<PaymentCurrencyAmount>()
    }

    pub fn set_total(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("total", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    pub fn instrument(&self) -> PaymentCredentialInstrument {
        self.inner
            .get("instrument")
            .as_::<PaymentCredentialInstrument>()
    }

    pub fn set_instrument(&mut self, value: &PaymentCredentialInstrument) {
        self.inner.set("instrument", value);
    }
}
