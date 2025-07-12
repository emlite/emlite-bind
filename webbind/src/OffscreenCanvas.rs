use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for ImageEncodeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImageEncodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageEncodeOptions> for emlite::Val {
    fn from(s: ImageEncodeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageEncodeOptions {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for OffscreenCanvas {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OffscreenCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OffscreenCanvas> for emlite::Val {
    fn from(s: OffscreenCanvas) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn get_context0(&self, context_id: OffscreenRenderingContextId) -> jsbind::Any {
        self.inner
            .call("getContext", &[context_id.into()])
            .as_::<jsbind::Any>()
    }

    pub fn get_context1(
        &self,
        context_id: OffscreenRenderingContextId,
        options: jsbind::Any,
    ) -> jsbind::Any {
        self.inner
            .call("getContext", &[context_id.into(), options.into()])
            .as_::<jsbind::Any>()
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
    pub fn convert_to_blob0(&self) -> jsbind::Promise {
        self.inner
            .call("convertToBlob", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn convert_to_blob1(&self, options: ImageEncodeOptions) -> jsbind::Promise {
        self.inner
            .call("convertToBlob", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl OffscreenCanvas {
    pub fn oncontextlost(&self) -> jsbind::Any {
        self.inner.get("oncontextlost").as_::<jsbind::Any>()
    }

    pub fn set_oncontextlost(&mut self, value: jsbind::Any) {
        self.inner.set("oncontextlost", value);
    }
}
impl OffscreenCanvas {
    pub fn oncontextrestored(&self) -> jsbind::Any {
        self.inner.get("oncontextrestored").as_::<jsbind::Any>()
    }

    pub fn set_oncontextrestored(&mut self, value: jsbind::Any) {
        self.inner.set("oncontextrestored", value);
    }
}
