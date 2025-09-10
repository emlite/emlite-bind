use super::*;

/// The FederatedCredentialRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FederatedCredentialRequestOptions {
    inner: Any,
}

impl FromVal for FederatedCredentialRequestOptions {
    fn from_val(v: &Any) -> Self {
        FederatedCredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FederatedCredentialRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FederatedCredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FederatedCredentialRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FederatedCredentialRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FederatedCredentialRequestOptions> for Any {
    fn from(s: FederatedCredentialRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FederatedCredentialRequestOptions> for Any {
    fn from(s: &FederatedCredentialRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl FederatedCredentialRequestOptions {
    /// Getter of the `providers` attribute.
    pub fn providers(&self) -> TypedArray<JsString> {
        self.inner.get("providers").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `providers` attribute.
    pub fn set_providers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("providers", value);
    }
}
impl FederatedCredentialRequestOptions {
    /// Getter of the `protocols` attribute.
    pub fn protocols(&self) -> TypedArray<JsString> {
        self.inner.get("protocols").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `protocols` attribute.
    pub fn set_protocols(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("protocols", value);
    }
}
