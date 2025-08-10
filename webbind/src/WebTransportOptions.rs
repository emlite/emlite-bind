use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportOptions {
    inner: Any,
}
impl FromVal for WebTransportOptions {
    fn from_val(v: &Any) -> Self {
        WebTransportOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransportOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportOptions> for Any {
    fn from(s: WebTransportOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportOptions> for Any {
    fn from(s: &WebTransportOptions) -> Any {
        s.inner.clone()
    }
}

impl WebTransportOptions {
    pub fn allow_pooling(&self) -> bool {
        self.inner.get("allowPooling").as_::<bool>()
    }

    pub fn set_allow_pooling(&mut self, value: bool) {
        self.inner.set("allowPooling", value);
    }
}
impl WebTransportOptions {
    pub fn require_unreliable(&self) -> bool {
        self.inner.get("requireUnreliable").as_::<bool>()
    }

    pub fn set_require_unreliable(&mut self, value: bool) {
        self.inner.set("requireUnreliable", value);
    }
}
impl WebTransportOptions {
    pub fn server_certificate_hashes(&self) -> TypedArray<WebTransportHash> {
        self.inner
            .get("serverCertificateHashes")
            .as_::<TypedArray<WebTransportHash>>()
    }

    pub fn set_server_certificate_hashes(&mut self, value: &TypedArray<WebTransportHash>) {
        self.inner.set("serverCertificateHashes", value);
    }
}
impl WebTransportOptions {
    pub fn congestion_control(&self) -> WebTransportCongestionControl {
        self.inner
            .get("congestionControl")
            .as_::<WebTransportCongestionControl>()
    }

    pub fn set_congestion_control(&mut self, value: &WebTransportCongestionControl) {
        self.inner.set("congestionControl", value);
    }
}
impl WebTransportOptions {
    pub fn anticipated_concurrent_incoming_unidirectional_streams(&self) -> u16 {
        self.inner
            .get("anticipatedConcurrentIncomingUnidirectionalStreams")
            .as_::<u16>()
    }

    pub fn set_anticipated_concurrent_incoming_unidirectional_streams(&mut self, value: u16) {
        self.inner
            .set("anticipatedConcurrentIncomingUnidirectionalStreams", value);
    }
}
impl WebTransportOptions {
    pub fn anticipated_concurrent_incoming_bidirectional_streams(&self) -> u16 {
        self.inner
            .get("anticipatedConcurrentIncomingBidirectionalStreams")
            .as_::<u16>()
    }

    pub fn set_anticipated_concurrent_incoming_bidirectional_streams(&mut self, value: u16) {
        self.inner
            .set("anticipatedConcurrentIncomingBidirectionalStreams", value);
    }
}
impl WebTransportOptions {
    pub fn protocols(&self) -> TypedArray<JsString> {
        self.inner.get("protocols").as_::<TypedArray<JsString>>()
    }

    pub fn set_protocols(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("protocols", value);
    }
}
