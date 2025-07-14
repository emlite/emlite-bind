use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PresentationConnectionCloseEvent {
    inner: Event,
}
impl FromVal for PresentationConnectionCloseEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationConnectionCloseEvent {
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
impl core::ops::Deref for PresentationConnectionCloseEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationConnectionCloseEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PresentationConnectionCloseEvent> for emlite::Val {
    fn from(s: PresentationConnectionCloseEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PresentationConnectionCloseEvent {
    pub fn new(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> PresentationConnectionCloseEvent {
        Self {
            inner: emlite::Val::global("PresentationConnectionCloseEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PresentationConnectionCloseEvent {
    pub fn reason(&self) -> PresentationConnectionCloseReason {
        self.inner
            .get("reason")
            .as_::<PresentationConnectionCloseReason>()
    }
}
impl PresentationConnectionCloseEvent {
    pub fn message(&self) -> jsbind::DOMString {
        self.inner.get("message").as_::<jsbind::DOMString>()
    }
}
