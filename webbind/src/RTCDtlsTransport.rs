use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RTCDtlsTransport {
    inner: EventTarget,
}
impl FromVal for RTCDtlsTransport {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDtlsTransport {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDtlsTransport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDtlsTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCDtlsTransport> for emlite::Val {
    fn from(s: RTCDtlsTransport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCDtlsTransport {
    pub fn ice_transport(&self) -> RTCIceTransport {
        self.inner.get("iceTransport").as_::<RTCIceTransport>()
    }
}
impl RTCDtlsTransport {
    pub fn state(&self) -> RTCDtlsTransportState {
        self.inner.get("state").as_::<RTCDtlsTransportState>()
    }
}
impl RTCDtlsTransport {
    pub fn get_remote_certificates(&self) -> jsbind::Sequence<jsbind::ArrayBuffer> {
        self.inner
            .call("getRemoteCertificates", &[])
            .as_::<jsbind::Sequence<jsbind::ArrayBuffer>>()
    }
}
impl RTCDtlsTransport {
    pub fn onstatechange(&self) -> jsbind::Any {
        self.inner.get("onstatechange").as_::<jsbind::Any>()
    }

    pub fn set_onstatechange(&mut self, value: jsbind::Any) {
        self.inner.set("onstatechange", value);
    }
}
impl RTCDtlsTransport {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
