use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SyncEvent {
    inner: ExtendableEvent,
}
impl FromVal for SyncEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SyncEvent {
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
impl core::ops::Deref for SyncEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SyncEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SyncEvent> for emlite::Val {
    fn from(s: SyncEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SyncEvent {
    pub fn new(type_: jsbind::DOMString, init: jsbind::Any) -> SyncEvent {
        Self {
            inner: emlite::Val::global("SyncEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl SyncEvent {
    pub fn tag(&self) -> jsbind::DOMString {
        self.inner.get("tag").as_::<jsbind::DOMString>()
    }
}
impl SyncEvent {
    pub fn last_chance(&self) -> bool {
        self.inner.get("lastChance").as_::<bool>()
    }
}
