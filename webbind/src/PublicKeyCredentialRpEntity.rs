use super::*;




/// The PublicKeyCredentialRpEntity dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialRpEntity {
    inner: Any,
}

impl FromVal for PublicKeyCredentialRpEntity {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialRpEntity { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialRpEntity {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialRpEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialRpEntity {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialRpEntity {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PublicKeyCredentialRpEntity> for Any {
    fn from(s: PublicKeyCredentialRpEntity) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialRpEntity> for Any {
    fn from(s: &PublicKeyCredentialRpEntity) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialRpEntity {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
