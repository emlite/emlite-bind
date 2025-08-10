use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsLargeBlobOutputsJSON {
    inner: Any,
}
impl FromVal for AuthenticationExtensionsLargeBlobOutputsJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsLargeBlobOutputsJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuthenticationExtensionsLargeBlobOutputsJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticationExtensionsLargeBlobOutputsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuthenticationExtensionsLargeBlobOutputsJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuthenticationExtensionsLargeBlobOutputsJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuthenticationExtensionsLargeBlobOutputsJSON> for Any {
    fn from(s: AuthenticationExtensionsLargeBlobOutputsJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuthenticationExtensionsLargeBlobOutputsJSON> for Any {
    fn from(s: &AuthenticationExtensionsLargeBlobOutputsJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsLargeBlobOutputsJSON {
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl AuthenticationExtensionsLargeBlobOutputsJSON {
    pub fn blob(&self) -> Any {
        self.inner.get("blob").as_::<Any>()
    }

    pub fn set_blob(&mut self, value: &Any) {
        self.inner.set("blob", value);
    }
}
impl AuthenticationExtensionsLargeBlobOutputsJSON {
    pub fn written(&self) -> bool {
        self.inner.get("written").as_::<bool>()
    }

    pub fn set_written(&mut self, value: bool) {
        self.inner.set("written", value);
    }
}
