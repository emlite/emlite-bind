use super::*;

#[derive(Clone, Debug)]
pub struct RTCPeerConnectionIceErrorEvent {
    inner: Event,
}
impl FromVal for RTCPeerConnectionIceErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCPeerConnectionIceErrorEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCPeerConnectionIceErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCPeerConnectionIceErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCPeerConnectionIceErrorEvent> for emlite::Val {
    fn from(s: RTCPeerConnectionIceErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCPeerConnectionIceErrorEvent {
    pub fn new(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> RTCPeerConnectionIceErrorEvent {
        Self {
            inner: emlite::Val::global("RTCPeerConnectionIceErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn address(&self) -> jsbind::DOMString {
        self.inner.get("address").as_::<jsbind::DOMString>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn port(&self) -> u16 {
        self.inner.get("port").as_::<u16>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn error_code(&self) -> u16 {
        self.inner.get("errorCode").as_::<u16>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn error_text(&self) -> jsbind::USVString {
        self.inner.get("errorText").as_::<jsbind::USVString>()
    }
}
