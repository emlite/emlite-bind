use super::*;

/// The AuthenticationExtensionsPRFOutputsJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPRFOutputsJSON {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsPRFOutputsJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPRFOutputsJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsPRFOutputsJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsPRFOutputsJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsPRFOutputsJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsPRFOutputsJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsPRFOutputsJSON> for Any {
    fn from(s: AuthenticationExtensionsPRFOutputsJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsPRFOutputsJSON> for Any {
    fn from(s: &AuthenticationExtensionsPRFOutputsJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPRFOutputsJSON {
    /// Getter of the `enabled` attribute.
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    /// Setter of the `enabled` attribute.
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl AuthenticationExtensionsPRFOutputsJSON {
    /// Getter of the `results` attribute.
    pub fn results(&self) -> AuthenticationExtensionsPRFValuesJSON {
        self.inner
            .get("results")
            .as_::<AuthenticationExtensionsPRFValuesJSON>()
    }

    /// Setter of the `results` attribute.
    pub fn set_results(&mut self, value: &AuthenticationExtensionsPRFValuesJSON) {
        self.inner.set("results", value);
    }
}
