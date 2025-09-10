use super::*;

/// The RTCIdentityProviderRegistrar class.
/// [`RTCIdentityProviderRegistrar`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityProviderRegistrar {
    inner: Any,
}

impl FromVal for RTCIdentityProviderRegistrar {
    fn from_val(v: &Any) -> Self {
        RTCIdentityProviderRegistrar {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIdentityProviderRegistrar {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIdentityProviderRegistrar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIdentityProviderRegistrar {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIdentityProviderRegistrar {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCIdentityProviderRegistrar> for Any {
    fn from(s: RTCIdentityProviderRegistrar) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIdentityProviderRegistrar> for Any {
    fn from(s: &RTCIdentityProviderRegistrar) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCIdentityProviderRegistrar);

impl RTCIdentityProviderRegistrar {
    /// The register method.
    /// [`RTCIdentityProviderRegistrar.register`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar/register)
    pub fn register(&self, idp: &RTCIdentityProvider) -> Undefined {
        self.inner
            .call("register", &[idp.into()])
            .as_::<Undefined>()
    }
}
