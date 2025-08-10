use super::*;

/// The PublicKeyCredentialDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialDescriptor {
    inner: Any,
}

impl FromVal for PublicKeyCredentialDescriptor {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PublicKeyCredentialDescriptor> for Any {
    fn from(s: PublicKeyCredentialDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialDescriptor> for Any {
    fn from(s: &PublicKeyCredentialDescriptor) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialDescriptor {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl PublicKeyCredentialDescriptor {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> Any {
        self.inner.get("id").as_::<Any>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &Any) {
        self.inner.set("id", value);
    }
}
impl PublicKeyCredentialDescriptor {
    /// Getter of the `transports` attribute.
    pub fn transports(&self) -> TypedArray<JsString> {
        self.inner.get("transports").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `transports` attribute.
    pub fn set_transports(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("transports", value);
    }
}
