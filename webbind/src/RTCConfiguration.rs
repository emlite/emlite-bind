use super::*;

/// The RTCConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCConfiguration {
    inner: Any,
}

impl FromVal for RTCConfiguration {
    fn from_val(v: &Any) -> Self {
        RTCConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCConfiguration> for Any {
    fn from(s: RTCConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCConfiguration> for Any {
    fn from(s: &RTCConfiguration) -> Any {
        s.inner.clone()
    }
}

impl RTCConfiguration {
    /// Getter of the `iceServers` attribute.
    pub fn ice_servers(&self) -> TypedArray<RTCIceServer> {
        self.inner
            .get("iceServers")
            .as_::<TypedArray<RTCIceServer>>()
    }

    /// Setter of the `iceServers` attribute.
    pub fn set_ice_servers(&mut self, value: &TypedArray<RTCIceServer>) {
        self.inner.set("iceServers", value);
    }
}
impl RTCConfiguration {
    /// Getter of the `iceTransportPolicy` attribute.
    pub fn ice_transport_policy(&self) -> RTCIceTransportPolicy {
        self.inner
            .get("iceTransportPolicy")
            .as_::<RTCIceTransportPolicy>()
    }

    /// Setter of the `iceTransportPolicy` attribute.
    pub fn set_ice_transport_policy(&mut self, value: &RTCIceTransportPolicy) {
        self.inner.set("iceTransportPolicy", value);
    }
}
impl RTCConfiguration {
    /// Getter of the `bundlePolicy` attribute.
    pub fn bundle_policy(&self) -> RTCBundlePolicy {
        self.inner.get("bundlePolicy").as_::<RTCBundlePolicy>()
    }

    /// Setter of the `bundlePolicy` attribute.
    pub fn set_bundle_policy(&mut self, value: &RTCBundlePolicy) {
        self.inner.set("bundlePolicy", value);
    }
}
impl RTCConfiguration {
    /// Getter of the `rtcpMuxPolicy` attribute.
    pub fn rtcp_mux_policy(&self) -> RTCRtcpMuxPolicy {
        self.inner.get("rtcpMuxPolicy").as_::<RTCRtcpMuxPolicy>()
    }

    /// Setter of the `rtcpMuxPolicy` attribute.
    pub fn set_rtcp_mux_policy(&mut self, value: &RTCRtcpMuxPolicy) {
        self.inner.set("rtcpMuxPolicy", value);
    }
}
impl RTCConfiguration {
    /// Getter of the `certificates` attribute.
    pub fn certificates(&self) -> TypedArray<RTCCertificate> {
        self.inner
            .get("certificates")
            .as_::<TypedArray<RTCCertificate>>()
    }

    /// Setter of the `certificates` attribute.
    pub fn set_certificates(&mut self, value: &TypedArray<RTCCertificate>) {
        self.inner.set("certificates", value);
    }
}
impl RTCConfiguration {
    /// Getter of the `iceCandidatePoolSize` attribute.
    pub fn ice_candidate_pool_size(&self) -> u8 {
        self.inner.get("iceCandidatePoolSize").as_::<u8>()
    }

    /// Setter of the `iceCandidatePoolSize` attribute.
    pub fn set_ice_candidate_pool_size(&mut self, value: u8) {
        self.inner.set("iceCandidatePoolSize", value);
    }
}
