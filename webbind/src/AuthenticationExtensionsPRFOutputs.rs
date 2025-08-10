use super::*;

/// The AuthenticationExtensionsPRFOutputs dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPRFOutputs {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsPRFOutputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPRFOutputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsPRFOutputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsPRFOutputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsPRFOutputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsPRFOutputs {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsPRFOutputs> for Any {
    fn from(s: AuthenticationExtensionsPRFOutputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsPRFOutputs> for Any {
    fn from(s: &AuthenticationExtensionsPRFOutputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPRFOutputs {
    /// Getter of the `enabled` attribute.
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    /// Setter of the `enabled` attribute.
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl AuthenticationExtensionsPRFOutputs {
    /// Getter of the `results` attribute.
    pub fn results(&self) -> AuthenticationExtensionsPRFValues {
        self.inner
            .get("results")
            .as_::<AuthenticationExtensionsPRFValues>()
    }

    /// Setter of the `results` attribute.
    pub fn set_results(&mut self, value: &AuthenticationExtensionsPRFValues) {
        self.inner.set("results", value);
    }
}
