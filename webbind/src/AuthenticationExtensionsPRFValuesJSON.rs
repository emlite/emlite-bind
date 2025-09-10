use super::*;

/// The AuthenticationExtensionsPRFValuesJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPRFValuesJSON {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsPRFValuesJSON {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPRFValuesJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsPRFValuesJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsPRFValuesJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsPRFValuesJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsPRFValuesJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticationExtensionsPRFValuesJSON> for Any {
    fn from(s: AuthenticationExtensionsPRFValuesJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsPRFValuesJSON> for Any {
    fn from(s: &AuthenticationExtensionsPRFValuesJSON) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPRFValuesJSON {
    /// Getter of the `first` attribute.
    pub fn first(&self) -> Any {
        self.inner.get("first").as_::<Any>()
    }

    /// Setter of the `first` attribute.
    pub fn set_first(&mut self, value: &Any) {
        self.inner.set("first", value);
    }
}
impl AuthenticationExtensionsPRFValuesJSON {
    /// Getter of the `second` attribute.
    pub fn second(&self) -> Any {
        self.inner.get("second").as_::<Any>()
    }

    /// Setter of the `second` attribute.
    pub fn set_second(&mut self, value: &Any) {
        self.inner.set("second", value);
    }
}
