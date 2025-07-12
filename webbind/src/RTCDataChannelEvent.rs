use super::*;

#[derive(Clone, Debug)]
pub struct RTCDataChannelEvent {
    inner: Event,
}
impl FromVal for RTCDataChannelEvent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDataChannelEvent {
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
impl std::ops::Deref for RTCDataChannelEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCDataChannelEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCDataChannelEvent> for emlite::Val {
    fn from(s: RTCDataChannelEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCDataChannelEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> RTCDataChannelEvent {
        Self {
            inner: emlite::Val::global("RTCDataChannelEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCDataChannelEvent {
    pub fn channel(&self) -> RTCDataChannel {
        self.inner.get("channel").as_::<RTCDataChannel>()
    }
}
