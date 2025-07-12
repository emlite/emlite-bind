use super::*;

#[derive(Clone, Debug)]
pub struct BeforeUnloadEvent {
    inner: Event,
}
impl FromVal for BeforeUnloadEvent {
    fn from_val(v: &emlite::Val) -> Self {
        BeforeUnloadEvent {
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
impl std::ops::Deref for BeforeUnloadEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BeforeUnloadEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BeforeUnloadEvent> for emlite::Val {
    fn from(s: BeforeUnloadEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BeforeUnloadEvent {
    pub fn return_value(&self) -> jsbind::DOMString {
        self.inner.get("returnValue").as_::<jsbind::DOMString>()
    }

    pub fn set_return_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("returnValue", value);
    }
}
