use super::*;




/// The AuthenticatorAttestationResponseJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticatorAttestationResponseJSON {
    inner: Any,
}

impl FromVal for AuthenticatorAttestationResponseJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticatorAttestationResponseJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticatorAttestationResponseJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticatorAttestationResponseJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticatorAttestationResponseJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticatorAttestationResponseJSON {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuthenticatorAttestationResponseJSON> for Any {
    fn from(s: AuthenticatorAttestationResponseJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticatorAttestationResponseJSON> for Any {
    fn from(s: &AuthenticatorAttestationResponseJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticatorAttestationResponseJSON {
    /// Getter of the `clientDataJSON` attribute.
    pub fn client_data_json(&self) -> Any {
        self.inner.get("clientDataJSON").as_::<Any>()
    }

    /// Setter of the `clientDataJSON` attribute.
    pub fn set_client_data_json(&mut self, value: &Any) {
        self.inner.set("clientDataJSON", value);
    }
}
impl AuthenticatorAttestationResponseJSON {
    /// Getter of the `authenticatorData` attribute.
    pub fn authenticator_data(&self) -> Any {
        self.inner.get("authenticatorData").as_::<Any>()
    }

    /// Setter of the `authenticatorData` attribute.
    pub fn set_authenticator_data(&mut self, value: &Any) {
        self.inner.set("authenticatorData", value);
    }
}
impl AuthenticatorAttestationResponseJSON {
    /// Getter of the `transports` attribute.
    pub fn transports(&self) -> TypedArray<JsString> {
        self.inner.get("transports").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `transports` attribute.
    pub fn set_transports(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("transports", value);
    }
}
impl AuthenticatorAttestationResponseJSON {
    /// Getter of the `publicKey` attribute.
    pub fn public_key(&self) -> Any {
        self.inner.get("publicKey").as_::<Any>()
    }

    /// Setter of the `publicKey` attribute.
    pub fn set_public_key(&mut self, value: &Any) {
        self.inner.set("publicKey", value);
    }
}
impl AuthenticatorAttestationResponseJSON {
    /// Getter of the `publicKeyAlgorithm` attribute.
    pub fn public_key_algorithm(&self) -> Any {
        self.inner.get("publicKeyAlgorithm").as_::<Any>()
    }

    /// Setter of the `publicKeyAlgorithm` attribute.
    pub fn set_public_key_algorithm(&mut self, value: &Any) {
        self.inner.set("publicKeyAlgorithm", value);
    }
}
impl AuthenticatorAttestationResponseJSON {
    /// Getter of the `attestationObject` attribute.
    pub fn attestation_object(&self) -> Any {
        self.inner.get("attestationObject").as_::<Any>()
    }

    /// Setter of the `attestationObject` attribute.
    pub fn set_attestation_object(&mut self, value: &Any) {
        self.inner.set("attestationObject", value);
    }
}
