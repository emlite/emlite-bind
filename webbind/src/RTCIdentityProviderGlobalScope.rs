use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for RTCIdentityProviderGlobalScope {
    type Target = WorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIdentityProviderGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIdentityProviderGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIdentityProviderGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCIdentityProviderGlobalScope> for emlite::Val {
    fn from(s: RTCIdentityProviderGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCIdentityProviderGlobalScope> for emlite::Val {
    fn from(s: &RTCIdentityProviderGlobalScope) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCIdentityProviderGlobalScope);

impl RTCIdentityProviderGlobalScope {
    pub fn rtc_identity_provider(&self) -> RTCIdentityProviderRegistrar {
        self.inner
            .get("rtcIdentityProvider")
            .as_::<RTCIdentityProviderRegistrar>()
    }
}
