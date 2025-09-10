use super::*;

/// The AuthenticationExtensionsPRFInputsJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPRFInputsJSON {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsPRFInputsJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPRFInputsJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsPRFInputsJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsPRFInputsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsPRFInputsJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsPRFInputsJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsPRFInputsJSON> for Any {
    fn from(s: AuthenticationExtensionsPRFInputsJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsPRFInputsJSON> for Any {
    fn from(s: &AuthenticationExtensionsPRFInputsJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPRFInputsJSON {
    /// Getter of the `eval` attribute.
    pub fn eval(&self) -> AuthenticationExtensionsPRFValuesJSON {
        self.inner
            .get("eval")
            .as_::<AuthenticationExtensionsPRFValuesJSON>()
    }

    /// Setter of the `eval` attribute.
    pub fn set_eval(&mut self, value: &AuthenticationExtensionsPRFValuesJSON) {
        self.inner.set("eval", value);
    }
}
impl AuthenticationExtensionsPRFInputsJSON {
    /// Getter of the `evalByCredential` attribute.
    pub fn eval_by_credential(&self) -> Record<JsString, AuthenticationExtensionsPRFValuesJSON> {
        self.inner
            .get("evalByCredential")
            .as_::<Record<JsString, AuthenticationExtensionsPRFValuesJSON>>()
    }

    /// Setter of the `evalByCredential` attribute.
    pub fn set_eval_by_credential(
        &mut self,
        value: &Record<JsString, AuthenticationExtensionsPRFValuesJSON>,
    ) {
        self.inner.set("evalByCredential", value);
    }
}
