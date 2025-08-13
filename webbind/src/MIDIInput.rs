use super::*;




/// The MIDIInput class.
/// [`MIDIInput`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIInput {
    inner: MIDIPort,
}

impl FromVal for MIDIInput {
    fn from_val(v: &Any) -> Self {
        MIDIInput { inner: MIDIPort::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MIDIInput {
    type Target = MIDIPort;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MIDIInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MIDIInput {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MIDIInput {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MIDIInput> for Any {
    fn from(s: MIDIInput) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MIDIInput> for Any {
    fn from(s: &MIDIInput) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MIDIInput);


impl MIDIInput {
    /// Getter of the `onmidimessage` attribute.
    /// [`MIDIInput.onmidimessage`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)
    pub fn onmidimessage(&self) -> Any {
        self.inner.get("onmidimessage").as_::<Any>()
    }

    /// Setter of the `onmidimessage` attribute.
    /// [`MIDIInput.onmidimessage`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)
    pub fn set_onmidimessage(&mut self, value: &Any) {
        self.inner.set("onmidimessage", value);
    }
}
