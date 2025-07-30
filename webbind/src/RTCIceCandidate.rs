use super::*;

/// The RTCIceCandidate class.
/// [`RTCIceCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidate {
    inner: Any,
}
impl FromVal for RTCIceCandidate {
    fn from_val(v: &Any) -> Self {
        RTCIceCandidate {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceCandidate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceCandidate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCIceCandidate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCIceCandidate {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCIceCandidate> for Any {
    fn from(s: RTCIceCandidate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCIceCandidate> for Any {
    fn from(s: &RTCIceCandidate) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCIceCandidate);

impl RTCIceCandidate {
    /// The `new RTCIceCandidate(..)` constructor, creating a new RTCIceCandidate instance
    pub fn new0() -> RTCIceCandidate {
        Self {
            inner: Any::global("RTCIceCandidate").new(&[]).as_::<Any>(),
        }
    }

    /// The `new RTCIceCandidate(..)` constructor, creating a new RTCIceCandidate instance
    pub fn new1(candidate_init_dict: &Any) -> RTCIceCandidate {
        Self {
            inner: Any::global("RTCIceCandidate")
                .new(&[candidate_init_dict.into()])
                .as_::<Any>(),
        }
    }
}
impl RTCIceCandidate {
    /// Getter of the `candidate` attribute.
    /// [`RTCIceCandidate.candidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)
    pub fn candidate(&self) -> JsString {
        self.inner.get("candidate").as_::<JsString>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `sdpMid` attribute.
    /// [`RTCIceCandidate.sdpMid`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)
    pub fn sdp_mid(&self) -> JsString {
        self.inner.get("sdpMid").as_::<JsString>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `sdpMLineIndex` attribute.
    /// [`RTCIceCandidate.sdpMLineIndex`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)
    pub fn sdp_m_line_index(&self) -> u16 {
        self.inner.get("sdpMLineIndex").as_::<u16>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `foundation` attribute.
    /// [`RTCIceCandidate.foundation`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/foundation)
    pub fn foundation(&self) -> JsString {
        self.inner.get("foundation").as_::<JsString>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `component` attribute.
    /// [`RTCIceCandidate.component`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/component)
    pub fn component(&self) -> RTCIceComponent {
        self.inner.get("component").as_::<RTCIceComponent>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `priority` attribute.
    /// [`RTCIceCandidate.priority`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/priority)
    pub fn priority(&self) -> u32 {
        self.inner.get("priority").as_::<u32>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `address` attribute.
    /// [`RTCIceCandidate.address`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/address)
    pub fn address(&self) -> JsString {
        self.inner.get("address").as_::<JsString>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `protocol` attribute.
    /// [`RTCIceCandidate.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/protocol)
    pub fn protocol(&self) -> RTCIceProtocol {
        self.inner.get("protocol").as_::<RTCIceProtocol>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `port` attribute.
    /// [`RTCIceCandidate.port`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/port)
    pub fn port(&self) -> u16 {
        self.inner.get("port").as_::<u16>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `type` attribute.
    /// [`RTCIceCandidate.type`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/type)
    pub fn type_(&self) -> RTCIceCandidateType {
        self.inner.get("type").as_::<RTCIceCandidateType>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `tcpType` attribute.
    /// [`RTCIceCandidate.tcpType`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/tcpType)
    pub fn tcp_type(&self) -> RTCIceTcpCandidateType {
        self.inner.get("tcpType").as_::<RTCIceTcpCandidateType>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `relatedAddress` attribute.
    /// [`RTCIceCandidate.relatedAddress`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/relatedAddress)
    pub fn related_address(&self) -> JsString {
        self.inner.get("relatedAddress").as_::<JsString>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `relatedPort` attribute.
    /// [`RTCIceCandidate.relatedPort`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/relatedPort)
    pub fn related_port(&self) -> u16 {
        self.inner.get("relatedPort").as_::<u16>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `usernameFragment` attribute.
    /// [`RTCIceCandidate.usernameFragment`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/usernameFragment)
    pub fn username_fragment(&self) -> JsString {
        self.inner.get("usernameFragment").as_::<JsString>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `relayProtocol` attribute.
    /// [`RTCIceCandidate.relayProtocol`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/relayProtocol)
    pub fn relay_protocol(&self) -> RTCIceServerTransportProtocol {
        self.inner
            .get("relayProtocol")
            .as_::<RTCIceServerTransportProtocol>()
    }
}
impl RTCIceCandidate {
    /// Getter of the `url` attribute.
    /// [`RTCIceCandidate.url`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl RTCIceCandidate {
    /// The toJSON method.
    /// [`RTCIceCandidate.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/toJSON)
    pub fn to_json(&self) -> RTCIceCandidateInit {
        self.inner.call("toJSON", &[]).as_::<RTCIceCandidateInit>()
    }
}
