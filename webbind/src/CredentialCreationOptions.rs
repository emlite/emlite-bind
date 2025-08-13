use super::*;




/// The CredentialCreationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialCreationOptions {
    inner: Any,
}

impl FromVal for CredentialCreationOptions {
    fn from_val(v: &Any) -> Self {
        CredentialCreationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CredentialCreationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CredentialCreationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CredentialCreationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CredentialCreationOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CredentialCreationOptions> for Any {
    fn from(s: CredentialCreationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CredentialCreationOptions> for Any {
    fn from(s: &CredentialCreationOptions) -> Any {
        s.inner.clone()
    }
}

impl CredentialCreationOptions {
    /// Getter of the `publicKey` attribute.
    pub fn public_key(&self) -> PublicKeyCredentialCreationOptions {
        self.inner.get("publicKey").as_::<PublicKeyCredentialCreationOptions>()
    }

    /// Setter of the `publicKey` attribute.
    pub fn set_public_key(&mut self, value: &PublicKeyCredentialCreationOptions) {
        self.inner.set("publicKey", value);
    }
}
