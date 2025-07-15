use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageEncodeOptions {
    inner: emlite::Val,
}
impl FromVal for ImageEncodeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ImageEncodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageEncodeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageEncodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageEncodeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageEncodeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ImageEncodeOptions> for emlite::Val {
    fn from(s: ImageEncodeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ImageEncodeOptions> for emlite::Val {
    fn from(s: &ImageEncodeOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl ImageEncodeOptions {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl ImageEncodeOptions {
    pub fn quality(&self) -> f64 {
        self.inner.get("quality").as_::<f64>()
    }

    pub fn set_quality(&mut self, value: f64) {
        self.inner.set("quality", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OffscreenCanvas {
    inner: EventTarget,
}
impl FromVal for OffscreenCanvas {
    fn from_val(v: &emlite::Val) -> Self {
        OffscreenCanvas {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OffscreenCanvas {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OffscreenCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OffscreenCanvas {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OffscreenCanvas {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OffscreenCanvas> for emlite::Val {
    fn from(s: OffscreenCanvas) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&OffscreenCanvas> for emlite::Val {
    fn from(s: &OffscreenCanvas) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OffscreenCanvas);

impl OffscreenCanvas {
    pub fn new(width: u64, height: u64) -> OffscreenCanvas {
        Self {
            inner: emlite::Val::global("OffscreenCanvas")
                .new(&[width.into(), height.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl OffscreenCanvas {
    pub fn width(&self) -> u64 {
        self.inner.get("width").as_::<u64>()
    }

    pub fn set_width(&mut self, value: u64) {
        self.inner.set("width", value);
    }
}
impl OffscreenCanvas {
    pub fn height(&self) -> u64 {
        self.inner.get("height").as_::<u64>()
    }

    pub fn set_height(&mut self, value: u64) {
        self.inner.set("height", value);
    }
}
impl OffscreenCanvas {
    pub fn get_context0(&self, context_id: &OffscreenRenderingContextId) -> Any {
        self.inner
            .call("getContext", &[context_id.into()])
            .as_::<Any>()
    }

    pub fn get_context1(&self, context_id: &OffscreenRenderingContextId, options: &Any) -> Any {
        self.inner
            .call("getContext", &[context_id.into(), options.into()])
            .as_::<Any>()
    }
}
impl OffscreenCanvas {
    pub fn transfer_to_image_bitmap(&self) -> ImageBitmap {
        self.inner
            .call("transferToImageBitmap", &[])
            .as_::<ImageBitmap>()
    }
}
impl OffscreenCanvas {
    pub fn convert_to_blob0(&self) -> Promise {
        self.inner.call("convertToBlob", &[]).as_::<Promise>()
    }

    pub fn convert_to_blob1(&self, options: &ImageEncodeOptions) -> Promise {
        self.inner
            .call("convertToBlob", &[options.into()])
            .as_::<Promise>()
    }
}
impl OffscreenCanvas {
    pub fn oncontextlost(&self) -> Any {
        self.inner.get("oncontextlost").as_::<Any>()
    }

    pub fn set_oncontextlost(&mut self, value: &Any) {
        self.inner.set("oncontextlost", value);
    }
}
impl OffscreenCanvas {
    pub fn oncontextrestored(&self) -> Any {
        self.inner.get("oncontextrestored").as_::<Any>()
    }

    pub fn set_oncontextrestored(&mut self, value: &Any) {
        self.inner.set("oncontextrestored", value);
    }
}
