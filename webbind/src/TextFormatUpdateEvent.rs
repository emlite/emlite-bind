use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextFormatUpdateEvent {
    inner: Event,
}
impl FromVal for TextFormatUpdateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TextFormatUpdateEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextFormatUpdateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextFormatUpdateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TextFormatUpdateEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextFormatUpdateEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<TextFormatUpdateEvent> for emlite::Val {
    fn from(s: TextFormatUpdateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TextFormatUpdateEvent);



impl TextFormatUpdateEvent {
    pub fn new0(type_: DOMString) -> TextFormatUpdateEvent {
        Self {
            inner: emlite::Val::global("TextFormatUpdateEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, options: Any) -> TextFormatUpdateEvent {
        Self {
            inner: emlite::Val::global("TextFormatUpdateEvent").new(&[type_.into(), options.into()]).as_::<Event>(),
        }
    }

}
impl TextFormatUpdateEvent {
    pub fn get_text_formats(&self, ) -> Sequence<TextFormat> {
        self.inner.call("getTextFormats", &[]).as_::<Sequence<TextFormat>>()
    }

}
