use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FocusEvent {
    inner: UIEvent,
}
impl FromVal for FocusEvent {
    fn from_val(v: &emlite::Val) -> Self {
        FocusEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FocusEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FocusEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FocusEvent> for emlite::Val {
    fn from(s: FocusEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FocusEvent {
    pub fn new0(type_: jsbind::DOMString) -> FocusEvent {
        Self {
            inner: emlite::Val::global("FocusEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> FocusEvent {
        Self {
            inner: emlite::Val::global("FocusEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl FocusEvent {
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }
}
