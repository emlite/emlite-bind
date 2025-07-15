use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for AuthenticatorAttestationResponse {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AuthenticatorAttestationResponse {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&AuthenticatorAttestationResponse> for emlite::Val {
    fn from(s: &AuthenticatorAttestationResponse) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AuthenticatorAttestationResponse);

impl AuthenticatorAttestationResponse {
    pub fn attestation_object(&self) -> ArrayBuffer {
        self.inner.get("attestationObject").as_::<ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_transports(&self) -> Sequence<DOMString> {
        self.inner
            .call("getTransports", &[])
            .as_::<Sequence<DOMString>>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_authenticator_data(&self) -> ArrayBuffer {
        self.inner
            .call("getAuthenticatorData", &[])
            .as_::<ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_public_key(&self) -> ArrayBuffer {
        self.inner.call("getPublicKey", &[]).as_::<ArrayBuffer>()
    }
}
impl AuthenticatorAttestationResponse {
    pub fn get_public_key_algorithm(&self) -> Any {
        self.inner.call("getPublicKeyAlgorithm", &[]).as_::<Any>()
    }
}
