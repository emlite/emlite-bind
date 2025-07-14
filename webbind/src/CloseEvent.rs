use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CloseEvent {
    inner: Event,
}
impl FromVal for CloseEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CloseEvent {
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
impl core::ops::Deref for CloseEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CloseEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CloseEvent> for emlite::Val {
    fn from(s: CloseEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CloseEvent {
    pub fn new0(type_: jsbind::DOMString) -> CloseEvent {
        Self {
            inner: emlite::Val::global("CloseEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> CloseEvent {
        Self {
            inner: emlite::Val::global("CloseEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CloseEvent {
    pub fn was_clean(&self) -> bool {
        self.inner.get("wasClean").as_::<bool>()
    }
}
impl CloseEvent {
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }
}
impl CloseEvent {
    pub fn reason(&self) -> jsbind::USVString {
        self.inner.get("reason").as_::<jsbind::USVString>()
    }
}
