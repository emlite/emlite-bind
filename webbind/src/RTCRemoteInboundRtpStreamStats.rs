use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRemoteInboundRtpStreamStats {
    inner: Any,
}
impl FromVal for RTCRemoteInboundRtpStreamStats {
    fn from_val(v: &Any) -> Self {
        RTCRemoteInboundRtpStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRemoteInboundRtpStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRemoteInboundRtpStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRemoteInboundRtpStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRemoteInboundRtpStreamStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRemoteInboundRtpStreamStats> for Any {
    fn from(s: RTCRemoteInboundRtpStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRemoteInboundRtpStreamStats> for Any {
    fn from(s: &RTCRemoteInboundRtpStreamStats) -> Any {
        s.inner.clone()
    }
}

impl RTCRemoteInboundRtpStreamStats {
    pub fn local_id(&self) -> JsString {
        self.inner.get("localId").as_::<JsString>()
    }

    pub fn set_local_id(&mut self, value: &JsString) {
        self.inner.set("localId", value);
    }
}
impl RTCRemoteInboundRtpStreamStats {
    pub fn round_trip_time(&self) -> f64 {
        self.inner.get("roundTripTime").as_::<f64>()
    }

    pub fn set_round_trip_time(&mut self, value: f64) {
        self.inner.set("roundTripTime", value);
    }
}
impl RTCRemoteInboundRtpStreamStats {
    pub fn total_round_trip_time(&self) -> f64 {
        self.inner.get("totalRoundTripTime").as_::<f64>()
    }

    pub fn set_total_round_trip_time(&mut self, value: f64) {
        self.inner.set("totalRoundTripTime", value);
    }
}
impl RTCRemoteInboundRtpStreamStats {
    pub fn fraction_lost(&self) -> f64 {
        self.inner.get("fractionLost").as_::<f64>()
    }

    pub fn set_fraction_lost(&mut self, value: f64) {
        self.inner.set("fractionLost", value);
    }
}
impl RTCRemoteInboundRtpStreamStats {
    pub fn round_trip_time_measurements(&self) -> u64 {
        self.inner.get("roundTripTimeMeasurements").as_::<u64>()
    }

    pub fn set_round_trip_time_measurements(&mut self, value: u64) {
        self.inner.set("roundTripTimeMeasurements", value);
    }
}
impl RTCRemoteInboundRtpStreamStats {
    pub fn packets_with_bleached_ect1_marking(&self) -> u64 {
        self.inner
            .get("packetsWithBleachedEct1Marking")
            .as_::<u64>()
    }

    pub fn set_packets_with_bleached_ect1_marking(&mut self, value: u64) {
        self.inner.set("packetsWithBleachedEct1Marking", value);
    }
}
