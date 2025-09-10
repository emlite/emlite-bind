use super::*;

/// The SerialOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialOptions {
    inner: Any,
}

impl FromVal for SerialOptions {
    fn from_val(v: &Any) -> Self {
        SerialOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SerialOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SerialOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SerialOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SerialOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SerialOptions> for Any {
    fn from(s: SerialOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SerialOptions> for Any {
    fn from(s: &SerialOptions) -> Any {
        s.inner.clone()
    }
}

impl SerialOptions {
    /// Getter of the `baudRate` attribute.
    pub fn baud_rate(&self) -> u32 {
        self.inner.get("baudRate").as_::<u32>()
    }

    /// Setter of the `baudRate` attribute.
    pub fn set_baud_rate(&mut self, value: u32) {
        self.inner.set("baudRate", value);
    }
}
impl SerialOptions {
    /// Getter of the `dataBits` attribute.
    pub fn data_bits(&self) -> u8 {
        self.inner.get("dataBits").as_::<u8>()
    }

    /// Setter of the `dataBits` attribute.
    pub fn set_data_bits(&mut self, value: u8) {
        self.inner.set("dataBits", value);
    }
}
impl SerialOptions {
    /// Getter of the `stopBits` attribute.
    pub fn stop_bits(&self) -> u8 {
        self.inner.get("stopBits").as_::<u8>()
    }

    /// Setter of the `stopBits` attribute.
    pub fn set_stop_bits(&mut self, value: u8) {
        self.inner.set("stopBits", value);
    }
}
impl SerialOptions {
    /// Getter of the `parity` attribute.
    pub fn parity(&self) -> ParityType {
        self.inner.get("parity").as_::<ParityType>()
    }

    /// Setter of the `parity` attribute.
    pub fn set_parity(&mut self, value: &ParityType) {
        self.inner.set("parity", value);
    }
}
impl SerialOptions {
    /// Getter of the `bufferSize` attribute.
    pub fn buffer_size(&self) -> u32 {
        self.inner.get("bufferSize").as_::<u32>()
    }

    /// Setter of the `bufferSize` attribute.
    pub fn set_buffer_size(&mut self, value: u32) {
        self.inner.set("bufferSize", value);
    }
}
impl SerialOptions {
    /// Getter of the `flowControl` attribute.
    pub fn flow_control(&self) -> FlowControlType {
        self.inner.get("flowControl").as_::<FlowControlType>()
    }

    /// Setter of the `flowControl` attribute.
    pub fn set_flow_control(&mut self, value: &FlowControlType) {
        self.inner.set("flowControl", value);
    }
}
