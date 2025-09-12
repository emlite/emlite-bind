use super::*;

/// The ImageCapture class.
/// [`ImageCapture`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageCapture {
    inner: Any,
}

impl FromVal for ImageCapture {
    fn from_val(v: &Any) -> Self {
        ImageCapture {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageCapture {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageCapture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageCapture {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageCapture {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ImageCapture> for Any {
    fn from(s: ImageCapture) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageCapture> for Any {
    fn from(s: &ImageCapture) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ImageCapture);

impl ImageCapture {
    /// Getter of the `track` attribute.
    /// [`ImageCapture.track`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}

impl ImageCapture {
    /// The `new ImageCapture(..)` constructor, creating a new ImageCapture instance
    pub fn new(video_track: &MediaStreamTrack) -> ImageCapture {
        Self {
            inner: Any::global("ImageCapture")
                .new(&[video_track.into()])
                .as_::<Any>(),
        }
    }
}

impl ImageCapture {
    /// The takePhoto method.
    /// [`ImageCapture.takePhoto`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/takePhoto)
    pub fn take_photo(&self) -> Promise<Blob> {
        self.inner.call("takePhoto", &[]).as_::<Promise<Blob>>()
    }
}
impl ImageCapture {
    /// The takePhoto method.
    /// [`ImageCapture.takePhoto`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/takePhoto)
    pub fn take_photo_with_photo_settings(&self, photo_settings: &PhotoSettings) -> Promise<Blob> {
        self.inner
            .call("takePhoto", &[photo_settings.into()])
            .as_::<Promise<Blob>>()
    }
}
impl ImageCapture {
    /// The getPhotoCapabilities method.
    /// [`ImageCapture.getPhotoCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/getPhotoCapabilities)
    pub fn get_photo_capabilities(&self) -> Promise<PhotoCapabilities> {
        self.inner
            .call("getPhotoCapabilities", &[])
            .as_::<Promise<PhotoCapabilities>>()
    }
}
impl ImageCapture {
    /// The getPhotoSettings method.
    /// [`ImageCapture.getPhotoSettings`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/getPhotoSettings)
    pub fn get_photo_settings(&self) -> Promise<PhotoSettings> {
        self.inner
            .call("getPhotoSettings", &[])
            .as_::<Promise<PhotoSettings>>()
    }
}
impl ImageCapture {
    /// The grabFrame method.
    /// [`ImageCapture.grabFrame`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/grabFrame)
    pub fn grab_frame(&self) -> Promise<ImageBitmap> {
        self.inner
            .call("grabFrame", &[])
            .as_::<Promise<ImageBitmap>>()
    }
}
