use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for ToggleEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ToggleEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(ToggleEvent);

impl ToggleEvent {
    pub fn new0(type_: DOMString) -> ToggleEvent {
        Self {
            inner: emlite::Val::global("ToggleEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> ToggleEvent {
        Self {
            inner: emlite::Val::global("ToggleEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ToggleEvent {
    pub fn old_state(&self) -> DOMString {
        self.inner.get("oldState").as_::<DOMString>()
    }
}
impl ToggleEvent {
    pub fn new_state(&self) -> DOMString {
        self.inner.get("newState").as_::<DOMString>()
    }
}
impl ToggleEvent {
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
