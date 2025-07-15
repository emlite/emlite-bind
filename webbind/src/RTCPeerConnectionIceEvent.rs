use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnectionIceEvent {
    inner: Event,
}
impl FromVal for RTCPeerConnectionIceEvent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCPeerConnectionIceEvent {
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
impl core::ops::Deref for RTCPeerConnectionIceEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCPeerConnectionIceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCPeerConnectionIceEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCPeerConnectionIceEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCPeerConnectionIceEvent> for emlite::Val {
    fn from(s: RTCPeerConnectionIceEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCPeerConnectionIceEvent> for emlite::Val {
    fn from(s: &RTCPeerConnectionIceEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCPeerConnectionIceEvent);

impl RTCPeerConnectionIceEvent {
    pub fn new0(type_: DOMString) -> RTCPeerConnectionIceEvent {
        Self {
            inner: emlite::Val::global("RTCPeerConnectionIceEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> RTCPeerConnectionIceEvent {
        Self {
            inner: emlite::Val::global("RTCPeerConnectionIceEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCPeerConnectionIceEvent {
    pub fn candidate(&self) -> RTCIceCandidate {
        self.inner.get("candidate").as_::<RTCIceCandidate>()
    }
}
impl RTCPeerConnectionIceEvent {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }
}
