use super::*;




/// The MIDIOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIOptions {
    inner: Any,
}

impl FromVal for MIDIOptions {
    fn from_val(v: &Any) -> Self {
        MIDIOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MIDIOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MIDIOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MIDIOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MIDIOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MIDIOptions> for Any {
    fn from(s: MIDIOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MIDIOptions> for Any {
    fn from(s: &MIDIOptions) -> Any {
        s.inner.clone()
    }
}

impl MIDIOptions {
    /// Getter of the `sysex` attribute.
    pub fn sysex(&self) -> bool {
        self.inner.get("sysex").as_::<bool>()
    }

    /// Setter of the `sysex` attribute.
    pub fn set_sysex(&mut self, value: bool) {
        self.inner.set("sysex", value);
    }
}
impl MIDIOptions {
    /// Getter of the `software` attribute.
    pub fn software(&self) -> bool {
        self.inner.get("software").as_::<bool>()
    }

    /// Setter of the `software` attribute.
    pub fn set_software(&mut self, value: bool) {
        self.inner.set("software", value);
    }
}
