use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PromiseRejectionEvent {
    inner: Event,
}
impl FromVal for PromiseRejectionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PromiseRejectionEvent {
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
impl core::ops::Deref for PromiseRejectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PromiseRejectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PromiseRejectionEvent> for emlite::Val {
    fn from(s: PromiseRejectionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PromiseRejectionEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> PromiseRejectionEvent {
        Self {
            inner: emlite::Val::global("PromiseRejectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PromiseRejectionEvent {
    pub fn promise(&self) -> jsbind::Object {
        self.inner.get("promise").as_::<jsbind::Object>()
    }
}
impl PromiseRejectionEvent {
    pub fn reason(&self) -> jsbind::Any {
        self.inner.get("reason").as_::<jsbind::Any>()
    }
}
