use super::*;

/// The RTCSentRtpStreamStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSentRtpStreamStats {
    inner: Any,
}

impl FromVal for RTCSentRtpStreamStats {
    fn from_val(v: &Any) -> Self {
        RTCSentRtpStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCSentRtpStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCSentRtpStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCSentRtpStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCSentRtpStreamStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCSentRtpStreamStats> for Any {
    fn from(s: RTCSentRtpStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCSentRtpStreamStats> for Any {
    fn from(s: &RTCSentRtpStreamStats) -> Any {
        s.inner.clone()
    }
}

impl RTCSentRtpStreamStats {
    /// Getter of the `packetsSent` attribute.
    pub fn packets_sent(&self) -> u64 {
        self.inner.get("packetsSent").as_::<u64>()
    }

    /// Setter of the `packetsSent` attribute.
    pub fn set_packets_sent(&mut self, value: u64) {
        self.inner.set("packetsSent", value);
    }
}
impl RTCSentRtpStreamStats {
    /// Getter of the `bytesSent` attribute.
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    /// Setter of the `bytesSent` attribute.
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl RTCSentRtpStreamStats {
    /// Getter of the `packetsSentWithEct1` attribute.
    pub fn packets_sent_with_ect1(&self) -> u64 {
        self.inner.get("packetsSentWithEct1").as_::<u64>()
    }

    /// Setter of the `packetsSentWithEct1` attribute.
    pub fn set_packets_sent_with_ect1(&mut self, value: u64) {
        self.inner.set("packetsSentWithEct1", value);
    }
}
