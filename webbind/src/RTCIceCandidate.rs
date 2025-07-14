use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCIceCandidate {
    inner: emlite::Val,
}
impl FromVal for RTCIceCandidate {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceCandidate {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceCandidate {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceCandidate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceCandidate> for emlite::Val {
    fn from(s: RTCIceCandidate) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceCandidate {
    pub fn new0() -> RTCIceCandidate {
        Self {
            inner: emlite::Val::global("RTCIceCandidate")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(candidate_init_dict: jsbind::Any) -> RTCIceCandidate {
        Self {
            inner: emlite::Val::global("RTCIceCandidate")
                .new(&[candidate_init_dict.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl RTCIceCandidate {
    pub fn candidate(&self) -> jsbind::DOMString {
        self.inner.get("candidate").as_::<jsbind::DOMString>()
    }
}
impl RTCIceCandidate {
    pub fn sdp_mid(&self) -> jsbind::DOMString {
        self.inner.get("sdpMid").as_::<jsbind::DOMString>()
    }
}
impl RTCIceCandidate {
    pub fn sdp_m_line_index(&self) -> u16 {
        self.inner.get("sdpMLineIndex").as_::<u16>()
    }
}
impl RTCIceCandidate {
    pub fn foundation(&self) -> jsbind::DOMString {
        self.inner.get("foundation").as_::<jsbind::DOMString>()
    }
}
impl RTCIceCandidate {
    pub fn component(&self) -> RTCIceComponent {
        self.inner.get("component").as_::<RTCIceComponent>()
    }
}
impl RTCIceCandidate {
    pub fn priority(&self) -> u32 {
        self.inner.get("priority").as_::<u32>()
    }
}
impl RTCIceCandidate {
    pub fn address(&self) -> jsbind::DOMString {
        self.inner.get("address").as_::<jsbind::DOMString>()
    }
}
impl RTCIceCandidate {
    pub fn protocol(&self) -> RTCIceProtocol {
        self.inner.get("protocol").as_::<RTCIceProtocol>()
    }
}
impl RTCIceCandidate {
    pub fn port(&self) -> u16 {
        self.inner.get("port").as_::<u16>()
    }
}
impl RTCIceCandidate {
    pub fn type_(&self) -> RTCIceCandidateType {
        self.inner.get("type").as_::<RTCIceCandidateType>()
    }
}
impl RTCIceCandidate {
    pub fn tcp_type(&self) -> RTCIceTcpCandidateType {
        self.inner.get("tcpType").as_::<RTCIceTcpCandidateType>()
    }
}
impl RTCIceCandidate {
    pub fn related_address(&self) -> jsbind::DOMString {
        self.inner.get("relatedAddress").as_::<jsbind::DOMString>()
    }
}
impl RTCIceCandidate {
    pub fn related_port(&self) -> u16 {
        self.inner.get("relatedPort").as_::<u16>()
    }
}
impl RTCIceCandidate {
    pub fn username_fragment(&self) -> jsbind::DOMString {
        self.inner
            .get("usernameFragment")
            .as_::<jsbind::DOMString>()
    }
}
impl RTCIceCandidate {
    pub fn relay_protocol(&self) -> RTCIceServerTransportProtocol {
        self.inner
            .get("relayProtocol")
            .as_::<RTCIceServerTransportProtocol>()
    }
}
impl RTCIceCandidate {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl RTCIceCandidate {
    pub fn to_json(&self) -> RTCIceCandidateInit {
        self.inner.call("toJSON", &[]).as_::<RTCIceCandidateInit>()
    }
}
