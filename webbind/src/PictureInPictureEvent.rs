use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for PictureInPictureEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PictureInPictureEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PictureInPictureEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PictureInPictureEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PictureInPictureEvent> for emlite::Val {
    fn from(s: PictureInPictureEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PictureInPictureEvent> for emlite::Val {
    fn from(s: &PictureInPictureEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PictureInPictureEvent);

impl PictureInPictureEvent {
    pub fn new(type_: &str, event_init_dict: &Any) -> PictureInPictureEvent {
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
