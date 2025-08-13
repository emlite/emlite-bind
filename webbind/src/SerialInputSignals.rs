use super::*;




/// The SerialInputSignals dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialInputSignals {
    inner: Any,
}

impl FromVal for SerialInputSignals {
    fn from_val(v: &Any) -> Self {
        SerialInputSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SerialInputSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SerialInputSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SerialInputSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SerialInputSignals {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SerialInputSignals> for Any {
    fn from(s: SerialInputSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SerialInputSignals> for Any {
    fn from(s: &SerialInputSignals) -> Any {
        s.inner.clone()
    }
}

impl SerialInputSignals {
    /// Getter of the `dataCarrierDetect` attribute.
    pub fn data_carrier_detect(&self) -> bool {
        self.inner.get("dataCarrierDetect").as_::<bool>()
    }

    /// Setter of the `dataCarrierDetect` attribute.
    pub fn set_data_carrier_detect(&mut self, value: bool) {
        self.inner.set("dataCarrierDetect", value);
    }
}
impl SerialInputSignals {
    /// Getter of the `clearToSend` attribute.
    pub fn clear_to_send(&self) -> bool {
        self.inner.get("clearToSend").as_::<bool>()
    }

    /// Setter of the `clearToSend` attribute.
    pub fn set_clear_to_send(&mut self, value: bool) {
        self.inner.set("clearToSend", value);
    }
}
impl SerialInputSignals {
    /// Getter of the `ringIndicator` attribute.
    pub fn ring_indicator(&self) -> bool {
        self.inner.get("ringIndicator").as_::<bool>()
    }

    /// Setter of the `ringIndicator` attribute.
    pub fn set_ring_indicator(&mut self, value: bool) {
        self.inner.set("ringIndicator", value);
    }
}
impl SerialInputSignals {
    /// Getter of the `dataSetReady` attribute.
    pub fn data_set_ready(&self) -> bool {
        self.inner.get("dataSetReady").as_::<bool>()
    }

    /// Setter of the `dataSetReady` attribute.
    pub fn set_data_set_ready(&mut self, value: bool) {
        self.inner.set("dataSetReady", value);
    }
}
