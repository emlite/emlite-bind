use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for SensorErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SensorErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SensorErrorEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SensorErrorEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SensorErrorEvent> for emlite::Val {
    fn from(s: SensorErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SensorErrorEvent);

impl SensorErrorEvent {
    pub fn new(type_: DOMString, error_event_init_dict: Any) -> SensorErrorEvent {
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
