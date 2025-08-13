use super::*;




/// The SerialOutputSignals dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialOutputSignals {
    inner: Any,
}

impl FromVal for SerialOutputSignals {
    fn from_val(v: &Any) -> Self {
        SerialOutputSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SerialOutputSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SerialOutputSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SerialOutputSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SerialOutputSignals {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SerialOutputSignals> for Any {
    fn from(s: SerialOutputSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SerialOutputSignals> for Any {
    fn from(s: &SerialOutputSignals) -> Any {
        s.inner.clone()
    }
}

impl SerialOutputSignals {
    /// Getter of the `dataTerminalReady` attribute.
    pub fn data_terminal_ready(&self) -> bool {
        self.inner.get("dataTerminalReady").as_::<bool>()
    }

    /// Setter of the `dataTerminalReady` attribute.
    pub fn set_data_terminal_ready(&mut self, value: bool) {
        self.inner.set("dataTerminalReady", value);
    }
}
impl SerialOutputSignals {
    /// Getter of the `requestToSend` attribute.
    pub fn request_to_send(&self) -> bool {
        self.inner.get("requestToSend").as_::<bool>()
    }

    /// Setter of the `requestToSend` attribute.
    pub fn set_request_to_send(&mut self, value: bool) {
        self.inner.set("requestToSend", value);
    }
}
impl SerialOutputSignals {
    /// Getter of the `break` attribute.
    pub fn break_(&self) -> bool {
        self.inner.get("break").as_::<bool>()
    }

    /// Setter of the `break` attribute.
    pub fn set_break_(&mut self, value: bool) {
        self.inner.set("break", value);
    }
}
