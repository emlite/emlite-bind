use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RegistrationResponseJSON {
    inner: Any,
}
impl FromVal for RegistrationResponseJSON {
    fn from_val(v: &Any) -> Self {
        RegistrationResponseJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RegistrationResponseJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RegistrationResponseJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RegistrationResponseJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RegistrationResponseJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RegistrationResponseJSON> for Any {
    fn from(s: RegistrationResponseJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RegistrationResponseJSON> for Any {
    fn from(s: &RegistrationResponseJSON) -> Any {
        s.inner.clone()
    }
}

impl RegistrationResponseJSON {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl RegistrationResponseJSON {
    pub fn raw_id(&self) -> Any {
        self.inner.get("rawId").as_::<Any>()
    }

    pub fn set_raw_id(&mut self, value: &Any) {
        self.inner.set("rawId", value);
    }
}
impl RegistrationResponseJSON {
    pub fn response(&self) -> AuthenticatorAttestationResponseJSON {
        self.inner
            .get("response")
            .as_::<AuthenticatorAttestationResponseJSON>()
    }

    pub fn set_response(&mut self, value: &AuthenticatorAttestationResponseJSON) {
        self.inner.set("response", value);
    }
}
impl RegistrationResponseJSON {
    pub fn authenticator_attachment(&self) -> JsString {
        self.inner.get("authenticatorAttachment").as_::<JsString>()
    }

    pub fn set_authenticator_attachment(&mut self, value: &JsString) {
        self.inner.set("authenticatorAttachment", value);
    }
}
impl RegistrationResponseJSON {
    pub fn client_extension_results(&self) -> AuthenticationExtensionsClientOutputsJSON {
        self.inner
            .get("clientExtensionResults")
            .as_::<AuthenticationExtensionsClientOutputsJSON>()
    }

    pub fn set_client_extension_results(
        &mut self,
        value: &AuthenticationExtensionsClientOutputsJSON,
    ) {
        self.inner.set("clientExtensionResults", value);
    }
}
impl RegistrationResponseJSON {
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
