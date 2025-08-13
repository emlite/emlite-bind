use super::*;




/// The PublicKeyCredentialDescriptorJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialDescriptorJSON {
    inner: Any,
}

impl FromVal for PublicKeyCredentialDescriptorJSON {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialDescriptorJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialDescriptorJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialDescriptorJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialDescriptorJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialDescriptorJSON {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PublicKeyCredentialDescriptorJSON> for Any {
    fn from(s: PublicKeyCredentialDescriptorJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialDescriptorJSON> for Any {
    fn from(s: &PublicKeyCredentialDescriptorJSON) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialDescriptorJSON {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl PublicKeyCredentialDescriptorJSON {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> Any {
        self.inner.get("id").as_::<Any>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &Any) {
        self.inner.set("id", value);
    }
}
impl PublicKeyCredentialDescriptorJSON {
    /// Getter of the `transports` attribute.
    pub fn transports(&self) -> TypedArray<JsString> {
        self.inner.get("transports").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `transports` attribute.
    pub fn set_transports(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("transports", value);
    }
}
