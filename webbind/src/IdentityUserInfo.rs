use super::*;




/// The IdentityUserInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityUserInfo {
    inner: Any,
}

impl FromVal for IdentityUserInfo {
    fn from_val(v: &Any) -> Self {
        IdentityUserInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityUserInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityUserInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityUserInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityUserInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdentityUserInfo> for Any {
    fn from(s: IdentityUserInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityUserInfo> for Any {
    fn from(s: &IdentityUserInfo) -> Any {
        s.inner.clone()
    }
}

impl IdentityUserInfo {
    /// Getter of the `email` attribute.
    pub fn email(&self) -> JsString {
        self.inner.get("email").as_::<JsString>()
    }

    /// Setter of the `email` attribute.
    pub fn set_email(&mut self, value: &JsString) {
        self.inner.set("email", value);
    }
}
impl IdentityUserInfo {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl IdentityUserInfo {
    /// Getter of the `givenName` attribute.
    pub fn given_name(&self) -> JsString {
        self.inner.get("givenName").as_::<JsString>()
    }

    /// Setter of the `givenName` attribute.
    pub fn set_given_name(&mut self, value: &JsString) {
        self.inner.set("givenName", value);
    }
}
impl IdentityUserInfo {
    /// Getter of the `picture` attribute.
    pub fn picture(&self) -> JsString {
        self.inner.get("picture").as_::<JsString>()
    }

    /// Setter of the `picture` attribute.
    pub fn set_picture(&mut self, value: &JsString) {
        self.inner.set("picture", value);
    }
}
