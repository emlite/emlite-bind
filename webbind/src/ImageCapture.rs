use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhotoSettings {
    inner: emlite::Val,
}
impl FromVal for PhotoSettings {
    fn from_val(v: &emlite::Val) -> Self {
        PhotoSettings { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PhotoSettings {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PhotoSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PhotoSettings {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PhotoSettings {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PhotoSettings> for emlite::Val {
    fn from(s: PhotoSettings) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PhotoSettings> for emlite::Val {
    fn from(s: &PhotoSettings) -> emlite::Val {
        s.inner.clone()
    }
}

impl PhotoSettings {
    pub fn fill_light_mode(&self) -> FillLightMode {
        self.inner.get("fillLightMode").as_::<FillLightMode>()
    }

    pub fn set_fill_light_mode(&mut self, value: FillLightMode) {
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
    inner: emlite::Val,
}
impl FromVal for PhotoCapabilities {
    fn from_val(v: &emlite::Val) -> Self {
        PhotoCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PhotoCapabilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PhotoCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PhotoCapabilities {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PhotoCapabilities {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PhotoCapabilities> for emlite::Val {
    fn from(s: PhotoCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PhotoCapabilities> for emlite::Val {
    fn from(s: &PhotoCapabilities) -> emlite::Val {
        s.inner.clone()
    }
}

impl PhotoCapabilities {
    pub fn red_eye_reduction(&self) -> RedEyeReduction {
        self.inner.get("redEyeReduction").as_::<RedEyeReduction>()
    }

    pub fn set_red_eye_reduction(&mut self, value: RedEyeReduction) {
        self.inner.set("redEyeReduction", value);
    }
}
impl PhotoCapabilities {
    pub fn image_height(&self) -> Any {
        self.inner.get("imageHeight").as_::<Any>()
    }

    pub fn set_image_height(&mut self, value: Any) {
        self.inner.set("imageHeight", value);
    }
}
impl PhotoCapabilities {
    pub fn image_width(&self) -> Any {
        self.inner.get("imageWidth").as_::<Any>()
    }

    pub fn set_image_width(&mut self, value: Any) {
        self.inner.set("imageWidth", value);
    }
}
impl PhotoCapabilities {
    pub fn fill_light_mode(&self) -> Sequence<FillLightMode> {
        self.inner
            .get("fillLightMode")
            .as_::<Sequence<FillLightMode>>()
    }

    pub fn set_fill_light_mode(&mut self, value: Sequence<FillLightMode>) {
        self.inner.set("fillLightMode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageCapture {
    inner: emlite::Val,
}
impl FromVal for ImageCapture {
    fn from_val(v: &emlite::Val) -> Self {
        ImageCapture {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageCapture {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageCapture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageCapture {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageCapture {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ImageCapture> for emlite::Val {
    fn from(s: ImageCapture) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ImageCapture> for emlite::Val {
    fn from(s: &ImageCapture) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ImageCapture);

impl ImageCapture {
    pub fn new(video_track: MediaStreamTrack) -> ImageCapture {
        Self {
            inner: emlite::Val::global("ImageCapture")
                .new(&[video_track.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ImageCapture {
    pub fn take_photo0(&self) -> Promise {
        self.inner.call("takePhoto", &[]).as_::<Promise>()
    }

    pub fn take_photo1(&self, photo_settings: PhotoSettings) -> Promise {
        self.inner
            .call("takePhoto", &[photo_settings.into()])
            .as_::<Promise>()
    }
}
impl ImageCapture {
    pub fn get_photo_capabilities(&self) -> Promise {
        self.inner
            .call("getPhotoCapabilities", &[])
            .as_::<Promise>()
    }
}
impl ImageCapture {
    pub fn get_photo_settings(&self) -> Promise {
        self.inner.call("getPhotoSettings", &[]).as_::<Promise>()
    }
}
impl ImageCapture {
    pub fn grab_frame(&self) -> Promise {
        self.inner.call("grabFrame", &[]).as_::<Promise>()
    }
}
impl ImageCapture {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
