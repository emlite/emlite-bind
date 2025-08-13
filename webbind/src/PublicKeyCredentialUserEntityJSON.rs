use super::*;




/// The PublicKeyCredentialUserEntityJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialUserEntityJSON {
    inner: Any,
}

impl FromVal for PublicKeyCredentialUserEntityJSON {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialUserEntityJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialUserEntityJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialUserEntityJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialUserEntityJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialUserEntityJSON {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PublicKeyCredentialUserEntityJSON> for Any {
    fn from(s: PublicKeyCredentialUserEntityJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialUserEntityJSON> for Any {
    fn from(s: &PublicKeyCredentialUserEntityJSON) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialUserEntityJSON {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> Any {
        self.inner.get("id").as_::<Any>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &Any) {
        self.inner.set("id", value);
    }
}
impl PublicKeyCredentialUserEntityJSON {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl PublicKeyCredentialUserEntityJSON {
    /// Getter of the `displayName` attribute.
    pub fn display_name(&self) -> JsString {
        self.inner.get("displayName").as_::<JsString>()
    }

    /// Setter of the `displayName` attribute.
    pub fn set_display_name(&mut self, value: &JsString) {
        self.inner.set("displayName", value);
    }
}
