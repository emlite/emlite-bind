use super::*;

#[derive(Clone, Debug)]
pub struct RTCIdentityProvider {
    inner: emlite::Val,
}
impl FromVal for RTCIdentityProvider {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIdentityProvider { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIdentityProvider {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIdentityProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIdentityProvider> for emlite::Val {
    fn from(s: RTCIdentityProvider) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIdentityProvider {
    pub fn generate_assertion(&self) -> jsbind::Function {
        self.inner
            .get("generateAssertion")
            .as_::<jsbind::Function>()
    }

    pub fn set_generate_assertion(&mut self, value: jsbind::Function) {
        self.inner.set("generateAssertion", value);
    }
}
impl RTCIdentityProvider {
    pub fn validate_assertion(&self) -> jsbind::Function {
        self.inner
            .get("validateAssertion")
            .as_::<jsbind::Function>()
    }

    pub fn set_validate_assertion(&mut self, value: jsbind::Function) {
        self.inner.set("validateAssertion", value);
    }
}
#[derive(Clone, Debug)]
pub struct RTCIdentityProviderRegistrar {
    inner: emlite::Val,
}
impl FromVal for RTCIdentityProviderRegistrar {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIdentityProviderRegistrar {
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
impl std::ops::Deref for RTCIdentityProviderRegistrar {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIdentityProviderRegistrar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIdentityProviderRegistrar> for emlite::Val {
    fn from(s: RTCIdentityProviderRegistrar) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIdentityProviderRegistrar {
    pub fn register(&self, idp: RTCIdentityProvider) -> jsbind::Undefined {
        self.inner
            .call("register", &[idp.into()])
            .as_::<jsbind::Undefined>()
    }
}
