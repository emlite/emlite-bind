use super::*;

#[derive(Clone, Debug)]
pub struct PictureInPictureEvent {
    inner: Event,
}
impl FromVal for PictureInPictureEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PictureInPictureEvent {
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
impl std::ops::Deref for PictureInPictureEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PictureInPictureEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PictureInPictureEvent> for emlite::Val {
    fn from(s: PictureInPictureEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PictureInPictureEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> PictureInPictureEvent {
        Self {
            inner: emlite::Val::global("PictureInPictureEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PictureInPictureEvent {
    pub fn picture_in_picture_window(&self) -> PictureInPictureWindow {
        self.inner
            .get("pictureInPictureWindow")
            .as_::<PictureInPictureWindow>()
    }
}
