use super::*;

/// The RTCIceCandidateInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidateInit {
    inner: Any,
}

impl FromVal for RTCIceCandidateInit {
    fn from_val(v: &Any) -> Self {
        RTCIceCandidateInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIceCandidateInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIceCandidateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIceCandidateInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIceCandidateInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCIceCandidateInit> for Any {
    fn from(s: RTCIceCandidateInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIceCandidateInit> for Any {
    fn from(s: &RTCIceCandidateInit) -> Any {
        s.inner.clone()
    }
}

impl RTCIceCandidateInit {
    /// Getter of the `candidate` attribute.
    pub fn candidate(&self) -> JsString {
        self.inner.get("candidate").as_::<JsString>()
    }

    /// Setter of the `candidate` attribute.
    pub fn set_candidate(&mut self, value: &JsString) {
        self.inner.set("candidate", value);
    }
}
impl RTCIceCandidateInit {
    /// Getter of the `sdpMid` attribute.
    pub fn sdp_mid(&self) -> JsString {
        self.inner.get("sdpMid").as_::<JsString>()
    }

    /// Setter of the `sdpMid` attribute.
    pub fn set_sdp_mid(&mut self, value: &JsString) {
        self.inner.set("sdpMid", value);
    }
}
impl RTCIceCandidateInit {
    /// Getter of the `sdpMLineIndex` attribute.
    pub fn sdp_m_line_index(&self) -> u16 {
        self.inner.get("sdpMLineIndex").as_::<u16>()
    }

    /// Setter of the `sdpMLineIndex` attribute.
    pub fn set_sdp_m_line_index(&mut self, value: u16) {
        self.inner.set("sdpMLineIndex", value);
    }
}
impl RTCIceCandidateInit {
    /// Getter of the `usernameFragment` attribute.
    pub fn username_fragment(&self) -> JsString {
        self.inner.get("usernameFragment").as_::<JsString>()
    }

    /// Setter of the `usernameFragment` attribute.
    pub fn set_username_fragment(&mut self, value: &JsString) {
        self.inner.set("usernameFragment", value);
    }
}
