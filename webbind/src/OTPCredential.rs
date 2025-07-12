use super::*;

#[derive(Clone, Debug)]
pub struct OTPCredential {
    inner: Credential,
}
impl FromVal for OTPCredential {
    fn from_val(v: &emlite::Val) -> Self {
        OTPCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OTPCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OTPCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OTPCredential> for emlite::Val {
    fn from(s: OTPCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OTPCredential {
    pub fn code(&self) -> jsbind::DOMString {
        self.inner.get("code").as_::<jsbind::DOMString>()
    }
}
