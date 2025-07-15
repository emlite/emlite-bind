use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushEvent {
    inner: ExtendableEvent,
}
impl FromVal for PushEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PushEvent { inner: ExtendableEvent::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PushEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PushEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PushEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PushEvent> for emlite::Val {
    fn from(s: PushEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PushEvent);



impl PushEvent {
    pub fn new0(type_: DOMString) -> PushEvent {
        Self {
            inner: emlite::Val::global("PushEvent").new(&[type_.into()]).as_::<ExtendableEvent>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> PushEvent {
        Self {
            inner: emlite::Val::global("PushEvent").new(&[type_.into(), event_init_dict.into()]).as_::<ExtendableEvent>(),
        }
    }

}
impl PushEvent {
    pub fn data(&self) -> PushMessageData {
        self.inner.get("data").as_::<PushMessageData>()
    }

}
