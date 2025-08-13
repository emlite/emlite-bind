use super::*;




/// The RTCIceCandidateStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidateStats {
    inner: Any,
}

impl FromVal for RTCIceCandidateStats {
    fn from_val(v: &Any) -> Self {
        RTCIceCandidateStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIceCandidateStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIceCandidateStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIceCandidateStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIceCandidateStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIceCandidateStats> for Any {
    fn from(s: RTCIceCandidateStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIceCandidateStats> for Any {
    fn from(s: &RTCIceCandidateStats) -> Any {
        s.inner.clone()
    }
}

impl RTCIceCandidateStats {
    /// Getter of the `transportId` attribute.
    pub fn transport_id(&self) -> JsString {
        self.inner.get("transportId").as_::<JsString>()
    }

    /// Setter of the `transportId` attribute.
    pub fn set_transport_id(&mut self, value: &JsString) {
        self.inner.set("transportId", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `address` attribute.
    pub fn address(&self) -> JsString {
        self.inner.get("address").as_::<JsString>()
    }

    /// Setter of the `address` attribute.
    pub fn set_address(&mut self, value: &JsString) {
        self.inner.set("address", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `port` attribute.
    pub fn port(&self) -> i32 {
        self.inner.get("port").as_::<i32>()
    }

    /// Setter of the `port` attribute.
    pub fn set_port(&mut self, value: i32) {
        self.inner.set("port", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `candidateType` attribute.
    pub fn candidate_type(&self) -> RTCIceCandidateType {
        self.inner.get("candidateType").as_::<RTCIceCandidateType>()
    }

    /// Setter of the `candidateType` attribute.
    pub fn set_candidate_type(&mut self, value: &RTCIceCandidateType) {
        self.inner.set("candidateType", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `priority` attribute.
    pub fn priority(&self) -> i32 {
        self.inner.get("priority").as_::<i32>()
    }

    /// Setter of the `priority` attribute.
    pub fn set_priority(&mut self, value: i32) {
        self.inner.set("priority", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `relayProtocol` attribute.
    pub fn relay_protocol(&self) -> RTCIceServerTransportProtocol {
        self.inner.get("relayProtocol").as_::<RTCIceServerTransportProtocol>()
    }

    /// Setter of the `relayProtocol` attribute.
    pub fn set_relay_protocol(&mut self, value: &RTCIceServerTransportProtocol) {
        self.inner.set("relayProtocol", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `foundation` attribute.
    pub fn foundation(&self) -> JsString {
        self.inner.get("foundation").as_::<JsString>()
    }

    /// Setter of the `foundation` attribute.
    pub fn set_foundation(&mut self, value: &JsString) {
        self.inner.set("foundation", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `relatedAddress` attribute.
    pub fn related_address(&self) -> JsString {
        self.inner.get("relatedAddress").as_::<JsString>()
    }

    /// Setter of the `relatedAddress` attribute.
    pub fn set_related_address(&mut self, value: &JsString) {
        self.inner.set("relatedAddress", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `relatedPort` attribute.
    pub fn related_port(&self) -> i32 {
        self.inner.get("relatedPort").as_::<i32>()
    }

    /// Setter of the `relatedPort` attribute.
    pub fn set_related_port(&mut self, value: i32) {
        self.inner.set("relatedPort", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `usernameFragment` attribute.
    pub fn username_fragment(&self) -> JsString {
        self.inner.get("usernameFragment").as_::<JsString>()
    }

    /// Setter of the `usernameFragment` attribute.
    pub fn set_username_fragment(&mut self, value: &JsString) {
        self.inner.set("usernameFragment", value);
    }
}
impl RTCIceCandidateStats {
    /// Getter of the `tcpType` attribute.
    pub fn tcp_type(&self) -> RTCIceTcpCandidateType {
        self.inner.get("tcpType").as_::<RTCIceTcpCandidateType>()
    }

    /// Setter of the `tcpType` attribute.
    pub fn set_tcp_type(&mut self, value: &RTCIceTcpCandidateType) {
        self.inner.set("tcpType", value);
    }
}
