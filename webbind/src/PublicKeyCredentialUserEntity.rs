use super::*;

/// The PublicKeyCredentialUserEntity dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PublicKeyCredentialUserEntity {
    inner: Any,
}

impl FromVal for PublicKeyCredentialUserEntity {
    fn from_val(v: &Any) -> Self {
        PublicKeyCredentialUserEntity { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PublicKeyCredentialUserEntity {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PublicKeyCredentialUserEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PublicKeyCredentialUserEntity {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PublicKeyCredentialUserEntity {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PublicKeyCredentialUserEntity> for Any {
    fn from(s: PublicKeyCredentialUserEntity) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PublicKeyCredentialUserEntity> for Any {
    fn from(s: &PublicKeyCredentialUserEntity) -> Any {
        s.inner.clone()
    }
}

impl PublicKeyCredentialUserEntity {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> Any {
        self.inner.get("id").as_::<Any>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &Any) {
        self.inner.set("id", value);
    }
}
impl PublicKeyCredentialUserEntity {
    /// Getter of the `displayName` attribute.
    pub fn display_name(&self) -> JsString {
        self.inner.get("displayName").as_::<JsString>()
    }

    /// Setter of the `displayName` attribute.
    pub fn set_display_name(&mut self, value: &JsString) {
        self.inner.set("displayName", value);
    }
}
