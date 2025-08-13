use super::*;




/// The HTMLVideoElement class.
/// [`HTMLVideoElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLVideoElement {
    inner: HTMLMediaElement,
}

impl FromVal for HTMLVideoElement {
    fn from_val(v: &Any) -> Self {
        HTMLVideoElement { inner: HTMLMediaElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLVideoElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLVideoElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLVideoElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLVideoElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLVideoElement> for Any {
    fn from(s: HTMLVideoElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLVideoElement> for Any {
    fn from(s: &HTMLVideoElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLVideoElement);



impl HTMLVideoElement {
    /// The `new HTMLVideoElement(..)` constructor, creating a new HTMLVideoElement instance
    pub fn new() -> HTMLVideoElement {
        Self {
            inner: Any::global("HTMLVideoElement").new(&[]).as_::<HTMLMediaElement>(),
        }
    }

}
impl HTMLVideoElement {
    /// Getter of the `width` attribute.
    /// [`HTMLVideoElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLVideoElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)
    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLVideoElement {
    /// Getter of the `height` attribute.
    /// [`HTMLVideoElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLVideoElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)
    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLVideoElement {
    /// Getter of the `videoWidth` attribute.
    /// [`HTMLVideoElement.videoWidth`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoWidth)
    pub fn video_width(&self) -> u32 {
        self.inner.get("videoWidth").as_::<u32>()
    }

}
impl HTMLVideoElement {
    /// Getter of the `videoHeight` attribute.
    /// [`HTMLVideoElement.videoHeight`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoHeight)
    pub fn video_height(&self) -> u32 {
        self.inner.get("videoHeight").as_::<u32>()
    }

}
impl HTMLVideoElement {
    /// Getter of the `poster` attribute.
    /// [`HTMLVideoElement.poster`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)
    pub fn poster(&self) -> JsString {
        self.inner.get("poster").as_::<JsString>()
    }

    /// Setter of the `poster` attribute.
    /// [`HTMLVideoElement.poster`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)
    pub fn set_poster(&mut self, value: &JsString) {
        self.inner.set("poster", value);
    }
}
impl HTMLVideoElement {
    /// Getter of the `playsInline` attribute.
    /// [`HTMLVideoElement.playsInline`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/playsInline)
    pub fn plays_inline(&self) -> bool {
        self.inner.get("playsInline").as_::<bool>()
    }

    /// Setter of the `playsInline` attribute.
    /// [`HTMLVideoElement.playsInline`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/playsInline)
    pub fn set_plays_inline(&mut self, value: bool) {
        self.inner.set("playsInline", value);
    }
}
impl HTMLVideoElement {
    /// The getVideoPlaybackQuality method.
    /// [`HTMLVideoElement.getVideoPlaybackQuality`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/getVideoPlaybackQuality)
    pub fn get_video_playback_quality(&self, ) -> VideoPlaybackQuality {
        self.inner.call("getVideoPlaybackQuality", &[]).as_::<VideoPlaybackQuality>()
    }
}
impl HTMLVideoElement {
    /// The requestPictureInPicture method.
    /// [`HTMLVideoElement.requestPictureInPicture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/requestPictureInPicture)
    pub fn request_picture_in_picture(&self, ) -> Promise<PictureInPictureWindow> {
        self.inner.call("requestPictureInPicture", &[]).as_::<Promise<PictureInPictureWindow>>()
    }
}
impl HTMLVideoElement {
    /// Getter of the `onenterpictureinpicture` attribute.
    /// [`HTMLVideoElement.onenterpictureinpicture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onenterpictureinpicture)
    pub fn onenterpictureinpicture(&self) -> Any {
        self.inner.get("onenterpictureinpicture").as_::<Any>()
    }

    /// Setter of the `onenterpictureinpicture` attribute.
    /// [`HTMLVideoElement.onenterpictureinpicture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onenterpictureinpicture)
    pub fn set_onenterpictureinpicture(&mut self, value: &Any) {
        self.inner.set("onenterpictureinpicture", value);
    }
}
impl HTMLVideoElement {
    /// Getter of the `onleavepictureinpicture` attribute.
    /// [`HTMLVideoElement.onleavepictureinpicture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onleavepictureinpicture)
    pub fn onleavepictureinpicture(&self) -> Any {
        self.inner.get("onleavepictureinpicture").as_::<Any>()
    }

    /// Setter of the `onleavepictureinpicture` attribute.
    /// [`HTMLVideoElement.onleavepictureinpicture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onleavepictureinpicture)
    pub fn set_onleavepictureinpicture(&mut self, value: &Any) {
        self.inner.set("onleavepictureinpicture", value);
    }
}
impl HTMLVideoElement {
    /// Getter of the `disablePictureInPicture` attribute.
    /// [`HTMLVideoElement.disablePictureInPicture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/disablePictureInPicture)
    pub fn disable_picture_in_picture(&self) -> bool {
        self.inner.get("disablePictureInPicture").as_::<bool>()
    }

    /// Setter of the `disablePictureInPicture` attribute.
    /// [`HTMLVideoElement.disablePictureInPicture`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/disablePictureInPicture)
    pub fn set_disable_picture_in_picture(&mut self, value: bool) {
        self.inner.set("disablePictureInPicture", value);
    }
}
impl HTMLVideoElement {
    /// The requestVideoFrameCallback method.
    /// [`HTMLVideoElement.requestVideoFrameCallback`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/requestVideoFrameCallback)
    pub fn request_video_frame_callback(&self, callback: &Function) -> u32 {
        self.inner.call("requestVideoFrameCallback", &[callback.into(), ]).as_::<u32>()
    }
}
impl HTMLVideoElement {
    /// The cancelVideoFrameCallback method.
    /// [`HTMLVideoElement.cancelVideoFrameCallback`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/cancelVideoFrameCallback)
    pub fn cancel_video_frame_callback(&self, handle: u32) -> Undefined {
        self.inner.call("cancelVideoFrameCallback", &[handle.into(), ]).as_::<Undefined>()
    }
}
