use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLVideoElement {
    inner: HTMLMediaElement,
}
impl FromVal for HTMLVideoElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLVideoElement {
            inner: HTMLMediaElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for HTMLVideoElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLVideoElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLVideoElement> for emlite::Val {
    fn from(s: HTMLVideoElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLVideoElement> for emlite::Val {
    fn from(s: &HTMLVideoElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLVideoElement);

impl HTMLVideoElement {
    pub fn new() -> HTMLVideoElement {
        Self {
            inner: emlite::Val::global("HTMLVideoElement")
                .new(&[])
                .as_::<HTMLMediaElement>(),
        }
    }
}
impl HTMLVideoElement {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLVideoElement {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLVideoElement {
    pub fn video_width(&self) -> u32 {
        self.inner.get("videoWidth").as_::<u32>()
    }
}
impl HTMLVideoElement {
    pub fn video_height(&self) -> u32 {
        self.inner.get("videoHeight").as_::<u32>()
    }
}
impl HTMLVideoElement {
    pub fn poster(&self) -> String {
        self.inner.get("poster").as_::<String>()
    }

    pub fn set_poster(&mut self, value: &str) {
        self.inner.set("poster", value);
    }
}
impl HTMLVideoElement {
    pub fn plays_inline(&self) -> bool {
        self.inner.get("playsInline").as_::<bool>()
    }

    pub fn set_plays_inline(&mut self, value: bool) {
        self.inner.set("playsInline", value);
    }
}
impl HTMLVideoElement {
    pub fn get_video_playback_quality(&self) -> VideoPlaybackQuality {
        self.inner
            .call("getVideoPlaybackQuality", &[])
            .as_::<VideoPlaybackQuality>()
    }
}
impl HTMLVideoElement {
    pub fn request_picture_in_picture(&self) -> Promise {
        self.inner
            .call("requestPictureInPicture", &[])
            .as_::<Promise>()
    }
}
impl HTMLVideoElement {
    pub fn onenterpictureinpicture(&self) -> Any {
        self.inner.get("onenterpictureinpicture").as_::<Any>()
    }

    pub fn set_onenterpictureinpicture(&mut self, value: &Any) {
        self.inner.set("onenterpictureinpicture", value);
    }
}
impl HTMLVideoElement {
    pub fn onleavepictureinpicture(&self) -> Any {
        self.inner.get("onleavepictureinpicture").as_::<Any>()
    }

    pub fn set_onleavepictureinpicture(&mut self, value: &Any) {
        self.inner.set("onleavepictureinpicture", value);
    }
}
impl HTMLVideoElement {
    pub fn disable_picture_in_picture(&self) -> bool {
        self.inner.get("disablePictureInPicture").as_::<bool>()
    }

    pub fn set_disable_picture_in_picture(&mut self, value: bool) {
        self.inner.set("disablePictureInPicture", value);
    }
}
impl HTMLVideoElement {
    pub fn request_video_frame_callback(&self, callback: &Function) -> u32 {
        self.inner
            .call("requestVideoFrameCallback", &[callback.into()])
            .as_::<u32>()
    }
}
impl HTMLVideoElement {
    pub fn cancel_video_frame_callback(&self, handle: u32) -> Undefined {
        self.inner
            .call("cancelVideoFrameCallback", &[handle.into()])
            .as_::<Undefined>()
    }
}
