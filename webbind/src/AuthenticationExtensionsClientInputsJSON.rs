use super::*;

/// The AuthenticationExtensionsClientInputsJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsClientInputsJSON {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsClientInputsJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsClientInputsJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsClientInputsJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsClientInputsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsClientInputsJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsClientInputsJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsClientInputsJSON> for Any {
    fn from(s: AuthenticationExtensionsClientInputsJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsClientInputsJSON> for Any {
    fn from(s: &AuthenticationExtensionsClientInputsJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsClientInputsJSON {
    /// Getter of the `largeBlob` attribute.
    pub fn large_blob(&self) -> AuthenticationExtensionsLargeBlobInputsJSON {
        self.inner
            .get("largeBlob")
            .as_::<AuthenticationExtensionsLargeBlobInputsJSON>()
    }

    /// Setter of the `largeBlob` attribute.
    pub fn set_large_blob(&mut self, value: &AuthenticationExtensionsLargeBlobInputsJSON) {
        self.inner.set("largeBlob", value);
    }
}
