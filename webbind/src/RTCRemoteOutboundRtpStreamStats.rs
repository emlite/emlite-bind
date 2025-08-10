use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRemoteOutboundRtpStreamStats {
    inner: Any,
}
impl FromVal for RTCRemoteOutboundRtpStreamStats {
    fn from_val(v: &Any) -> Self {
        RTCRemoteOutboundRtpStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRemoteOutboundRtpStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRemoteOutboundRtpStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRemoteOutboundRtpStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRemoteOutboundRtpStreamStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRemoteOutboundRtpStreamStats> for Any {
    fn from(s: RTCRemoteOutboundRtpStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRemoteOutboundRtpStreamStats> for Any {
    fn from(s: &RTCRemoteOutboundRtpStreamStats) -> Any {
        s.inner.clone()
    }
}

impl RTCRemoteOutboundRtpStreamStats {
    pub fn local_id(&self) -> JsString {
        self.inner.get("localId").as_::<JsString>()
    }

    pub fn set_local_id(&mut self, value: &JsString) {
        self.inner.set("localId", value);
    }
}
impl RTCRemoteOutboundRtpStreamStats {
    pub fn remote_timestamp(&self) -> Any {
        self.inner.get("remoteTimestamp").as_::<Any>()
    }

    pub fn set_remote_timestamp(&mut self, value: &Any) {
        self.inner.set("remoteTimestamp", value);
    }
}
impl RTCRemoteOutboundRtpStreamStats {
    pub fn reports_sent(&self) -> u64 {
        self.inner.get("reportsSent").as_::<u64>()
    }

    pub fn set_reports_sent(&mut self, value: u64) {
        self.inner.set("reportsSent", value);
    }
}
impl RTCRemoteOutboundRtpStreamStats {
    pub fn round_trip_time(&self) -> f64 {
        self.inner.get("roundTripTime").as_::<f64>()
    }

    pub fn set_round_trip_time(&mut self, value: f64) {
        self.inner.set("roundTripTime", value);
    }
}
impl RTCRemoteOutboundRtpStreamStats {
    pub fn total_round_trip_time(&self) -> f64 {
        self.inner.get("totalRoundTripTime").as_::<f64>()
    }

    pub fn set_total_round_trip_time(&mut self, value: f64) {
        self.inner.set("totalRoundTripTime", value);
    }
}
impl RTCRemoteOutboundRtpStreamStats {
    pub fn round_trip_time_measurements(&self) -> u64 {
        self.inner.get("roundTripTimeMeasurements").as_::<u64>()
    }

    pub fn set_round_trip_time_measurements(&mut self, value: u64) {
        self.inner.set("roundTripTimeMeasurements", value);
    }
}
