use super::*;

/// The RTCLocalIceCandidateInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCLocalIceCandidateInit {
    inner: Any,
}

impl FromVal for RTCLocalIceCandidateInit {
    fn from_val(v: &Any) -> Self {
        RTCLocalIceCandidateInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCLocalIceCandidateInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCLocalIceCandidateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCLocalIceCandidateInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCLocalIceCandidateInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCLocalIceCandidateInit> for Any {
    fn from(s: RTCLocalIceCandidateInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCLocalIceCandidateInit> for Any {
    fn from(s: &RTCLocalIceCandidateInit) -> Any {
        s.inner.clone()
    }
}

impl RTCLocalIceCandidateInit {
    /// Getter of the `relayProtocol` attribute.
    pub fn relay_protocol(&self) -> RTCIceServerTransportProtocol {
        self.inner
            .get("relayProtocol")
            .as_::<RTCIceServerTransportProtocol>()
    }

    /// Setter of the `relayProtocol` attribute.
    pub fn set_relay_protocol(&mut self, value: &RTCIceServerTransportProtocol) {
        self.inner.set("relayProtocol", value);
    }
}
impl RTCLocalIceCandidateInit {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
