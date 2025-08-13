use super::*;




/// The AuthenticationResponseJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationResponseJSON {
    inner: Any,
}

impl FromVal for AuthenticationResponseJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticationResponseJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationResponseJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationResponseJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationResponseJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationResponseJSON {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuthenticationResponseJSON> for Any {
    fn from(s: AuthenticationResponseJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationResponseJSON> for Any {
    fn from(s: &AuthenticationResponseJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationResponseJSON {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl AuthenticationResponseJSON {
    /// Getter of the `rawId` attribute.
    pub fn raw_id(&self) -> Any {
        self.inner.get("rawId").as_::<Any>()
    }

    /// Setter of the `rawId` attribute.
    pub fn set_raw_id(&mut self, value: &Any) {
        self.inner.set("rawId", value);
    }
}
impl AuthenticationResponseJSON {
    /// Getter of the `response` attribute.
    pub fn response(&self) -> AuthenticatorAssertionResponseJSON {
        self.inner.get("response").as_::<AuthenticatorAssertionResponseJSON>()
    }

    /// Setter of the `response` attribute.
    pub fn set_response(&mut self, value: &AuthenticatorAssertionResponseJSON) {
        self.inner.set("response", value);
    }
}
impl AuthenticationResponseJSON {
    /// Getter of the `authenticatorAttachment` attribute.
    pub fn authenticator_attachment(&self) -> JsString {
        self.inner.get("authenticatorAttachment").as_::<JsString>()
    }

    /// Setter of the `authenticatorAttachment` attribute.
    pub fn set_authenticator_attachment(&mut self, value: &JsString) {
        self.inner.set("authenticatorAttachment", value);
    }
}
impl AuthenticationResponseJSON {
    /// Getter of the `clientExtensionResults` attribute.
    pub fn client_extension_results(&self) -> AuthenticationExtensionsClientOutputsJSON {
        self.inner.get("clientExtensionResults").as_::<AuthenticationExtensionsClientOutputsJSON>()
    }

    /// Setter of the `clientExtensionResults` attribute.
    pub fn set_client_extension_results(&mut self, value: &AuthenticationExtensionsClientOutputsJSON) {
        self.inner.set("clientExtensionResults", value);
    }
}
impl AuthenticationResponseJSON {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
