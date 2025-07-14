use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticatorAssertionResponse {
    inner: AuthenticatorResponse,
}
impl FromVal for AuthenticatorAssertionResponse {
    fn from_val(v: &emlite::Val) -> Self {
        AuthenticatorAssertionResponse {
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
impl core::ops::Deref for AuthenticatorAssertionResponse {
    type Target = AuthenticatorResponse;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticatorAssertionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AuthenticatorAssertionResponse {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AuthenticatorAssertionResponse {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AuthenticatorAssertionResponse> for emlite::Val {
    fn from(s: AuthenticatorAssertionResponse) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AuthenticatorAssertionResponse);

impl AuthenticatorAssertionResponse {
    pub fn authenticator_data(&self) -> jsbind::ArrayBuffer {
        self.inner
            .get("authenticatorData")
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl AuthenticatorAssertionResponse {
    pub fn signature(&self) -> jsbind::ArrayBuffer {
        self.inner.get("signature").as_::<jsbind::ArrayBuffer>()
    }
}
impl AuthenticatorAssertionResponse {
    pub fn user_handle(&self) -> jsbind::ArrayBuffer {
        self.inner.get("userHandle").as_::<jsbind::ArrayBuffer>()
    }
}
