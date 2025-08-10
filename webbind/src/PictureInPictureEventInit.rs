use super::*;

/// The PictureInPictureEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PictureInPictureEventInit {
    inner: Any,
}

impl FromVal for PictureInPictureEventInit {
    fn from_val(v: &Any) -> Self {
        PictureInPictureEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PictureInPictureEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PictureInPictureEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PictureInPictureEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PictureInPictureEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PictureInPictureEventInit> for Any {
    fn from(s: PictureInPictureEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PictureInPictureEventInit> for Any {
    fn from(s: &PictureInPictureEventInit) -> Any {
        s.inner.clone()
    }
}

impl PictureInPictureEventInit {
    /// Getter of the `pictureInPictureWindow` attribute.
    pub fn picture_in_picture_window(&self) -> PictureInPictureWindow {
        self.inner
            .get("pictureInPictureWindow")
            .as_::<PictureInPictureWindow>()
    }

    /// Setter of the `pictureInPictureWindow` attribute.
    pub fn set_picture_in_picture_window(&mut self, value: &PictureInPictureWindow) {
        self.inner.set("pictureInPictureWindow", value);
    }
}
