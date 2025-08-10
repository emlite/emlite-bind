use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticatorAssertionResponseJSON {
    inner: Any,
}
impl FromVal for AuthenticatorAssertionResponseJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticatorAssertionResponseJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuthenticatorAssertionResponseJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticatorAssertionResponseJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuthenticatorAssertionResponseJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuthenticatorAssertionResponseJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuthenticatorAssertionResponseJSON> for Any {
    fn from(s: AuthenticatorAssertionResponseJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuthenticatorAssertionResponseJSON> for Any {
    fn from(s: &AuthenticatorAssertionResponseJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticatorAssertionResponseJSON {
    pub fn client_data_json(&self) -> Any {
        self.inner.get("clientDataJSON").as_::<Any>()
    }

    pub fn set_client_data_json(&mut self, value: &Any) {
        self.inner.set("clientDataJSON", value);
    }
}
impl AuthenticatorAssertionResponseJSON {
    pub fn authenticator_data(&self) -> Any {
        self.inner.get("authenticatorData").as_::<Any>()
    }

    pub fn set_authenticator_data(&mut self, value: &Any) {
        self.inner.set("authenticatorData", value);
    }
}
impl AuthenticatorAssertionResponseJSON {
    pub fn signature(&self) -> Any {
        self.inner.get("signature").as_::<Any>()
    }

    pub fn set_signature(&mut self, value: &Any) {
        self.inner.set("signature", value);
    }
}
impl AuthenticatorAssertionResponseJSON {
    pub fn user_handle(&self) -> Any {
        self.inner.get("userHandle").as_::<Any>()
    }

    pub fn set_user_handle(&mut self, value: &Any) {
        self.inner.set("userHandle", value);
    }
}
