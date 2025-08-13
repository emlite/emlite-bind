use super::*;




/// The MIDIMessageEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIMessageEventInit {
    inner: Any,
}

impl FromVal for MIDIMessageEventInit {
    fn from_val(v: &Any) -> Self {
        MIDIMessageEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MIDIMessageEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MIDIMessageEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MIDIMessageEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MIDIMessageEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MIDIMessageEventInit> for Any {
    fn from(s: MIDIMessageEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MIDIMessageEventInit> for Any {
    fn from(s: &MIDIMessageEventInit) -> Any {
        s.inner.clone()
    }
}

impl MIDIMessageEventInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Uint8Array {
        self.inner.get("data").as_::<Uint8Array>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Uint8Array) {
        self.inner.set("data", value);
    }
}
