use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PasswordCredentialData {
    inner: Any,
}
impl FromVal for PasswordCredentialData {
    fn from_val(v: &Any) -> Self {
        PasswordCredentialData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PasswordCredentialData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PasswordCredentialData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PasswordCredentialData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PasswordCredentialData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PasswordCredentialData> for Any {
    fn from(s: PasswordCredentialData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PasswordCredentialData> for Any {
    fn from(s: &PasswordCredentialData) -> Any {
        s.inner.clone()
    }
}

impl PasswordCredentialData {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl PasswordCredentialData {
    pub fn icon_url(&self) -> JsString {
        self.inner.get("iconURL").as_::<JsString>()
    }

    pub fn set_icon_url(&mut self, value: &JsString) {
        self.inner.set("iconURL", value);
    }
}
impl PasswordCredentialData {
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }

    pub fn set_origin(&mut self, value: &JsString) {
        self.inner.set("origin", value);
    }
}
impl PasswordCredentialData {
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }

    pub fn set_password(&mut self, value: &JsString) {
        self.inner.set("password", value);
    }
}
