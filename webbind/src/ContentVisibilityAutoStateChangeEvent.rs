use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentVisibilityAutoStateChangeEvent {
    inner: Event,
}
impl FromVal for ContentVisibilityAutoStateChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ContentVisibilityAutoStateChangeEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ContentVisibilityAutoStateChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContentVisibilityAutoStateChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ContentVisibilityAutoStateChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ContentVisibilityAutoStateChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<ContentVisibilityAutoStateChangeEvent> for emlite::Val {
    fn from(s: ContentVisibilityAutoStateChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ContentVisibilityAutoStateChangeEvent);



impl ContentVisibilityAutoStateChangeEvent {
    pub fn new0(type_: DOMString) -> ContentVisibilityAutoStateChangeEvent {
        Self {
            inner: emlite::Val::global("ContentVisibilityAutoStateChangeEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> ContentVisibilityAutoStateChangeEvent {
        Self {
            inner: emlite::Val::global("ContentVisibilityAutoStateChangeEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl ContentVisibilityAutoStateChangeEvent {
    pub fn skipped(&self) -> bool {
        self.inner.get("skipped").as_::<bool>()
    }

}
