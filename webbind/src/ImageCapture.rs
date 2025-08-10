use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhotoSettings {
    inner: Any,
}
impl FromVal for PhotoSettings {
    fn from_val(v: &Any) -> Self {
        PhotoSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PhotoSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PhotoSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PhotoSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PhotoSettings {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PhotoSettings> for Any {
    fn from(s: PhotoSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PhotoSettings> for Any {
    fn from(s: &PhotoSettings) -> Any {
        s.inner.clone()
    }
}

impl PhotoSettings {
    pub fn fill_light_mode(&self) -> FillLightMode {
        self.inner.get("fillLightMode").as_::<FillLightMode>()
    }

    pub fn set_fill_light_mode(&mut self, value: &FillLightMode) {
        self.inner.set("fillLightMode", value);
    }
}
impl PhotoSettings {
    pub fn image_height(&self) -> f64 {
        self.inner.get("imageHeight").as_::<f64>()
    }

    pub fn set_image_height(&mut self, value: f64) {
        self.inner.set("imageHeight", value);
    }
}
impl PhotoSettings {
    pub fn image_width(&self) -> f64 {
        self.inner.get("imageWidth").as_::<f64>()
    }

    pub fn set_image_width(&mut self, value: f64) {
        self.inner.set("imageWidth", value);
    }
}
impl PhotoSettings {
    pub fn red_eye_reduction(&self) -> bool {
        self.inner.get("redEyeReduction").as_::<bool>()
    }

    pub fn set_red_eye_reduction(&mut self, value: bool) {
        self.inner.set("redEyeReduction", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhotoCapabilities {
    inner: Any,
}
impl FromVal for PhotoCapabilities {
    fn from_val(v: &Any) -> Self {
        PhotoCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PhotoCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PhotoCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PhotoCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PhotoCapabilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PhotoCapabilities> for Any {
    fn from(s: PhotoCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PhotoCapabilities> for Any {
    fn from(s: &PhotoCapabilities) -> Any {
        s.inner.clone()
    }
}

impl PhotoCapabilities {
    pub fn red_eye_reduction(&self) -> RedEyeReduction {
        self.inner.get("redEyeReduction").as_::<RedEyeReduction>()
    }

    pub fn set_red_eye_reduction(&mut self, value: &RedEyeReduction) {
        self.inner.set("redEyeReduction", value);
    }
}
impl PhotoCapabilities {
    pub fn image_height(&self) -> MediaSettingsRange {
        self.inner.get("imageHeight").as_::<MediaSettingsRange>()
    }

    pub fn set_image_height(&mut self, value: &MediaSettingsRange) {
        self.inner.set("imageHeight", value);
    }
}
impl PhotoCapabilities {
    pub fn image_width(&self) -> MediaSettingsRange {
        self.inner.get("imageWidth").as_::<MediaSettingsRange>()
    }

    pub fn set_image_width(&mut self, value: &MediaSettingsRange) {
        self.inner.set("imageWidth", value);
    }
}
impl PhotoCapabilities {
    pub fn fill_light_mode(&self) -> TypedArray<FillLightMode> {
        self.inner
            .get("fillLightMode")
            .as_::<TypedArray<FillLightMode>>()
    }

    pub fn set_fill_light_mode(&mut self, value: &TypedArray<FillLightMode>) {
        self.inner.set("fillLightMode", value);
    }
}
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
    pub fn take_photo0(&self) -> Promise<Blob> {
        self.inner.call("takePhoto", &[]).as_::<Promise<Blob>>()
    }
    /// The takePhoto method.
    /// [`ImageCapture.takePhoto`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/takePhoto)
    pub fn take_photo1(&self, photo_settings: &PhotoSettings) -> Promise<Blob> {
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
impl ImageCapture {
    /// Getter of the `track` attribute.
    /// [`ImageCapture.track`](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
