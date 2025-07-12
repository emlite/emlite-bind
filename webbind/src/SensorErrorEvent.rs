use super::*;

#[derive(Clone, Debug)]
pub struct SensorErrorEvent {
    inner: Event,
}
impl FromVal for SensorErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SensorErrorEvent {
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
impl std::ops::Deref for SensorErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SensorErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SensorErrorEvent> for emlite::Val {
    fn from(s: SensorErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SensorErrorEvent {
    pub fn new(type_: jsbind::DOMString, error_event_init_dict: jsbind::Any) -> SensorErrorEvent {
        Self {
            inner: emlite::Val::global("SensorErrorEvent")
                .new(&[type_.into(), error_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SensorErrorEvent {
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }
}
