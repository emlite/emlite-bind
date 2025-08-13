use super::*;




/// The AuthenticationExtensionsLargeBlobInputs dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsLargeBlobInputs {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsLargeBlobInputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsLargeBlobInputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsLargeBlobInputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsLargeBlobInputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsLargeBlobInputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsLargeBlobInputs {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuthenticationExtensionsLargeBlobInputs> for Any {
    fn from(s: AuthenticationExtensionsLargeBlobInputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsLargeBlobInputs> for Any {
    fn from(s: &AuthenticationExtensionsLargeBlobInputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsLargeBlobInputs {
    /// Getter of the `support` attribute.
    pub fn support(&self) -> JsString {
        self.inner.get("support").as_::<JsString>()
    }

    /// Setter of the `support` attribute.
    pub fn set_support(&mut self, value: &JsString) {
        self.inner.set("support", value);
    }
}
impl AuthenticationExtensionsLargeBlobInputs {
    /// Getter of the `read` attribute.
    pub fn read(&self) -> bool {
        self.inner.get("read").as_::<bool>()
    }

    /// Setter of the `read` attribute.
    pub fn set_read(&mut self, value: bool) {
        self.inner.set("read", value);
    }
}
impl AuthenticationExtensionsLargeBlobInputs {
    /// Getter of the `write` attribute.
    pub fn write(&self) -> Any {
        self.inner.get("write").as_::<Any>()
    }

    /// Setter of the `write` attribute.
    pub fn set_write(&mut self, value: &Any) {
        self.inner.set("write", value);
    }
}
