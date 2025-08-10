use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredentialRequestOptions {
    inner: Any,
}
impl FromVal for IdentityCredentialRequestOptions {
    fn from_val(v: &Any) -> Self {
        IdentityCredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityCredentialRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityCredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityCredentialRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityCredentialRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityCredentialRequestOptions> for Any {
    fn from(s: IdentityCredentialRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityCredentialRequestOptions> for Any {
    fn from(s: &IdentityCredentialRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl IdentityCredentialRequestOptions {
    pub fn providers(&self) -> TypedArray<IdentityProviderRequestOptions> {
        self.inner
            .get("providers")
            .as_::<TypedArray<IdentityProviderRequestOptions>>()
    }

    pub fn set_providers(&mut self, value: &TypedArray<IdentityProviderRequestOptions>) {
        self.inner.set("providers", value);
    }
}
impl IdentityCredentialRequestOptions {
    pub fn context(&self) -> IdentityCredentialRequestOptionsContext {
        self.inner
            .get("context")
            .as_::<IdentityCredentialRequestOptionsContext>()
    }

    pub fn set_context(&mut self, value: &IdentityCredentialRequestOptionsContext) {
        self.inner.set("context", value);
    }
}
impl IdentityCredentialRequestOptions {
    pub fn mode(&self) -> IdentityCredentialRequestOptionsMode {
        self.inner
            .get("mode")
            .as_::<IdentityCredentialRequestOptionsMode>()
    }

    pub fn set_mode(&mut self, value: &IdentityCredentialRequestOptionsMode) {
        self.inner.set("mode", value);
    }
}
