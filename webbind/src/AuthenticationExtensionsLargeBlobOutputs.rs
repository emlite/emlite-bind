use super::*;

/// The AuthenticationExtensionsLargeBlobOutputs dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsLargeBlobOutputs {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsLargeBlobOutputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsLargeBlobOutputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsLargeBlobOutputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsLargeBlobOutputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsLargeBlobOutputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsLargeBlobOutputs {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsLargeBlobOutputs> for Any {
    fn from(s: AuthenticationExtensionsLargeBlobOutputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsLargeBlobOutputs> for Any {
    fn from(s: &AuthenticationExtensionsLargeBlobOutputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsLargeBlobOutputs {
    /// Getter of the `supported` attribute.
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    /// Setter of the `supported` attribute.
    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl AuthenticationExtensionsLargeBlobOutputs {
    /// Getter of the `blob` attribute.
    pub fn blob(&self) -> ArrayBuffer {
        self.inner.get("blob").as_::<ArrayBuffer>()
    }

    /// Setter of the `blob` attribute.
    pub fn set_blob(&mut self, value: &ArrayBuffer) {
        self.inner.set("blob", value);
    }
}
impl AuthenticationExtensionsLargeBlobOutputs {
    /// Getter of the `written` attribute.
    pub fn written(&self) -> bool {
        self.inner.get("written").as_::<bool>()
    }

    /// Setter of the `written` attribute.
    pub fn set_written(&mut self, value: bool) {
        self.inner.set("written", value);
    }
}
