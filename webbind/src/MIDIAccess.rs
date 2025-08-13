use super::*;




/// The MIDIAccess class.
/// [`MIDIAccess`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIAccess {
    inner: EventTarget,
}

impl FromVal for MIDIAccess {
    fn from_val(v: &Any) -> Self {
        MIDIAccess { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MIDIAccess {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MIDIAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MIDIAccess {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MIDIAccess {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MIDIAccess> for Any {
    fn from(s: MIDIAccess) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MIDIAccess> for Any {
    fn from(s: &MIDIAccess) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MIDIAccess);


impl MIDIAccess {
    /// Getter of the `inputs` attribute.
    /// [`MIDIAccess.inputs`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/inputs)
    pub fn inputs(&self) -> MIDIInputMap {
        self.inner.get("inputs").as_::<MIDIInputMap>()
    }

}
impl MIDIAccess {
    /// Getter of the `outputs` attribute.
    /// [`MIDIAccess.outputs`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/outputs)
    pub fn outputs(&self) -> MIDIOutputMap {
        self.inner.get("outputs").as_::<MIDIOutputMap>()
    }

}
impl MIDIAccess {
    /// Getter of the `onstatechange` attribute.
    /// [`MIDIAccess.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`MIDIAccess.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
impl MIDIAccess {
    /// Getter of the `sysexEnabled` attribute.
    /// [`MIDIAccess.sysexEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/sysexEnabled)
    pub fn sysex_enabled(&self) -> bool {
        self.inner.get("sysexEnabled").as_::<bool>()
    }

}
