use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPRFValues {
    inner: Any,
}
impl FromVal for AuthenticationExtensionsPRFValues {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPRFValues { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AuthenticationExtensionsPRFValues {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AuthenticationExtensionsPRFValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AuthenticationExtensionsPRFValues {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AuthenticationExtensionsPRFValues {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AuthenticationExtensionsPRFValues> for Any {
    fn from(s: AuthenticationExtensionsPRFValues) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AuthenticationExtensionsPRFValues> for Any {
    fn from(s: &AuthenticationExtensionsPRFValues) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPRFValues {
    pub fn first(&self) -> Any {
        self.inner.get("first").as_::<Any>()
    }

    pub fn set_first(&mut self, value: &Any) {
        self.inner.set("first", value);
    }
}
impl AuthenticationExtensionsPRFValues {
    pub fn second(&self) -> Any {
        self.inner.get("second").as_::<Any>()
    }

    pub fn set_second(&mut self, value: &Any) {
        self.inner.set("second", value);
    }
}
