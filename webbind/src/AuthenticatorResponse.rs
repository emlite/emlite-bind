use super::*;

#[derive(Clone, Debug)]
pub struct AuthenticatorResponse {
    inner: emlite::Val,
}
impl FromVal for AuthenticatorResponse {
    fn from_val(v: &emlite::Val) -> Self {
        AuthenticatorResponse {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AuthenticatorResponse {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AuthenticatorResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AuthenticatorResponse> for emlite::Val {
    fn from(s: AuthenticatorResponse) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AuthenticatorResponse {
    pub fn client_data_json(&self) -> jsbind::ArrayBuffer {
        self.inner
            .get("clientDataJSON")
            .as_::<jsbind::ArrayBuffer>()
    }
}
