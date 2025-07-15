use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCErrorEvent {
    inner: Event,
}
impl FromVal for RTCErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCErrorEvent {
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
impl core::ops::Deref for RTCErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCErrorEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCErrorEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCErrorEvent> for emlite::Val {
    fn from(s: RTCErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCErrorEvent);

impl RTCErrorEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> RTCErrorEvent {
        Self {
            inner: emlite::Val::global("RTCErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCErrorEvent {
    pub fn error(&self) -> RTCError {
        self.inner.get("error").as_::<RTCError>()
    }
}
