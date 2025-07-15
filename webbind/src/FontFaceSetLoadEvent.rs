use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceSetLoadEvent {
    inner: Event,
}
impl FromVal for FontFaceSetLoadEvent {
    fn from_val(v: &emlite::Val) -> Self {
        FontFaceSetLoadEvent {
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
impl core::ops::Deref for FontFaceSetLoadEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFaceSetLoadEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FontFaceSetLoadEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FontFaceSetLoadEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FontFaceSetLoadEvent> for emlite::Val {
    fn from(s: FontFaceSetLoadEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FontFaceSetLoadEvent);

impl FontFaceSetLoadEvent {
    pub fn new0(type_: CSSOMString) -> FontFaceSetLoadEvent {
        Self {
            inner: emlite::Val::global("FontFaceSetLoadEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: CSSOMString, event_init_dict: Any) -> FontFaceSetLoadEvent {
        Self {
            inner: emlite::Val::global("FontFaceSetLoadEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl FontFaceSetLoadEvent {
    pub fn fontfaces(&self) -> FrozenArray<FontFace> {
        self.inner.get("fontfaces").as_::<FrozenArray<FontFace>>()
    }
}
