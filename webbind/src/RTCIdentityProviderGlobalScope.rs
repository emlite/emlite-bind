use super::*;

/// The RTCIdentityProviderGlobalScope class.
/// [`RTCIdentityProviderGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityProviderGlobalScope {
    inner: WorkerGlobalScope,
}

impl FromVal for RTCIdentityProviderGlobalScope {
    fn from_val(v: &Any) -> Self {
        RTCIdentityProviderGlobalScope {
            inner: WorkerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for RTCIdentityProviderGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIdentityProviderGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCIdentityProviderGlobalScope> for Any {
    fn from(s: RTCIdentityProviderGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIdentityProviderGlobalScope> for Any {
    fn from(s: &RTCIdentityProviderGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCIdentityProviderGlobalScope);

impl RTCIdentityProviderGlobalScope {
    /// Getter of the `rtcIdentityProvider` attribute.
    /// [`RTCIdentityProviderGlobalScope.rtcIdentityProvider`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderGlobalScope/rtcIdentityProvider)
    pub fn rtc_identity_provider(&self) -> RTCIdentityProviderRegistrar {
        self.inner
            .get("rtcIdentityProvider")
            .as_::<RTCIdentityProviderRegistrar>()
    }
}
