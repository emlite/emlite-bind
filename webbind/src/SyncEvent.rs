use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for SyncEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SyncEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&SyncEvent> for emlite::Val {
    fn from(s: &SyncEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SyncEvent);

impl SyncEvent {
    pub fn new(type_: DOMString, init: Any) -> SyncEvent {
        Self {
            inner: emlite::Val::global("SyncEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl SyncEvent {
    pub fn tag(&self) -> DOMString {
        self.inner.get("tag").as_::<DOMString>()
    }
}
impl SyncEvent {
    pub fn last_chance(&self) -> bool {
        self.inner.get("lastChance").as_::<bool>()
    }
}
