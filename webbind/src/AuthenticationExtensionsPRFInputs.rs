use super::*;

/// The AuthenticationExtensionsPRFInputs dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPRFInputs {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsPRFInputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPRFInputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsPRFInputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsPRFInputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsPRFInputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsPRFInputs {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsPRFInputs> for Any {
    fn from(s: AuthenticationExtensionsPRFInputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsPRFInputs> for Any {
    fn from(s: &AuthenticationExtensionsPRFInputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPRFInputs {
    /// Getter of the `eval` attribute.
    pub fn eval(&self) -> AuthenticationExtensionsPRFValues {
        self.inner
            .get("eval")
            .as_::<AuthenticationExtensionsPRFValues>()
    }

    /// Setter of the `eval` attribute.
    pub fn set_eval(&mut self, value: &AuthenticationExtensionsPRFValues) {
        self.inner.set("eval", value);
    }
}
impl AuthenticationExtensionsPRFInputs {
    /// Getter of the `evalByCredential` attribute.
    pub fn eval_by_credential(&self) -> Record<JsString, AuthenticationExtensionsPRFValues> {
        self.inner
            .get("evalByCredential")
            .as_::<Record<JsString, AuthenticationExtensionsPRFValues>>()
    }

    /// Setter of the `evalByCredential` attribute.
    pub fn set_eval_by_credential(
        &mut self,
        value: &Record<JsString, AuthenticationExtensionsPRFValues>,
    ) {
        self.inner.set("evalByCredential", value);
    }
}
