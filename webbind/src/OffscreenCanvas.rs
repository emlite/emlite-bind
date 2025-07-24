use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageEncodeOptions {
    inner: Any,
}
impl FromVal for ImageEncodeOptions {
    fn from_val(v: &Any) -> Self {
        ImageEncodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageEncodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageEncodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ImageEncodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ImageEncodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ImageEncodeOptions> for Any {
    fn from(s: ImageEncodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ImageEncodeOptions> for Any {
    fn from(s: &ImageEncodeOptions) -> Any {
        s.inner.clone()
    }
}

impl ImageEncodeOptions {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: &DOMString) {
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
/// The OffscreenCanvas class.
/// [`OffscreenCanvas`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OffscreenCanvas {
    inner: EventTarget,
}
impl FromVal for OffscreenCanvas {
    fn from_val(v: &Any) -> Self {
        OffscreenCanvas {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for OffscreenCanvas {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OffscreenCanvas {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OffscreenCanvas> for Any {
    fn from(s: OffscreenCanvas) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OffscreenCanvas> for Any {
    fn from(s: &OffscreenCanvas) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OffscreenCanvas);

impl OffscreenCanvas {
    /// The `new OffscreenCanvas(..)` constructor, creating a new OffscreenCanvas instance
    pub fn new(width: u64, height: u64) -> OffscreenCanvas {
        Self {
            inner: Any::global("OffscreenCanvas")
                .new(&[width.into(), height.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl OffscreenCanvas {
    /// Getter of the `width` attribute.
    /// [`OffscreenCanvas.width`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)
    pub fn width(&self) -> u64 {
        self.inner.get("width").as_::<u64>()
    }

    /// Setter of the `width` attribute.
    /// [`OffscreenCanvas.width`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)
    pub fn set_width(&mut self, value: u64) {
        self.inner.set("width", value);
    }
}
impl OffscreenCanvas {
    /// Getter of the `height` attribute.
    /// [`OffscreenCanvas.height`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)
    pub fn height(&self) -> u64 {
        self.inner.get("height").as_::<u64>()
    }

    /// Setter of the `height` attribute.
    /// [`OffscreenCanvas.height`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)
    pub fn set_height(&mut self, value: u64) {
        self.inner.set("height", value);
    }
}
impl OffscreenCanvas {
    /// The getContext method.
    /// [`OffscreenCanvas.getContext`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)
    pub fn get_context0(&self, context_id: &OffscreenRenderingContextId) -> Any {
        self.inner
            .call("getContext", &[context_id.into()])
            .as_::<Any>()
    }
    /// The getContext method.
    /// [`OffscreenCanvas.getContext`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)
    pub fn get_context1(&self, context_id: &OffscreenRenderingContextId, options: &Any) -> Any {
        self.inner
            .call("getContext", &[context_id.into(), options.into()])
            .as_::<Any>()
    }
}
impl OffscreenCanvas {
    /// The transferToImageBitmap method.
    /// [`OffscreenCanvas.transferToImageBitmap`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/transferToImageBitmap)
    pub fn transfer_to_image_bitmap(&self) -> ImageBitmap {
        self.inner
            .call("transferToImageBitmap", &[])
            .as_::<ImageBitmap>()
    }
}
impl OffscreenCanvas {
    /// The convertToBlob method.
    /// [`OffscreenCanvas.convertToBlob`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/convertToBlob)
    pub fn convert_to_blob0(&self) -> Promise<Blob> {
        self.inner.call("convertToBlob", &[]).as_::<Promise<Blob>>()
    }
    /// The convertToBlob method.
    /// [`OffscreenCanvas.convertToBlob`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/convertToBlob)
    pub fn convert_to_blob1(&self, options: &ImageEncodeOptions) -> Promise<Blob> {
        self.inner
            .call("convertToBlob", &[options.into()])
            .as_::<Promise<Blob>>()
    }
}
impl OffscreenCanvas {
    /// Getter of the `oncontextlost` attribute.
    /// [`OffscreenCanvas.oncontextlost`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/oncontextlost)
    pub fn oncontextlost(&self) -> Any {
        self.inner.get("oncontextlost").as_::<Any>()
    }

    /// Setter of the `oncontextlost` attribute.
    /// [`OffscreenCanvas.oncontextlost`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/oncontextlost)
    pub fn set_oncontextlost(&mut self, value: &Any) {
        self.inner.set("oncontextlost", value);
    }
}
impl OffscreenCanvas {
    /// Getter of the `oncontextrestored` attribute.
    /// [`OffscreenCanvas.oncontextrestored`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/oncontextrestored)
    pub fn oncontextrestored(&self) -> Any {
        self.inner.get("oncontextrestored").as_::<Any>()
    }

    /// Setter of the `oncontextrestored` attribute.
    /// [`OffscreenCanvas.oncontextrestored`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/oncontextrestored)
    pub fn set_oncontextrestored(&mut self, value: &Any) {
        self.inner.set("oncontextrestored", value);
    }
}
