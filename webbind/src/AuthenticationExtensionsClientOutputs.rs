use super::*;




/// The AuthenticationExtensionsClientOutputs dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsClientOutputs {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsClientOutputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsClientOutputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsClientOutputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsClientOutputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsClientOutputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsClientOutputs {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuthenticationExtensionsClientOutputs> for Any {
    fn from(s: AuthenticationExtensionsClientOutputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsClientOutputs> for Any {
    fn from(s: &AuthenticationExtensionsClientOutputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsClientOutputs {
    /// Getter of the `largeBlob` attribute.
    pub fn large_blob(&self) -> AuthenticationExtensionsLargeBlobOutputs {
        self.inner.get("largeBlob").as_::<AuthenticationExtensionsLargeBlobOutputs>()
    }

    /// Setter of the `largeBlob` attribute.
    pub fn set_large_blob(&mut self, value: &AuthenticationExtensionsLargeBlobOutputs) {
        self.inner.set("largeBlob", value);
    }
}
