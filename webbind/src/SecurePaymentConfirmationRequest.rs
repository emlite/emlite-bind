use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SecurePaymentConfirmationRequest {
    inner: Any,
}
impl FromVal for SecurePaymentConfirmationRequest {
    fn from_val(v: &Any) -> Self {
        SecurePaymentConfirmationRequest { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SecurePaymentConfirmationRequest {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SecurePaymentConfirmationRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SecurePaymentConfirmationRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SecurePaymentConfirmationRequest {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SecurePaymentConfirmationRequest> for Any {
    fn from(s: SecurePaymentConfirmationRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SecurePaymentConfirmationRequest> for Any {
    fn from(s: &SecurePaymentConfirmationRequest) -> Any {
        s.inner.clone()
    }
}

impl SecurePaymentConfirmationRequest {
    pub fn challenge(&self) -> Any {
        self.inner.get("challenge").as_::<Any>()
    }

    pub fn set_challenge(&mut self, value: &Any) {
        self.inner.set("challenge", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn credential_ids(&self) -> TypedArray<Any> {
        self.inner.get("credentialIds").as_::<TypedArray<Any>>()
    }

    pub fn set_credential_ids(&mut self, value: &TypedArray<Any>) {
        self.inner.set("credentialIds", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn instrument(&self) -> PaymentCredentialInstrument {
        self.inner
            .get("instrument")
            .as_::<PaymentCredentialInstrument>()
    }

    pub fn set_instrument(&mut self, value: &PaymentCredentialInstrument) {
        self.inner.set("instrument", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn payee_name(&self) -> JsString {
        self.inner.get("payeeName").as_::<JsString>()
    }

    pub fn set_payee_name(&mut self, value: &JsString) {
        self.inner.set("payeeName", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn payee_origin(&self) -> JsString {
        self.inner.get("payeeOrigin").as_::<JsString>()
    }

    pub fn set_payee_origin(&mut self, value: &JsString) {
        self.inner.set("payeeOrigin", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn payment_entities_logos(&self) -> TypedArray<PaymentEntityLogo> {
        self.inner
            .get("paymentEntitiesLogos")
            .as_::<TypedArray<PaymentEntityLogo>>()
    }

    pub fn set_payment_entities_logos(&mut self, value: &TypedArray<PaymentEntityLogo>) {
        self.inner.set("paymentEntitiesLogos", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn extensions(&self) -> AuthenticationExtensionsClientInputs {
        self.inner
            .get("extensions")
            .as_::<AuthenticationExtensionsClientInputs>()
    }

    pub fn set_extensions(&mut self, value: &AuthenticationExtensionsClientInputs) {
        self.inner.set("extensions", value);
    }
}
impl SecurePaymentConfirmationRequest {
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
impl SecurePaymentConfirmationRequest {
    pub fn locale(&self) -> TypedArray<JsString> {
        self.inner.get("locale").as_::<TypedArray<JsString>>()
    }

    pub fn set_locale(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("locale", value);
    }
}
impl SecurePaymentConfirmationRequest {
    pub fn show_opt_out(&self) -> bool {
        self.inner.get("showOptOut").as_::<bool>()
    }

    pub fn set_show_opt_out(&mut self, value: bool) {
        self.inner.set("showOptOut", value);
    }
}
