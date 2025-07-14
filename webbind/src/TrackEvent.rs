use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrackEvent {
    inner: Event,
}
impl FromVal for TrackEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TrackEvent {
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
impl core::ops::Deref for TrackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TrackEvent> for emlite::Val {
    fn from(s: TrackEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TrackEvent {
    pub fn new0(type_: jsbind::DOMString) -> TrackEvent {
        Self {
            inner: emlite::Val::global("TrackEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> TrackEvent {
        Self {
            inner: emlite::Val::global("TrackEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl TrackEvent {
    pub fn track(&self) -> jsbind::Any {
        self.inner.get("track").as_::<jsbind::Any>()
    }
}
