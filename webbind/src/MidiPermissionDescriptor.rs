use super::*;




/// The MidiPermissionDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MidiPermissionDescriptor {
    inner: Any,
}

impl FromVal for MidiPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        MidiPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MidiPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MidiPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MidiPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MidiPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MidiPermissionDescriptor> for Any {
    fn from(s: MidiPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MidiPermissionDescriptor> for Any {
    fn from(s: &MidiPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl MidiPermissionDescriptor {
    /// Getter of the `sysex` attribute.
    pub fn sysex(&self) -> bool {
        self.inner.get("sysex").as_::<bool>()
    }

    /// Setter of the `sysex` attribute.
    pub fn set_sysex(&mut self, value: bool) {
        self.inner.set("sysex", value);
    }
}
