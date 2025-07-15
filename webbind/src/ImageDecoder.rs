use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDecodeResult {
    inner: emlite::Val,
}
impl FromVal for ImageDecodeResult {
    fn from_val(v: &emlite::Val) -> Self {
        ImageDecodeResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageDecodeResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageDecodeResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageDecodeResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageDecodeResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ImageDecodeResult> for emlite::Val {
    fn from(s: ImageDecodeResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageDecodeResult {
    pub fn image(&self) -> VideoFrame {
        self.inner.get("image").as_::<VideoFrame>()
    }

    pub fn set_image(&mut self, value: VideoFrame) {
        self.inner.set("image", value);
    }
}
impl ImageDecodeResult {
    pub fn complete(&self) -> bool {
        self.inner.get("complete").as_::<bool>()
    }

    pub fn set_complete(&mut self, value: bool) {
        self.inner.set("complete", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDecodeOptions {
    inner: emlite::Val,
}
impl FromVal for ImageDecodeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ImageDecodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageDecodeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageDecodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageDecodeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageDecodeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ImageDecodeOptions> for emlite::Val {
    fn from(s: ImageDecodeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageDecodeOptions {
    pub fn frame_index(&self) -> u32 {
        self.inner.get("frameIndex").as_::<u32>()
    }

    pub fn set_frame_index(&mut self, value: u32) {
        self.inner.set("frameIndex", value);
    }
}
impl ImageDecodeOptions {
    pub fn complete_frames_only(&self) -> bool {
        self.inner.get("completeFramesOnly").as_::<bool>()
    }

    pub fn set_complete_frames_only(&mut self, value: bool) {
        self.inner.set("completeFramesOnly", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDecoder {
    inner: emlite::Val,
}
impl FromVal for ImageDecoder {
    fn from_val(v: &emlite::Val) -> Self {
        ImageDecoder {
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
impl core::ops::Deref for ImageDecoder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageDecoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageDecoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ImageDecoder> for emlite::Val {
    fn from(s: ImageDecoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ImageDecoder);

impl ImageDecoder {
    pub fn new(init: Any) -> ImageDecoder {
        Self {
            inner: emlite::Val::global("ImageDecoder")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ImageDecoder {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }
}
impl ImageDecoder {
    pub fn complete(&self) -> bool {
        self.inner.get("complete").as_::<bool>()
    }
}
impl ImageDecoder {
    pub fn completed(&self) -> Promise {
        self.inner.get("completed").as_::<Promise>()
    }
}
impl ImageDecoder {
    pub fn tracks(&self) -> ImageTrackList {
        self.inner.get("tracks").as_::<ImageTrackList>()
    }
}
impl ImageDecoder {
    pub fn decode0(&self) -> Promise {
        self.inner.call("decode", &[]).as_::<Promise>()
    }

    pub fn decode1(&self, options: ImageDecodeOptions) -> Promise {
        self.inner
            .call("decode", &[options.into()])
            .as_::<Promise>()
    }
}
impl ImageDecoder {
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl ImageDecoder {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl ImageDecoder {
    pub fn is_type_supported(type_: DOMString) -> Promise {
        emlite::Val::global("ImageDecoder")
            .call("isTypeSupported", &[type_.into()])
            .as_::<Promise>()
    }
}
