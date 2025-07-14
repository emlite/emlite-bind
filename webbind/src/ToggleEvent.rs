use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToggleEvent {
    inner: Event,
}
impl FromVal for ToggleEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ToggleEvent {
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
impl core::ops::Deref for ToggleEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ToggleEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ToggleEvent> for emlite::Val {
    fn from(s: ToggleEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ToggleEvent {
    pub fn new0(type_: jsbind::DOMString) -> ToggleEvent {
        Self {
            inner: emlite::Val::global("ToggleEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> ToggleEvent {
        Self {
            inner: emlite::Val::global("ToggleEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ToggleEvent {
    pub fn old_state(&self) -> jsbind::DOMString {
        self.inner.get("oldState").as_::<jsbind::DOMString>()
    }
}
impl ToggleEvent {
    pub fn new_state(&self) -> jsbind::DOMString {
        self.inner.get("newState").as_::<jsbind::DOMString>()
    }
}
impl ToggleEvent {
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
