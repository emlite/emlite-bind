use super::*;

/// The CurrentUserDetailsOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CurrentUserDetailsOptions {
    inner: Any,
}

impl FromVal for CurrentUserDetailsOptions {
    fn from_val(v: &Any) -> Self {
        CurrentUserDetailsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CurrentUserDetailsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CurrentUserDetailsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CurrentUserDetailsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CurrentUserDetailsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CurrentUserDetailsOptions> for Any {
    fn from(s: CurrentUserDetailsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CurrentUserDetailsOptions> for Any {
    fn from(s: &CurrentUserDetailsOptions) -> Any {
        s.inner.clone()
    }
}

impl CurrentUserDetailsOptions {
    /// Getter of the `rpId` attribute.
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    /// Setter of the `rpId` attribute.
    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl CurrentUserDetailsOptions {
    /// Getter of the `userId` attribute.
    pub fn user_id(&self) -> Any {
        self.inner.get("userId").as_::<Any>()
    }

    /// Setter of the `userId` attribute.
    pub fn set_user_id(&mut self, value: &Any) {
        self.inner.set("userId", value);
    }
}
impl CurrentUserDetailsOptions {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl CurrentUserDetailsOptions {
    /// Getter of the `displayName` attribute.
    pub fn display_name(&self) -> JsString {
        self.inner.get("displayName").as_::<JsString>()
    }

    /// Setter of the `displayName` attribute.
    pub fn set_display_name(&mut self, value: &JsString) {
        self.inner.set("displayName", value);
    }
}
