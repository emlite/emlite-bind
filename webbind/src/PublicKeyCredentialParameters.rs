use super::*;




/// The PublicKeyCredentialParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialParameters {
    inner: Any,
}

impl FromVal for PublicKeyCredentialParameters {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PublicKeyCredentialParameters> for Any {
    fn from(s: PublicKeyCredentialParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialParameters> for Any {
    fn from(s: &PublicKeyCredentialParameters) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialParameters {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl PublicKeyCredentialParameters {
    /// Getter of the `alg` attribute.
    pub fn alg(&self) -> Any {
        self.inner.get("alg").as_::<Any>()
    }

    /// Setter of the `alg` attribute.
    pub fn set_alg(&mut self, value: &Any) {
        self.inner.set("alg", value);
    }
}
