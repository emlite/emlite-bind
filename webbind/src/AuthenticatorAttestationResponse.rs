use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AuthenticatorAttestationResponse {
    inner: AuthenticatorResponse,
}
impl FromVal for AuthenticatorAttestationResponse {
    fn from_val(v: &emlite::Val) -> Self {
        AuthenticatorAttestationResponse {
            inner: AuthenticatorResponse::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<AuthenticatorAttestationResponse> for emlite::Val {
    fn from(s: AuthenticatorAttestationResponse) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AuthenticatorAttestationResponse {
    pub fn attestation_object(&self) -> jsbind::ArrayBuffer {
        self.inner
            .get("attestationObject")
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_transports(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .call("getTransports", &[])
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_authenticator_data(&self) -> jsbind::ArrayBuffer {
        self.inner
            .call("getAuthenticatorData", &[])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_public_key(&self) -> jsbind::ArrayBuffer {
        self.inner
            .call("getPublicKey", &[])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_public_key_algorithm(&self) -> jsbind::Any {
        self.inner
            .call("getPublicKeyAlgorithm", &[])
            .as_::<jsbind::Any>()
    }
}
