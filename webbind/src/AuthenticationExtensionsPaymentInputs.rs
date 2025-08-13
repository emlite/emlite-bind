use super::*;




/// The AuthenticationExtensionsPaymentInputs dictionary.
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
    /// Getter of the `isPayment` attribute.
    pub fn is_payment(&self) -> bool {
        self.inner.get("isPayment").as_::<bool>()
    }

    /// Setter of the `isPayment` attribute.
    pub fn set_is_payment(&mut self, value: bool) {
        self.inner.set("isPayment", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `browserBoundPubKeyCredParams` attribute.
    pub fn browser_bound_pub_key_cred_params(&self) -> TypedArray<PublicKeyCredentialParameters> {
        self.inner.get("browserBoundPubKeyCredParams").as_::<TypedArray<PublicKeyCredentialParameters>>()
    }

    /// Setter of the `browserBoundPubKeyCredParams` attribute.
    pub fn set_browser_bound_pub_key_cred_params(&mut self, value: &TypedArray<PublicKeyCredentialParameters>) {
        self.inner.set("browserBoundPubKeyCredParams", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `rpId` attribute.
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    /// Setter of the `rpId` attribute.
    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `topOrigin` attribute.
    pub fn top_origin(&self) -> JsString {
        self.inner.get("topOrigin").as_::<JsString>()
    }

    /// Setter of the `topOrigin` attribute.
    pub fn set_top_origin(&mut self, value: &JsString) {
        self.inner.set("topOrigin", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `payeeName` attribute.
    pub fn payee_name(&self) -> JsString {
        self.inner.get("payeeName").as_::<JsString>()
    }

    /// Setter of the `payeeName` attribute.
    pub fn set_payee_name(&mut self, value: &JsString) {
        self.inner.set("payeeName", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `payeeOrigin` attribute.
    pub fn payee_origin(&self) -> JsString {
        self.inner.get("payeeOrigin").as_::<JsString>()
    }

    /// Setter of the `payeeOrigin` attribute.
    pub fn set_payee_origin(&mut self, value: &JsString) {
        self.inner.set("payeeOrigin", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `paymentEntitiesLogos` attribute.
    pub fn payment_entities_logos(&self) -> TypedArray<PaymentEntityLogo> {
        self.inner.get("paymentEntitiesLogos").as_::<TypedArray<PaymentEntityLogo>>()
    }

    /// Setter of the `paymentEntitiesLogos` attribute.
    pub fn set_payment_entities_logos(&mut self, value: &TypedArray<PaymentEntityLogo>) {
        self.inner.set("paymentEntitiesLogos", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `total` attribute.
    pub fn total(&self) -> PaymentCurrencyAmount {
        self.inner.get("total").as_::<PaymentCurrencyAmount>()
    }

    /// Setter of the `total` attribute.
    pub fn set_total(&mut self, value: &PaymentCurrencyAmount) {
        self.inner.set("total", value);
    }
}
impl AuthenticationExtensionsPaymentInputs {
    /// Getter of the `instrument` attribute.
    pub fn instrument(&self) -> PaymentCredentialInstrument {
        self.inner.get("instrument").as_::<PaymentCredentialInstrument>()
    }

    /// Setter of the `instrument` attribute.
    pub fn set_instrument(&mut self, value: &PaymentCredentialInstrument) {
        self.inner.set("instrument", value);
    }
}
