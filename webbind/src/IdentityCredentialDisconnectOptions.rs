use super::*;




/// The IdentityCredentialDisconnectOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredentialDisconnectOptions {
    inner: Any,
}

impl FromVal for IdentityCredentialDisconnectOptions {
    fn from_val(v: &Any) -> Self {
        IdentityCredentialDisconnectOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityCredentialDisconnectOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityCredentialDisconnectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityCredentialDisconnectOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityCredentialDisconnectOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdentityCredentialDisconnectOptions> for Any {
    fn from(s: IdentityCredentialDisconnectOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityCredentialDisconnectOptions> for Any {
    fn from(s: &IdentityCredentialDisconnectOptions) -> Any {
        s.inner.clone()
    }
}

impl IdentityCredentialDisconnectOptions {
    /// Getter of the `accountHint` attribute.
    pub fn account_hint(&self) -> JsString {
        self.inner.get("accountHint").as_::<JsString>()
    }

    /// Setter of the `accountHint` attribute.
    pub fn set_account_hint(&mut self, value: &JsString) {
        self.inner.set("accountHint", value);
    }
}
