use super::*;




/// The PictureInPictureWindow class.
/// [`PictureInPictureWindow`](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureWindow)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PictureInPictureWindow {
    inner: EventTarget,
}

impl FromVal for PictureInPictureWindow {
    fn from_val(v: &Any) -> Self {
        PictureInPictureWindow { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PictureInPictureWindow {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PictureInPictureWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PictureInPictureWindow {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PictureInPictureWindow {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PictureInPictureWindow> for Any {
    fn from(s: PictureInPictureWindow) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PictureInPictureWindow> for Any {
    fn from(s: &PictureInPictureWindow) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PictureInPictureWindow);


impl PictureInPictureWindow {
    /// Getter of the `width` attribute.
    /// [`PictureInPictureWindow.width`](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureWindow/width)
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }

}
impl PictureInPictureWindow {
    /// Getter of the `height` attribute.
    /// [`PictureInPictureWindow.height`](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureWindow/height)
    pub fn height(&self) -> i32 {
        self.inner.get("height").as_::<i32>()
    }

}
impl PictureInPictureWindow {
    /// Getter of the `onresize` attribute.
    /// [`PictureInPictureWindow.onresize`](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureWindow/onresize)
    pub fn onresize(&self) -> Any {
        self.inner.get("onresize").as_::<Any>()
    }

    /// Setter of the `onresize` attribute.
    /// [`PictureInPictureWindow.onresize`](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureWindow/onresize)
    pub fn set_onresize(&mut self, value: &Any) {
        self.inner.set("onresize", value);
    }
}
