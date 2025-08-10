use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderRequestOptions {
    inner: Any,
}
impl FromVal for IdentityProviderRequestOptions {
    fn from_val(v: &Any) -> Self {
        IdentityProviderRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProviderRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProviderRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityProviderRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityProviderRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityProviderRequestOptions> for Any {
    fn from(s: IdentityProviderRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityProviderRequestOptions> for Any {
    fn from(s: &IdentityProviderRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderRequestOptions {
    pub fn nonce(&self) -> JsString {
        self.inner.get("nonce").as_::<JsString>()
    }

    pub fn set_nonce(&mut self, value: &JsString) {
        self.inner.set("nonce", value);
    }
}
impl IdentityProviderRequestOptions {
    pub fn login_hint(&self) -> JsString {
        self.inner.get("loginHint").as_::<JsString>()
    }

    pub fn set_login_hint(&mut self, value: &JsString) {
        self.inner.set("loginHint", value);
    }
}
impl IdentityProviderRequestOptions {
    pub fn domain_hint(&self) -> JsString {
        self.inner.get("domainHint").as_::<JsString>()
    }

    pub fn set_domain_hint(&mut self, value: &JsString) {
        self.inner.set("domainHint", value);
    }
}
impl IdentityProviderRequestOptions {
    pub fn fields(&self) -> TypedArray<JsString> {
        self.inner.get("fields").as_::<TypedArray<JsString>>()
    }

    pub fn set_fields(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("fields", value);
    }
}
impl IdentityProviderRequestOptions {
    pub fn params(&self) -> Any {
        self.inner.get("params").as_::<Any>()
    }

    pub fn set_params(&mut self, value: &Any) {
        self.inner.set("params", value);
    }
}
