use super::*;

#[derive(Clone, Debug)]
pub struct RTCIdentityProviderGlobalScope {
    inner: WorkerGlobalScope,
}
impl FromVal for RTCIdentityProviderGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIdentityProviderGlobalScope {
            inner: WorkerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIdentityProviderGlobalScope {
    type Target = WorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIdentityProviderGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIdentityProviderGlobalScope> for emlite::Val {
    fn from(s: RTCIdentityProviderGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIdentityProviderGlobalScope {
    pub fn rtc_identity_provider(&self) -> RTCIdentityProviderRegistrar {
        self.inner
            .get("rtcIdentityProvider")
            .as_::<RTCIdentityProviderRegistrar>()
    }
}
