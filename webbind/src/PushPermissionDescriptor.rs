use super::*;




/// The PushPermissionDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushPermissionDescriptor {
    inner: Any,
}

impl FromVal for PushPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        PushPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PushPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PushPermissionDescriptor> for Any {
    fn from(s: PushPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushPermissionDescriptor> for Any {
    fn from(s: &PushPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl PushPermissionDescriptor {
    /// Getter of the `userVisibleOnly` attribute.
    pub fn user_visible_only(&self) -> bool {
        self.inner.get("userVisibleOnly").as_::<bool>()
    }

    /// Setter of the `userVisibleOnly` attribute.
    pub fn set_user_visible_only(&mut self, value: bool) {
        self.inner.set("userVisibleOnly", value);
    }
}
