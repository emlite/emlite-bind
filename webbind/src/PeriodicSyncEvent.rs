use super::*;

#[derive(Clone, Debug)]
pub struct PeriodicSyncEvent {
    inner: ExtendableEvent,
}
impl FromVal for PeriodicSyncEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PeriodicSyncEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PeriodicSyncEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PeriodicSyncEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PeriodicSyncEvent> for emlite::Val {
    fn from(s: PeriodicSyncEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PeriodicSyncEvent {
    pub fn new(type_: jsbind::DOMString, init: jsbind::Any) -> PeriodicSyncEvent {
        Self {
            inner: emlite::Val::global("PeriodicSyncEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl PeriodicSyncEvent {
    pub fn tag(&self) -> jsbind::DOMString {
        self.inner.get("tag").as_::<jsbind::DOMString>()
    }
}
