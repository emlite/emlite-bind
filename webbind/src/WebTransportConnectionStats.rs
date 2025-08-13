use super::*;




/// The WebTransportConnectionStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportConnectionStats {
    inner: Any,
}

impl FromVal for WebTransportConnectionStats {
    fn from_val(v: &Any) -> Self {
        WebTransportConnectionStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportConnectionStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportConnectionStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportConnectionStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportConnectionStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebTransportConnectionStats> for Any {
    fn from(s: WebTransportConnectionStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportConnectionStats> for Any {
    fn from(s: &WebTransportConnectionStats) -> Any {
        s.inner.clone()
    }
}

impl WebTransportConnectionStats {
    /// Getter of the `bytesSent` attribute.
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    /// Setter of the `bytesSent` attribute.
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `packetsSent` attribute.
    pub fn packets_sent(&self) -> u64 {
        self.inner.get("packetsSent").as_::<u64>()
    }

    /// Setter of the `packetsSent` attribute.
    pub fn set_packets_sent(&mut self, value: u64) {
        self.inner.set("packetsSent", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `bytesLost` attribute.
    pub fn bytes_lost(&self) -> u64 {
        self.inner.get("bytesLost").as_::<u64>()
    }

    /// Setter of the `bytesLost` attribute.
    pub fn set_bytes_lost(&mut self, value: u64) {
        self.inner.set("bytesLost", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `packetsLost` attribute.
    pub fn packets_lost(&self) -> u64 {
        self.inner.get("packetsLost").as_::<u64>()
    }

    /// Setter of the `packetsLost` attribute.
    pub fn set_packets_lost(&mut self, value: u64) {
        self.inner.set("packetsLost", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `bytesReceived` attribute.
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    /// Setter of the `bytesReceived` attribute.
    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `packetsReceived` attribute.
    pub fn packets_received(&self) -> u64 {
        self.inner.get("packetsReceived").as_::<u64>()
    }

    /// Setter of the `packetsReceived` attribute.
    pub fn set_packets_received(&mut self, value: u64) {
        self.inner.set("packetsReceived", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `smoothedRtt` attribute.
    pub fn smoothed_rtt(&self) -> Any {
        self.inner.get("smoothedRtt").as_::<Any>()
    }

    /// Setter of the `smoothedRtt` attribute.
    pub fn set_smoothed_rtt(&mut self, value: &Any) {
        self.inner.set("smoothedRtt", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `rttVariation` attribute.
    pub fn rtt_variation(&self) -> Any {
        self.inner.get("rttVariation").as_::<Any>()
    }

    /// Setter of the `rttVariation` attribute.
    pub fn set_rtt_variation(&mut self, value: &Any) {
        self.inner.set("rttVariation", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `minRtt` attribute.
    pub fn min_rtt(&self) -> Any {
        self.inner.get("minRtt").as_::<Any>()
    }

    /// Setter of the `minRtt` attribute.
    pub fn set_min_rtt(&mut self, value: &Any) {
        self.inner.set("minRtt", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `datagrams` attribute.
    pub fn datagrams(&self) -> WebTransportDatagramStats {
        self.inner.get("datagrams").as_::<WebTransportDatagramStats>()
    }

    /// Setter of the `datagrams` attribute.
    pub fn set_datagrams(&mut self, value: &WebTransportDatagramStats) {
        self.inner.set("datagrams", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `estimatedSendRate` attribute.
    pub fn estimated_send_rate(&self) -> u64 {
        self.inner.get("estimatedSendRate").as_::<u64>()
    }

    /// Setter of the `estimatedSendRate` attribute.
    pub fn set_estimated_send_rate(&mut self, value: u64) {
        self.inner.set("estimatedSendRate", value);
    }
}
impl WebTransportConnectionStats {
    /// Getter of the `atSendCapacity` attribute.
    pub fn at_send_capacity(&self) -> bool {
        self.inner.get("atSendCapacity").as_::<bool>()
    }

    /// Setter of the `atSendCapacity` attribute.
    pub fn set_at_send_capacity(&mut self, value: bool) {
        self.inner.set("atSendCapacity", value);
    }
}
