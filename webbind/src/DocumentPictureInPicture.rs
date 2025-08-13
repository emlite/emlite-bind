use super::*;




/// The DocumentPictureInPicture class.
/// [`DocumentPictureInPicture`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPicture {
    inner: EventTarget,
}

impl FromVal for DocumentPictureInPicture {
    fn from_val(v: &Any) -> Self {
        DocumentPictureInPicture { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DocumentPictureInPicture {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DocumentPictureInPicture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DocumentPictureInPicture {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DocumentPictureInPicture {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DocumentPictureInPicture> for Any {
    fn from(s: DocumentPictureInPicture) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DocumentPictureInPicture> for Any {
    fn from(s: &DocumentPictureInPicture) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DocumentPictureInPicture);


impl DocumentPictureInPicture {
    /// The requestWindow method.
    /// [`DocumentPictureInPicture.requestWindow`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/requestWindow)
    pub fn request_window0(&self, ) -> Promise<Window> {
        self.inner.call("requestWindow", &[]).as_::<Promise<Window>>()
    }
    /// The requestWindow method.
    /// [`DocumentPictureInPicture.requestWindow`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/requestWindow)
    pub fn request_window1(&self, options: &DocumentPictureInPictureOptions) -> Promise<Window> {
        self.inner.call("requestWindow", &[options.into(), ]).as_::<Promise<Window>>()
    }
}
impl DocumentPictureInPicture {
    /// Getter of the `window` attribute.
    /// [`DocumentPictureInPicture.window`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/window)
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }

}
impl DocumentPictureInPicture {
    /// Getter of the `onenter` attribute.
    /// [`DocumentPictureInPicture.onenter`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/onenter)
    pub fn onenter(&self) -> Any {
        self.inner.get("onenter").as_::<Any>()
    }

    /// Setter of the `onenter` attribute.
    /// [`DocumentPictureInPicture.onenter`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/onenter)
    pub fn set_onenter(&mut self, value: &Any) {
        self.inner.set("onenter", value);
    }
}
