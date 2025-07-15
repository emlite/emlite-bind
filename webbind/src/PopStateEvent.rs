use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PopStateEvent {
    inner: Event,
}
impl FromVal for PopStateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PopStateEvent {
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
impl core::ops::Deref for PopStateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PopStateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PopStateEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PopStateEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PopStateEvent> for emlite::Val {
    fn from(s: PopStateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PopStateEvent);

impl PopStateEvent {
    pub fn new0(type_: DOMString) -> PopStateEvent {
        Self {
            inner: emlite::Val::global("PopStateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> PopStateEvent {
        Self {
            inner: emlite::Val::global("PopStateEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PopStateEvent {
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }
}
impl PopStateEvent {
    pub fn has_ua_visual_transition(&self) -> bool {
        self.inner.get("hasUAVisualTransition").as_::<bool>()
    }
}
