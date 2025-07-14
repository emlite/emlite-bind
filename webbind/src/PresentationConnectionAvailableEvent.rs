use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnectionAvailableEvent {
    inner: Event,
}
impl FromVal for PresentationConnectionAvailableEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationConnectionAvailableEvent {
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
impl core::ops::Deref for PresentationConnectionAvailableEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationConnectionAvailableEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PresentationConnectionAvailableEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PresentationConnectionAvailableEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PresentationConnectionAvailableEvent> for emlite::Val {
    fn from(s: PresentationConnectionAvailableEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PresentationConnectionAvailableEvent);

impl PresentationConnectionAvailableEvent {
    pub fn new(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> PresentationConnectionAvailableEvent {
        Self {
            inner: emlite::Val::global("PresentationConnectionAvailableEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PresentationConnectionAvailableEvent {
    pub fn connection(&self) -> PresentationConnection {
        self.inner.get("connection").as_::<PresentationConnection>()
    }
}
