use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPictureEvent {
    inner: Event,
}
impl FromVal for DocumentPictureInPictureEvent {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentPictureInPictureEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DocumentPictureInPictureEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentPictureInPictureEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DocumentPictureInPictureEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DocumentPictureInPictureEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<DocumentPictureInPictureEvent> for emlite::Val {
    fn from(s: DocumentPictureInPictureEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DocumentPictureInPictureEvent);



impl DocumentPictureInPictureEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> DocumentPictureInPictureEvent {
        Self {
            inner: emlite::Val::global("DocumentPictureInPictureEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl DocumentPictureInPictureEvent {
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }

}
