use super::*;




/// The CredentialRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialRequestOptions {
    inner: Any,
}

impl FromVal for CredentialRequestOptions {
    fn from_val(v: &Any) -> Self {
        CredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CredentialRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CredentialRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CredentialRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CredentialRequestOptions> for Any {
    fn from(s: CredentialRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CredentialRequestOptions> for Any {
    fn from(s: &CredentialRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl CredentialRequestOptions {
    /// Getter of the `publicKey` attribute.
    pub fn public_key(&self) -> PublicKeyCredentialRequestOptions {
        self.inner.get("publicKey").as_::<PublicKeyCredentialRequestOptions>()
    }

    /// Setter of the `publicKey` attribute.
    pub fn set_public_key(&mut self, value: &PublicKeyCredentialRequestOptions) {
        self.inner.set("publicKey", value);
    }
}
