use super::*;




/// The CredentialData dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialData {
    inner: Any,
}

impl FromVal for CredentialData {
    fn from_val(v: &Any) -> Self {
        CredentialData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CredentialData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CredentialData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CredentialData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CredentialData {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CredentialData> for Any {
    fn from(s: CredentialData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CredentialData> for Any {
    fn from(s: &CredentialData) -> Any {
        s.inner.clone()
    }
}

impl CredentialData {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
