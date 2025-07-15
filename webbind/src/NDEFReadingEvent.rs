use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for NDEFReadingEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NDEFReadingEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&NDEFReadingEvent> for emlite::Val {
    fn from(s: &NDEFReadingEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NDEFReadingEvent);

impl NDEFReadingEvent {
    pub fn new(type_: &str, reading_event_init_dict: &Any) -> NDEFReadingEvent {
        Self {
            inner: emlite::Val::global("NDEFReadingEvent")
                .new(&[type_.into(), reading_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl NDEFReadingEvent {
    pub fn serial_number(&self) -> String {
        self.inner.get("serialNumber").as_::<String>()
    }
}
impl NDEFReadingEvent {
    pub fn message(&self) -> NDEFMessage {
        self.inner.get("message").as_::<NDEFMessage>()
    }
}
