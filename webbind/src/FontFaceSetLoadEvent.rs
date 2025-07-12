use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for FontFaceSetLoadEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FontFaceSetLoadEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FontFaceSetLoadEvent> for emlite::Val {
    fn from(s: FontFaceSetLoadEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FontFaceSetLoadEvent {
    pub fn new0(type_: jsbind::CSSOMString) -> FontFaceSetLoadEvent {
        Self {
            inner: emlite::Val::global("FontFaceSetLoadEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::CSSOMString, event_init_dict: jsbind::Any) -> FontFaceSetLoadEvent {
        Self {
            inner: emlite::Val::global("FontFaceSetLoadEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl FontFaceSetLoadEvent {
    pub fn fontfaces(&self) -> jsbind::FrozenArray<FontFace> {
        self.inner
            .get("fontfaces")
            .as_::<jsbind::FrozenArray<FontFace>>()
    }
}
