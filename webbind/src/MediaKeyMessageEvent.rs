use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MediaKeyMessageEvent {
    inner: Event,
}
impl FromVal for MediaKeyMessageEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeyMessageEvent {
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
impl core::ops::Deref for MediaKeyMessageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeyMessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeyMessageEvent> for emlite::Val {
    fn from(s: MediaKeyMessageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeyMessageEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> MediaKeyMessageEvent {
        Self {
            inner: emlite::Val::global("MediaKeyMessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MediaKeyMessageEvent {
    pub fn message_type(&self) -> MediaKeyMessageType {
        self.inner.get("messageType").as_::<MediaKeyMessageType>()
    }
}
impl MediaKeyMessageEvent {
    pub fn message(&self) -> jsbind::ArrayBuffer {
        self.inner.get("message").as_::<jsbind::ArrayBuffer>()
    }
}
