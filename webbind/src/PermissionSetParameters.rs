use super::*;




/// The PermissionSetParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PermissionSetParameters {
    inner: Any,
}

impl FromVal for PermissionSetParameters {
    fn from_val(v: &Any) -> Self {
        PermissionSetParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PermissionSetParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PermissionSetParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PermissionSetParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PermissionSetParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PermissionSetParameters> for Any {
    fn from(s: PermissionSetParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PermissionSetParameters> for Any {
    fn from(s: &PermissionSetParameters) -> Any {
        s.inner.clone()
    }
}

impl PermissionSetParameters {
    /// Getter of the `descriptor` attribute.
    pub fn descriptor(&self) -> Object {
        self.inner.get("descriptor").as_::<Object>()
    }

    /// Setter of the `descriptor` attribute.
    pub fn set_descriptor(&mut self, value: &Object) {
        self.inner.set("descriptor", value);
    }
}
impl PermissionSetParameters {
    /// Getter of the `state` attribute.
    pub fn state(&self) -> PermissionState {
        self.inner.get("state").as_::<PermissionState>()
    }

    /// Setter of the `state` attribute.
    pub fn set_state(&mut self, value: &PermissionState) {
        self.inner.set("state", value);
    }
}
