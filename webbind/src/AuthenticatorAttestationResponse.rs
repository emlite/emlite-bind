use super::*;

/// The AuthenticatorAttestationResponse class.
/// [`AuthenticatorAttestationResponse`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticatorAttestationResponse {
    inner: AuthenticatorResponse,
}
impl FromVal for AuthenticatorAttestationResponse {
    fn from_val(v: &Any) -> Self {
        AuthenticatorAttestationResponse {
            inner: AuthenticatorResponse::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuthenticatorAttestationResponse {
    type Target = AuthenticatorResponse;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticatorAttestationResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuthenticatorAttestationResponse {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuthenticatorAttestationResponse {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuthenticatorAttestationResponse> for Any {
    fn from(s: AuthenticatorAttestationResponse) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuthenticatorAttestationResponse> for Any {
    fn from(s: &AuthenticatorAttestationResponse) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AuthenticatorAttestationResponse);

impl AuthenticatorAttestationResponse {
    /// Getter of the `attestationObject` attribute.
    /// [`AuthenticatorAttestationResponse.attestationObject`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/attestationObject)
    pub fn attestation_object(&self) -> ArrayBuffer {
        self.inner.get("attestationObject").as_::<ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    /// The getTransports method.
    /// [`AuthenticatorAttestationResponse.getTransports`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getTransports)
    pub fn get_transports(&self) -> Sequence<DOMString> {
        self.inner
            .call("getTransports", &[])
            .as_::<Sequence<DOMString>>()
    }
}
impl AuthenticatorAttestationResponse {
    /// The getAuthenticatorData method.
    /// [`AuthenticatorAttestationResponse.getAuthenticatorData`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getAuthenticatorData)
    pub fn get_authenticator_data(&self) -> ArrayBuffer {
        self.inner
            .call("getAuthenticatorData", &[])
            .as_::<ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    /// The getPublicKey method.
    /// [`AuthenticatorAttestationResponse.getPublicKey`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getPublicKey)
    pub fn get_public_key(&self) -> ArrayBuffer {
        self.inner.call("getPublicKey", &[]).as_::<ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    /// The getPublicKeyAlgorithm method.
    /// [`AuthenticatorAttestationResponse.getPublicKeyAlgorithm`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getPublicKeyAlgorithm)
    pub fn get_public_key_algorithm(&self) -> Any {
        self.inner.call("getPublicKeyAlgorithm", &[]).as_::<Any>()
    }
}
