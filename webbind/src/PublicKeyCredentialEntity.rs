use super::*;

/// The PublicKeyCredentialEntity dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialEntity {
    inner: Any,
}

impl FromVal for PublicKeyCredentialEntity {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialEntity { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialEntity {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialEntity {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialEntity {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PublicKeyCredentialEntity> for Any {
    fn from(s: PublicKeyCredentialEntity) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialEntity> for Any {
    fn from(s: &PublicKeyCredentialEntity) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialEntity {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
