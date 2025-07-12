use super::*;

#[derive(Clone, Debug)]
pub struct RTCSctpTransport {
    inner: EventTarget,
}
impl FromVal for RTCSctpTransport {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSctpTransport {
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
impl std::ops::Deref for RTCSctpTransport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCSctpTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCSctpTransport> for emlite::Val {
    fn from(s: RTCSctpTransport) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCSctpTransport {
    pub fn transport(&self) -> RTCDtlsTransport {
        self.inner.get("transport").as_::<RTCDtlsTransport>()
    }
}
impl RTCSctpTransport {
    pub fn state(&self) -> RTCSctpTransportState {
        self.inner.get("state").as_::<RTCSctpTransportState>()
    }
}
impl RTCSctpTransport {
    pub fn max_message_size(&self) -> f64 {
        self.inner.get("maxMessageSize").as_::<f64>()
    }
}
impl RTCSctpTransport {
    pub fn max_channels(&self) -> u16 {
        self.inner.get("maxChannels").as_::<u16>()
    }
}
impl RTCSctpTransport {
    pub fn onstatechange(&self) -> jsbind::Any {
        self.inner.get("onstatechange").as_::<jsbind::Any>()
    }

    pub fn set_onstatechange(&mut self, value: jsbind::Any) {
        self.inner.set("onstatechange", value);
    }
}
