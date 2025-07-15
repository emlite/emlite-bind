use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for RTCPeerConnectionIceErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCPeerConnectionIceErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCPeerConnectionIceErrorEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCPeerConnectionIceErrorEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCPeerConnectionIceErrorEvent> for emlite::Val {
    fn from(s: RTCPeerConnectionIceErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCPeerConnectionIceErrorEvent> for emlite::Val {
    fn from(s: &RTCPeerConnectionIceErrorEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCPeerConnectionIceErrorEvent);

impl RTCPeerConnectionIceErrorEvent {
    pub fn new(type_: &str, event_init_dict: &Any) -> RTCPeerConnectionIceErrorEvent {
        Self {
            inner: emlite::Val::global("RTCPeerConnectionIceErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn address(&self) -> String {
        self.inner.get("address").as_::<String>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn port(&self) -> u16 {
        self.inner.get("port").as_::<u16>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn error_code(&self) -> u16 {
        self.inner.get("errorCode").as_::<u16>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    pub fn error_text(&self) -> String {
        self.inner.get("errorText").as_::<String>()
    }
}
