use super::*;




/// The RTCTransportStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCTransportStats {
    inner: Any,
}

impl FromVal for RTCTransportStats {
    fn from_val(v: &Any) -> Self {
        RTCTransportStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCTransportStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCTransportStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCTransportStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCTransportStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCTransportStats> for Any {
    fn from(s: RTCTransportStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCTransportStats> for Any {
    fn from(s: &RTCTransportStats) -> Any {
        s.inner.clone()
    }
}

impl RTCTransportStats {
    /// Getter of the `packetsSent` attribute.
    pub fn packets_sent(&self) -> u64 {
        self.inner.get("packetsSent").as_::<u64>()
    }

    /// Setter of the `packetsSent` attribute.
    pub fn set_packets_sent(&mut self, value: u64) {
        self.inner.set("packetsSent", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `packetsReceived` attribute.
    pub fn packets_received(&self) -> u64 {
        self.inner.get("packetsReceived").as_::<u64>()
    }

    /// Setter of the `packetsReceived` attribute.
    pub fn set_packets_received(&mut self, value: u64) {
        self.inner.set("packetsReceived", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `bytesSent` attribute.
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    /// Setter of the `bytesSent` attribute.
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `bytesReceived` attribute.
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    /// Setter of the `bytesReceived` attribute.
    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `iceRole` attribute.
    pub fn ice_role(&self) -> RTCIceRole {
        self.inner.get("iceRole").as_::<RTCIceRole>()
    }

    /// Setter of the `iceRole` attribute.
    pub fn set_ice_role(&mut self, value: &RTCIceRole) {
        self.inner.set("iceRole", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `iceLocalUsernameFragment` attribute.
    pub fn ice_local_username_fragment(&self) -> JsString {
        self.inner.get("iceLocalUsernameFragment").as_::<JsString>()
    }

    /// Setter of the `iceLocalUsernameFragment` attribute.
    pub fn set_ice_local_username_fragment(&mut self, value: &JsString) {
        self.inner.set("iceLocalUsernameFragment", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `dtlsState` attribute.
    pub fn dtls_state(&self) -> RTCDtlsTransportState {
        self.inner.get("dtlsState").as_::<RTCDtlsTransportState>()
    }

    /// Setter of the `dtlsState` attribute.
    pub fn set_dtls_state(&mut self, value: &RTCDtlsTransportState) {
        self.inner.set("dtlsState", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `iceState` attribute.
    pub fn ice_state(&self) -> RTCIceTransportState {
        self.inner.get("iceState").as_::<RTCIceTransportState>()
    }

    /// Setter of the `iceState` attribute.
    pub fn set_ice_state(&mut self, value: &RTCIceTransportState) {
        self.inner.set("iceState", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `selectedCandidatePairId` attribute.
    pub fn selected_candidate_pair_id(&self) -> JsString {
        self.inner.get("selectedCandidatePairId").as_::<JsString>()
    }

    /// Setter of the `selectedCandidatePairId` attribute.
    pub fn set_selected_candidate_pair_id(&mut self, value: &JsString) {
        self.inner.set("selectedCandidatePairId", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `localCertificateId` attribute.
    pub fn local_certificate_id(&self) -> JsString {
        self.inner.get("localCertificateId").as_::<JsString>()
    }

    /// Setter of the `localCertificateId` attribute.
    pub fn set_local_certificate_id(&mut self, value: &JsString) {
        self.inner.set("localCertificateId", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `remoteCertificateId` attribute.
    pub fn remote_certificate_id(&self) -> JsString {
        self.inner.get("remoteCertificateId").as_::<JsString>()
    }

    /// Setter of the `remoteCertificateId` attribute.
    pub fn set_remote_certificate_id(&mut self, value: &JsString) {
        self.inner.set("remoteCertificateId", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `tlsVersion` attribute.
    pub fn tls_version(&self) -> JsString {
        self.inner.get("tlsVersion").as_::<JsString>()
    }

    /// Setter of the `tlsVersion` attribute.
    pub fn set_tls_version(&mut self, value: &JsString) {
        self.inner.set("tlsVersion", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `dtlsCipher` attribute.
    pub fn dtls_cipher(&self) -> JsString {
        self.inner.get("dtlsCipher").as_::<JsString>()
    }

    /// Setter of the `dtlsCipher` attribute.
    pub fn set_dtls_cipher(&mut self, value: &JsString) {
        self.inner.set("dtlsCipher", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `dtlsRole` attribute.
    pub fn dtls_role(&self) -> RTCDtlsRole {
        self.inner.get("dtlsRole").as_::<RTCDtlsRole>()
    }

    /// Setter of the `dtlsRole` attribute.
    pub fn set_dtls_role(&mut self, value: &RTCDtlsRole) {
        self.inner.set("dtlsRole", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `srtpCipher` attribute.
    pub fn srtp_cipher(&self) -> JsString {
        self.inner.get("srtpCipher").as_::<JsString>()
    }

    /// Setter of the `srtpCipher` attribute.
    pub fn set_srtp_cipher(&mut self, value: &JsString) {
        self.inner.set("srtpCipher", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `selectedCandidatePairChanges` attribute.
    pub fn selected_candidate_pair_changes(&self) -> u32 {
        self.inner.get("selectedCandidatePairChanges").as_::<u32>()
    }

    /// Setter of the `selectedCandidatePairChanges` attribute.
    pub fn set_selected_candidate_pair_changes(&mut self, value: u32) {
        self.inner.set("selectedCandidatePairChanges", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `ccfbMessagesSent` attribute.
    pub fn ccfb_messages_sent(&self) -> u32 {
        self.inner.get("ccfbMessagesSent").as_::<u32>()
    }

    /// Setter of the `ccfbMessagesSent` attribute.
    pub fn set_ccfb_messages_sent(&mut self, value: u32) {
        self.inner.set("ccfbMessagesSent", value);
    }
}
impl RTCTransportStats {
    /// Getter of the `ccfbMessagesReceived` attribute.
    pub fn ccfb_messages_received(&self) -> u32 {
        self.inner.get("ccfbMessagesReceived").as_::<u32>()
    }

    /// Setter of the `ccfbMessagesReceived` attribute.
    pub fn set_ccfb_messages_received(&mut self, value: u32) {
        self.inner.set("ccfbMessagesReceived", value);
    }
}
