use super::*;




/// The ClipboardPermissionDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClipboardPermissionDescriptor {
    inner: Any,
}

impl FromVal for ClipboardPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        ClipboardPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ClipboardPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ClipboardPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ClipboardPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ClipboardPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ClipboardPermissionDescriptor> for Any {
    fn from(s: ClipboardPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ClipboardPermissionDescriptor> for Any {
    fn from(s: &ClipboardPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl ClipboardPermissionDescriptor {
    /// Getter of the `allowWithoutGesture` attribute.
    pub fn allow_without_gesture(&self) -> bool {
        self.inner.get("allowWithoutGesture").as_::<bool>()
    }

    /// Setter of the `allowWithoutGesture` attribute.
    pub fn set_allow_without_gesture(&mut self, value: bool) {
        self.inner.set("allowWithoutGesture", value);
    }
}
