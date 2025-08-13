use super::*;




/// The AuthenticationExtensionsClientInputs dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsClientInputs {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsClientInputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsClientInputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsClientInputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsClientInputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsClientInputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsClientInputs {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuthenticationExtensionsClientInputs> for Any {
    fn from(s: AuthenticationExtensionsClientInputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsClientInputs> for Any {
    fn from(s: &AuthenticationExtensionsClientInputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsClientInputs {
    /// Getter of the `largeBlob` attribute.
    pub fn large_blob(&self) -> AuthenticationExtensionsLargeBlobInputs {
        self.inner.get("largeBlob").as_::<AuthenticationExtensionsLargeBlobInputs>()
    }

    /// Setter of the `largeBlob` attribute.
    pub fn set_large_blob(&mut self, value: &AuthenticationExtensionsLargeBlobInputs) {
        self.inner.set("largeBlob", value);
    }
}
