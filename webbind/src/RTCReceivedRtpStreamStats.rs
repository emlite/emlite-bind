use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCReceivedRtpStreamStats {
    inner: Any,
}
impl FromVal for RTCReceivedRtpStreamStats {
    fn from_val(v: &Any) -> Self {
        RTCReceivedRtpStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCReceivedRtpStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCReceivedRtpStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCReceivedRtpStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCReceivedRtpStreamStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCReceivedRtpStreamStats> for Any {
    fn from(s: RTCReceivedRtpStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCReceivedRtpStreamStats> for Any {
    fn from(s: &RTCReceivedRtpStreamStats) -> Any {
        s.inner.clone()
    }
}

impl RTCReceivedRtpStreamStats {
    pub fn packets_received(&self) -> u64 {
        self.inner.get("packetsReceived").as_::<u64>()
    }

    pub fn set_packets_received(&mut self, value: u64) {
        self.inner.set("packetsReceived", value);
    }
}
impl RTCReceivedRtpStreamStats {
    pub fn packets_received_with_ect1(&self) -> u64 {
        self.inner.get("packetsReceivedWithEct1").as_::<u64>()
    }

    pub fn set_packets_received_with_ect1(&mut self, value: u64) {
        self.inner.set("packetsReceivedWithEct1", value);
    }
}
impl RTCReceivedRtpStreamStats {
    pub fn packets_received_with_ce(&self) -> u64 {
        self.inner.get("packetsReceivedWithCe").as_::<u64>()
    }

    pub fn set_packets_received_with_ce(&mut self, value: u64) {
        self.inner.set("packetsReceivedWithCe", value);
    }
}
impl RTCReceivedRtpStreamStats {
    pub fn packets_reported_as_lost(&self) -> u64 {
        self.inner.get("packetsReportedAsLost").as_::<u64>()
    }

    pub fn set_packets_reported_as_lost(&mut self, value: u64) {
        self.inner.set("packetsReportedAsLost", value);
    }
}
impl RTCReceivedRtpStreamStats {
    pub fn packets_reported_as_lost_but_recovered(&self) -> u64 {
        self.inner
            .get("packetsReportedAsLostButRecovered")
            .as_::<u64>()
    }

    pub fn set_packets_reported_as_lost_but_recovered(&mut self, value: u64) {
        self.inner.set("packetsReportedAsLostButRecovered", value);
    }
}
impl RTCReceivedRtpStreamStats {
    pub fn packets_lost(&self) -> i64 {
        self.inner.get("packetsLost").as_::<i64>()
    }

    pub fn set_packets_lost(&mut self, value: i64) {
        self.inner.set("packetsLost", value);
    }
}
impl RTCReceivedRtpStreamStats {
    pub fn jitter(&self) -> f64 {
        self.inner.get("jitter").as_::<f64>()
    }

    pub fn set_jitter(&mut self, value: f64) {
        self.inner.set("jitter", value);
    }
}
