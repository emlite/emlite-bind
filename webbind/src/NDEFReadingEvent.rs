use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NDEFReadingEvent {
    inner: Event,
}
impl FromVal for NDEFReadingEvent {
    fn from_val(v: &emlite::Val) -> Self {
        NDEFReadingEvent {
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
impl core::ops::Deref for NDEFReadingEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFReadingEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NDEFReadingEvent> for emlite::Val {
    fn from(s: NDEFReadingEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NDEFReadingEvent {
    pub fn new(type_: jsbind::DOMString, reading_event_init_dict: jsbind::Any) -> NDEFReadingEvent {
        Self {
            inner: emlite::Val::global("NDEFReadingEvent")
                .new(&[type_.into(), reading_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl NDEFReadingEvent {
    pub fn serial_number(&self) -> jsbind::DOMString {
        self.inner.get("serialNumber").as_::<jsbind::DOMString>()
    }
}
impl NDEFReadingEvent {
    pub fn message(&self) -> NDEFMessage {
        self.inner.get("message").as_::<NDEFMessage>()
    }
}
