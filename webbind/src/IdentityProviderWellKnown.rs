use super::*;

/// The IdentityProviderWellKnown dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderWellKnown {
    inner: Any,
}

impl FromVal for IdentityProviderWellKnown {
    fn from_val(v: &Any) -> Self {
        IdentityProviderWellKnown { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityProviderWellKnown {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityProviderWellKnown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityProviderWellKnown {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityProviderWellKnown {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IdentityProviderWellKnown> for Any {
    fn from(s: IdentityProviderWellKnown) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityProviderWellKnown> for Any {
    fn from(s: &IdentityProviderWellKnown) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderWellKnown {
    /// Getter of the `provider_urls` attribute.
    pub fn provider_urls(&self) -> TypedArray<JsString> {
        self.inner
            .get("provider_urls")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `provider_urls` attribute.
    pub fn set_provider_urls(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("provider_urls", value);
    }
}
impl IdentityProviderWellKnown {
    /// Getter of the `accounts_endpoint` attribute.
    pub fn accounts_endpoint(&self) -> JsString {
        self.inner.get("accounts_endpoint").as_::<JsString>()
    }

    /// Setter of the `accounts_endpoint` attribute.
    pub fn set_accounts_endpoint(&mut self, value: &JsString) {
        self.inner.set("accounts_endpoint", value);
    }
}
impl IdentityProviderWellKnown {
    /// Getter of the `login_url` attribute.
    pub fn login_url(&self) -> JsString {
        self.inner.get("login_url").as_::<JsString>()
    }

    /// Setter of the `login_url` attribute.
    pub fn set_login_url(&mut self, value: &JsString) {
        self.inner.set("login_url", value);
    }
}
