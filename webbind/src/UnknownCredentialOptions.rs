use super::*;




/// The UnknownCredentialOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UnknownCredentialOptions {
    inner: Any,
}

impl FromVal for UnknownCredentialOptions {
    fn from_val(v: &Any) -> Self {
        UnknownCredentialOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for UnknownCredentialOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for UnknownCredentialOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for UnknownCredentialOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for UnknownCredentialOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<UnknownCredentialOptions> for Any {
    fn from(s: UnknownCredentialOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&UnknownCredentialOptions> for Any {
    fn from(s: &UnknownCredentialOptions) -> Any {
        s.inner.clone()
    }
}

impl UnknownCredentialOptions {
    /// Getter of the `rpId` attribute.
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    /// Setter of the `rpId` attribute.
    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl UnknownCredentialOptions {
    /// Getter of the `credentialId` attribute.
    pub fn credential_id(&self) -> Any {
        self.inner.get("credentialId").as_::<Any>()
    }

    /// Setter of the `credentialId` attribute.
    pub fn set_credential_id(&mut self, value: &Any) {
        self.inner.set("credentialId", value);
    }
}
