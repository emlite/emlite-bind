use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PromiseRejectionEvent {
    inner: Event,
}
impl FromVal for PromiseRejectionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PromiseRejectionEvent {
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
impl core::ops::Deref for PromiseRejectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PromiseRejectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PromiseRejectionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PromiseRejectionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PromiseRejectionEvent> for emlite::Val {
    fn from(s: PromiseRejectionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PromiseRejectionEvent> for emlite::Val {
    fn from(s: &PromiseRejectionEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PromiseRejectionEvent);

impl PromiseRejectionEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> PromiseRejectionEvent {
        Self {
            inner: emlite::Val::global("PromiseRejectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PromiseRejectionEvent {
    pub fn promise(&self) -> Object {
        self.inner.get("promise").as_::<Object>()
    }
}
impl PromiseRejectionEvent {
    pub fn reason(&self) -> Any {
        self.inner.get("reason").as_::<Any>()
    }
}
