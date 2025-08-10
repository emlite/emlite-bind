use super::*;

/// The MIDIConnectionEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIConnectionEventInit {
    inner: Any,
}

impl FromVal for MIDIConnectionEventInit {
    fn from_val(v: &Any) -> Self {
        MIDIConnectionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MIDIConnectionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MIDIConnectionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MIDIConnectionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MIDIConnectionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MIDIConnectionEventInit> for Any {
    fn from(s: MIDIConnectionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MIDIConnectionEventInit> for Any {
    fn from(s: &MIDIConnectionEventInit) -> Any {
        s.inner.clone()
    }
}

impl MIDIConnectionEventInit {
    /// Getter of the `port` attribute.
    pub fn port(&self) -> MIDIPort {
        self.inner.get("port").as_::<MIDIPort>()
    }

    /// Setter of the `port` attribute.
    pub fn set_port(&mut self, value: &MIDIPort) {
        self.inner.set("port", value);
    }
}
