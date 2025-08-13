use super::*;




/// The PictureInPictureEvent class.
/// [`PictureInPictureEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PictureInPictureEvent {
    inner: Event,
}

impl FromVal for PictureInPictureEvent {
    fn from_val(v: &Any) -> Self {
        PictureInPictureEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for PictureInPictureEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PictureInPictureEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PictureInPictureEvent> for Any {
    fn from(s: PictureInPictureEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PictureInPictureEvent> for Any {
    fn from(s: &PictureInPictureEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PictureInPictureEvent);



impl PictureInPictureEvent {
    /// The `new PictureInPictureEvent(..)` constructor, creating a new PictureInPictureEvent instance
    pub fn new(type_: &JsString, event_init_dict: &PictureInPictureEventInit) -> PictureInPictureEvent {
        Self {
            inner: Any::global("PictureInPictureEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl PictureInPictureEvent {
    /// Getter of the `pictureInPictureWindow` attribute.
    /// [`PictureInPictureEvent.pictureInPictureWindow`](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent/pictureInPictureWindow)
    pub fn picture_in_picture_window(&self) -> PictureInPictureWindow {
        self.inner.get("pictureInPictureWindow").as_::<PictureInPictureWindow>()
    }

}
