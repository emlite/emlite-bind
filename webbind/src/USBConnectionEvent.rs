use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBConnectionEvent {
    inner: Event,
}
impl FromVal for USBConnectionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        USBConnectionEvent {
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
impl core::ops::Deref for USBConnectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBConnectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBConnectionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBConnectionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBConnectionEvent> for emlite::Val {
    fn from(s: USBConnectionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USBConnectionEvent);

impl USBConnectionEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> USBConnectionEvent {
        Self {
            inner: emlite::Val::global("USBConnectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl USBConnectionEvent {
    pub fn device(&self) -> USBDevice {
        self.inner.get("device").as_::<USBDevice>()
    }
}
