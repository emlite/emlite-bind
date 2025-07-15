use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCTrackEvent {
    inner: Event,
}
impl FromVal for RTCTrackEvent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCTrackEvent {
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
impl core::ops::Deref for RTCTrackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCTrackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCTrackEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCTrackEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCTrackEvent> for emlite::Val {
    fn from(s: RTCTrackEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCTrackEvent);

impl RTCTrackEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> RTCTrackEvent {
        Self {
            inner: emlite::Val::global("RTCTrackEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCTrackEvent {
    pub fn receiver(&self) -> RTCRtpReceiver {
        self.inner.get("receiver").as_::<RTCRtpReceiver>()
    }
}
impl RTCTrackEvent {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
impl RTCTrackEvent {
    pub fn streams(&self) -> FrozenArray<MediaStream> {
        self.inner.get("streams").as_::<FrozenArray<MediaStream>>()
    }
}
impl RTCTrackEvent {
    pub fn transceiver(&self) -> RTCRtpTransceiver {
        self.inner.get("transceiver").as_::<RTCRtpTransceiver>()
    }
}
