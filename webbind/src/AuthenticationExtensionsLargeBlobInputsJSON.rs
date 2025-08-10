use super::*;

/// The AuthenticationExtensionsLargeBlobInputsJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsLargeBlobInputsJSON {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsLargeBlobInputsJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsLargeBlobInputsJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsLargeBlobInputsJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsLargeBlobInputsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsLargeBlobInputsJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsLargeBlobInputsJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsLargeBlobInputsJSON> for Any {
    fn from(s: AuthenticationExtensionsLargeBlobInputsJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsLargeBlobInputsJSON> for Any {
    fn from(s: &AuthenticationExtensionsLargeBlobInputsJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsLargeBlobInputsJSON {
    /// Getter of the `support` attribute.
    pub fn support(&self) -> JsString {
        self.inner.get("support").as_::<JsString>()
    }

    /// Setter of the `support` attribute.
    pub fn set_support(&mut self, value: &JsString) {
        self.inner.set("support", value);
    }
}
impl AuthenticationExtensionsLargeBlobInputsJSON {
    /// Getter of the `read` attribute.
    pub fn read(&self) -> bool {
        self.inner.get("read").as_::<bool>()
    }

    /// Setter of the `read` attribute.
    pub fn set_read(&mut self, value: bool) {
        self.inner.set("read", value);
    }
}
impl AuthenticationExtensionsLargeBlobInputsJSON {
    /// Getter of the `write` attribute.
    pub fn write(&self) -> Any {
        self.inner.get("write").as_::<Any>()
    }

    /// Setter of the `write` attribute.
    pub fn set_write(&mut self, value: &Any) {
        self.inner.set("write", value);
    }
}
