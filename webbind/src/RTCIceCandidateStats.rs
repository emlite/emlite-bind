use super::*;

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
    pub fn transport_id(&self) -> JsString {
        self.inner.get("transportId").as_::<JsString>()
    }

    pub fn set_transport_id(&mut self, value: &JsString) {
        self.inner.set("transportId", value);
    }
}
impl RTCIceCandidateStats {
    pub fn address(&self) -> JsString {
        self.inner.get("address").as_::<JsString>()
    }

    pub fn set_address(&mut self, value: &JsString) {
        self.inner.set("address", value);
    }
}
impl RTCIceCandidateStats {
    pub fn port(&self) -> i32 {
        self.inner.get("port").as_::<i32>()
    }

    pub fn set_port(&mut self, value: i32) {
        self.inner.set("port", value);
    }
}
impl RTCIceCandidateStats {
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl RTCIceCandidateStats {
    pub fn candidate_type(&self) -> RTCIceCandidateType {
        self.inner.get("candidateType").as_::<RTCIceCandidateType>()
    }

    pub fn set_candidate_type(&mut self, value: &RTCIceCandidateType) {
        self.inner.set("candidateType", value);
    }
}
impl RTCIceCandidateStats {
    pub fn priority(&self) -> i32 {
        self.inner.get("priority").as_::<i32>()
    }

    pub fn set_priority(&mut self, value: i32) {
        self.inner.set("priority", value);
    }
}
impl RTCIceCandidateStats {
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl RTCIceCandidateStats {
    pub fn relay_protocol(&self) -> RTCIceServerTransportProtocol {
        self.inner
            .get("relayProtocol")
            .as_::<RTCIceServerTransportProtocol>()
    }

    pub fn set_relay_protocol(&mut self, value: &RTCIceServerTransportProtocol) {
        self.inner.set("relayProtocol", value);
    }
}
impl RTCIceCandidateStats {
    pub fn foundation(&self) -> JsString {
        self.inner.get("foundation").as_::<JsString>()
    }

    pub fn set_foundation(&mut self, value: &JsString) {
        self.inner.set("foundation", value);
    }
}
impl RTCIceCandidateStats {
    pub fn related_address(&self) -> JsString {
        self.inner.get("relatedAddress").as_::<JsString>()
    }

    pub fn set_related_address(&mut self, value: &JsString) {
        self.inner.set("relatedAddress", value);
    }
}
impl RTCIceCandidateStats {
    pub fn related_port(&self) -> i32 {
        self.inner.get("relatedPort").as_::<i32>()
    }

    pub fn set_related_port(&mut self, value: i32) {
        self.inner.set("relatedPort", value);
    }
}
impl RTCIceCandidateStats {
    pub fn username_fragment(&self) -> JsString {
        self.inner.get("usernameFragment").as_::<JsString>()
    }

    pub fn set_username_fragment(&mut self, value: &JsString) {
        self.inner.set("usernameFragment", value);
    }
}
impl RTCIceCandidateStats {
    pub fn tcp_type(&self) -> RTCIceTcpCandidateType {
        self.inner.get("tcpType").as_::<RTCIceTcpCandidateType>()
    }

    pub fn set_tcp_type(&mut self, value: &RTCIceTcpCandidateType) {
        self.inner.set("tcpType", value);
    }
}
