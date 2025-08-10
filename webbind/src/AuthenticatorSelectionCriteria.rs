use super::*;

/// The AuthenticatorSelectionCriteria dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticatorSelectionCriteria {
    inner: Any,
}

impl FromVal for AuthenticatorSelectionCriteria {
    fn from_val(v: &Any) -> Self {
        AuthenticatorSelectionCriteria { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticatorSelectionCriteria {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticatorSelectionCriteria {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticatorSelectionCriteria {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticatorSelectionCriteria {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticatorSelectionCriteria> for Any {
    fn from(s: AuthenticatorSelectionCriteria) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticatorSelectionCriteria> for Any {
    fn from(s: &AuthenticatorSelectionCriteria) -> Any {
        s.inner.clone()
    }
}

impl AuthenticatorSelectionCriteria {
    /// Getter of the `authenticatorAttachment` attribute.
    pub fn authenticator_attachment(&self) -> JsString {
        self.inner.get("authenticatorAttachment").as_::<JsString>()
    }

    /// Setter of the `authenticatorAttachment` attribute.
    pub fn set_authenticator_attachment(&mut self, value: &JsString) {
        self.inner.set("authenticatorAttachment", value);
    }
}
impl AuthenticatorSelectionCriteria {
    /// Getter of the `residentKey` attribute.
    pub fn resident_key(&self) -> JsString {
        self.inner.get("residentKey").as_::<JsString>()
    }

    /// Setter of the `residentKey` attribute.
    pub fn set_resident_key(&mut self, value: &JsString) {
        self.inner.set("residentKey", value);
    }
}
impl AuthenticatorSelectionCriteria {
    /// Getter of the `requireResidentKey` attribute.
    pub fn require_resident_key(&self) -> bool {
        self.inner.get("requireResidentKey").as_::<bool>()
    }

    /// Setter of the `requireResidentKey` attribute.
    pub fn set_require_resident_key(&mut self, value: bool) {
        self.inner.set("requireResidentKey", value);
    }
}
impl AuthenticatorSelectionCriteria {
    /// Getter of the `userVerification` attribute.
    pub fn user_verification(&self) -> JsString {
        self.inner.get("userVerification").as_::<JsString>()
    }

    /// Setter of the `userVerification` attribute.
    pub fn set_user_verification(&mut self, value: &JsString) {
        self.inner.set("userVerification", value);
    }
}
