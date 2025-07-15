use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OfflineAudioCompletionEvent {
    inner: Event,
}
impl FromVal for OfflineAudioCompletionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        OfflineAudioCompletionEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OfflineAudioCompletionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OfflineAudioCompletionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OfflineAudioCompletionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OfflineAudioCompletionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<OfflineAudioCompletionEvent> for emlite::Val {
    fn from(s: OfflineAudioCompletionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(OfflineAudioCompletionEvent);



impl OfflineAudioCompletionEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> OfflineAudioCompletionEvent {
        Self {
            inner: emlite::Val::global("OfflineAudioCompletionEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl OfflineAudioCompletionEvent {
    pub fn rendered_buffer(&self) -> AudioBuffer {
        self.inner.get("renderedBuffer").as_::<AudioBuffer>()
    }

}
