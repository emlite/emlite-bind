use super::*;

/// The WebTransportSendStreamStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendStreamStats {
    inner: Any,
}

impl FromVal for WebTransportSendStreamStats {
    fn from_val(v: &Any) -> Self {
        WebTransportSendStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportSendStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportSendStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportSendStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportSendStreamStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebTransportSendStreamStats> for Any {
    fn from(s: WebTransportSendStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportSendStreamStats> for Any {
    fn from(s: &WebTransportSendStreamStats) -> Any {
        s.inner.clone()
    }
}

impl WebTransportSendStreamStats {
    /// Getter of the `bytesWritten` attribute.
    pub fn bytes_written(&self) -> u64 {
        self.inner.get("bytesWritten").as_::<u64>()
    }

    /// Setter of the `bytesWritten` attribute.
    pub fn set_bytes_written(&mut self, value: u64) {
        self.inner.set("bytesWritten", value);
    }
}
impl WebTransportSendStreamStats {
    /// Getter of the `bytesSent` attribute.
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    /// Setter of the `bytesSent` attribute.
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl WebTransportSendStreamStats {
    /// Getter of the `bytesAcknowledged` attribute.
    pub fn bytes_acknowledged(&self) -> u64 {
        self.inner.get("bytesAcknowledged").as_::<u64>()
    }

    /// Setter of the `bytesAcknowledged` attribute.
    pub fn set_bytes_acknowledged(&mut self, value: u64) {
        self.inner.set("bytesAcknowledged", value);
    }
}
