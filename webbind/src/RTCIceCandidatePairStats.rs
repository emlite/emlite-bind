use super::*;




/// The RTCIceCandidatePairStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidatePairStats {
    inner: Any,
}

impl FromVal for RTCIceCandidatePairStats {
    fn from_val(v: &Any) -> Self {
        RTCIceCandidatePairStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIceCandidatePairStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIceCandidatePairStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIceCandidatePairStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIceCandidatePairStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIceCandidatePairStats> for Any {
    fn from(s: RTCIceCandidatePairStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIceCandidatePairStats> for Any {
    fn from(s: &RTCIceCandidatePairStats) -> Any {
        s.inner.clone()
    }
}

impl RTCIceCandidatePairStats {
    /// Getter of the `transportId` attribute.
    pub fn transport_id(&self) -> JsString {
        self.inner.get("transportId").as_::<JsString>()
    }

    /// Setter of the `transportId` attribute.
    pub fn set_transport_id(&mut self, value: &JsString) {
        self.inner.set("transportId", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `localCandidateId` attribute.
    pub fn local_candidate_id(&self) -> JsString {
        self.inner.get("localCandidateId").as_::<JsString>()
    }

    /// Setter of the `localCandidateId` attribute.
    pub fn set_local_candidate_id(&mut self, value: &JsString) {
        self.inner.set("localCandidateId", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `remoteCandidateId` attribute.
    pub fn remote_candidate_id(&self) -> JsString {
        self.inner.get("remoteCandidateId").as_::<JsString>()
    }

    /// Setter of the `remoteCandidateId` attribute.
    pub fn set_remote_candidate_id(&mut self, value: &JsString) {
        self.inner.set("remoteCandidateId", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `state` attribute.
    pub fn state(&self) -> RTCStatsIceCandidatePairState {
        self.inner.get("state").as_::<RTCStatsIceCandidatePairState>()
    }

    /// Setter of the `state` attribute.
    pub fn set_state(&mut self, value: &RTCStatsIceCandidatePairState) {
        self.inner.set("state", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `nominated` attribute.
    pub fn nominated(&self) -> bool {
        self.inner.get("nominated").as_::<bool>()
    }

    /// Setter of the `nominated` attribute.
    pub fn set_nominated(&mut self, value: bool) {
        self.inner.set("nominated", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `packetsSent` attribute.
    pub fn packets_sent(&self) -> u64 {
        self.inner.get("packetsSent").as_::<u64>()
    }

    /// Setter of the `packetsSent` attribute.
    pub fn set_packets_sent(&mut self, value: u64) {
        self.inner.set("packetsSent", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `packetsReceived` attribute.
    pub fn packets_received(&self) -> u64 {
        self.inner.get("packetsReceived").as_::<u64>()
    }

    /// Setter of the `packetsReceived` attribute.
    pub fn set_packets_received(&mut self, value: u64) {
        self.inner.set("packetsReceived", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `bytesSent` attribute.
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    /// Setter of the `bytesSent` attribute.
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `bytesReceived` attribute.
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    /// Setter of the `bytesReceived` attribute.
    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `lastPacketSentTimestamp` attribute.
    pub fn last_packet_sent_timestamp(&self) -> Any {
        self.inner.get("lastPacketSentTimestamp").as_::<Any>()
    }

    /// Setter of the `lastPacketSentTimestamp` attribute.
    pub fn set_last_packet_sent_timestamp(&mut self, value: &Any) {
        self.inner.set("lastPacketSentTimestamp", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `lastPacketReceivedTimestamp` attribute.
    pub fn last_packet_received_timestamp(&self) -> Any {
        self.inner.get("lastPacketReceivedTimestamp").as_::<Any>()
    }

    /// Setter of the `lastPacketReceivedTimestamp` attribute.
    pub fn set_last_packet_received_timestamp(&mut self, value: &Any) {
        self.inner.set("lastPacketReceivedTimestamp", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `totalRoundTripTime` attribute.
    pub fn total_round_trip_time(&self) -> f64 {
        self.inner.get("totalRoundTripTime").as_::<f64>()
    }

    /// Setter of the `totalRoundTripTime` attribute.
    pub fn set_total_round_trip_time(&mut self, value: f64) {
        self.inner.set("totalRoundTripTime", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `currentRoundTripTime` attribute.
    pub fn current_round_trip_time(&self) -> f64 {
        self.inner.get("currentRoundTripTime").as_::<f64>()
    }

    /// Setter of the `currentRoundTripTime` attribute.
    pub fn set_current_round_trip_time(&mut self, value: f64) {
        self.inner.set("currentRoundTripTime", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `availableOutgoingBitrate` attribute.
    pub fn available_outgoing_bitrate(&self) -> f64 {
        self.inner.get("availableOutgoingBitrate").as_::<f64>()
    }

    /// Setter of the `availableOutgoingBitrate` attribute.
    pub fn set_available_outgoing_bitrate(&mut self, value: f64) {
        self.inner.set("availableOutgoingBitrate", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `availableIncomingBitrate` attribute.
    pub fn available_incoming_bitrate(&self) -> f64 {
        self.inner.get("availableIncomingBitrate").as_::<f64>()
    }

    /// Setter of the `availableIncomingBitrate` attribute.
    pub fn set_available_incoming_bitrate(&mut self, value: f64) {
        self.inner.set("availableIncomingBitrate", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `requestsReceived` attribute.
    pub fn requests_received(&self) -> u64 {
        self.inner.get("requestsReceived").as_::<u64>()
    }

    /// Setter of the `requestsReceived` attribute.
    pub fn set_requests_received(&mut self, value: u64) {
        self.inner.set("requestsReceived", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `requestsSent` attribute.
    pub fn requests_sent(&self) -> u64 {
        self.inner.get("requestsSent").as_::<u64>()
    }

    /// Setter of the `requestsSent` attribute.
    pub fn set_requests_sent(&mut self, value: u64) {
        self.inner.set("requestsSent", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `responsesReceived` attribute.
    pub fn responses_received(&self) -> u64 {
        self.inner.get("responsesReceived").as_::<u64>()
    }

    /// Setter of the `responsesReceived` attribute.
    pub fn set_responses_received(&mut self, value: u64) {
        self.inner.set("responsesReceived", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `responsesSent` attribute.
    pub fn responses_sent(&self) -> u64 {
        self.inner.get("responsesSent").as_::<u64>()
    }

    /// Setter of the `responsesSent` attribute.
    pub fn set_responses_sent(&mut self, value: u64) {
        self.inner.set("responsesSent", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `consentRequestsSent` attribute.
    pub fn consent_requests_sent(&self) -> u64 {
        self.inner.get("consentRequestsSent").as_::<u64>()
    }

    /// Setter of the `consentRequestsSent` attribute.
    pub fn set_consent_requests_sent(&mut self, value: u64) {
        self.inner.set("consentRequestsSent", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `packetsDiscardedOnSend` attribute.
    pub fn packets_discarded_on_send(&self) -> u32 {
        self.inner.get("packetsDiscardedOnSend").as_::<u32>()
    }

    /// Setter of the `packetsDiscardedOnSend` attribute.
    pub fn set_packets_discarded_on_send(&mut self, value: u32) {
        self.inner.set("packetsDiscardedOnSend", value);
    }
}
impl RTCIceCandidatePairStats {
    /// Getter of the `bytesDiscardedOnSend` attribute.
    pub fn bytes_discarded_on_send(&self) -> u64 {
        self.inner.get("bytesDiscardedOnSend").as_::<u64>()
    }

    /// Setter of the `bytesDiscardedOnSend` attribute.
    pub fn set_bytes_discarded_on_send(&mut self, value: u64) {
        self.inner.set("bytesDiscardedOnSend", value);
    }
}
