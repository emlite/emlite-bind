use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasRenderingContext2DSettings {
    inner: Any,
}
impl FromVal for CanvasRenderingContext2DSettings {
    fn from_val(v: &Any) -> Self {
        CanvasRenderingContext2DSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CanvasRenderingContext2DSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasRenderingContext2DSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CanvasRenderingContext2DSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CanvasRenderingContext2DSettings {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CanvasRenderingContext2DSettings> for Any {
    fn from(s: CanvasRenderingContext2DSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CanvasRenderingContext2DSettings> for Any {
    fn from(s: &CanvasRenderingContext2DSettings) -> Any {
        s.inner.clone()
    }
}

impl CanvasRenderingContext2DSettings {
    pub fn alpha(&self) -> bool {
        self.inner.get("alpha").as_::<bool>()
    }

    pub fn set_alpha(&mut self, value: bool) {
        self.inner.set("alpha", value);
    }
}
impl CanvasRenderingContext2DSettings {
    pub fn desynchronized(&self) -> bool {
        self.inner.get("desynchronized").as_::<bool>()
    }

    pub fn set_desynchronized(&mut self, value: bool) {
        self.inner.set("desynchronized", value);
    }
}
impl CanvasRenderingContext2DSettings {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl CanvasRenderingContext2DSettings {
    pub fn color_type(&self) -> CanvasColorType {
        self.inner.get("colorType").as_::<CanvasColorType>()
    }

    pub fn set_color_type(&mut self, value: &CanvasColorType) {
        self.inner.set("colorType", value);
    }
}
impl CanvasRenderingContext2DSettings {
    pub fn will_read_frequently(&self) -> bool {
        self.inner.get("willReadFrequently").as_::<bool>()
    }

    pub fn set_will_read_frequently(&mut self, value: bool) {
        self.inner.set("willReadFrequently", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDataSettings {
    inner: Any,
}
impl FromVal for ImageDataSettings {
    fn from_val(v: &Any) -> Self {
        ImageDataSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageDataSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageDataSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ImageDataSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ImageDataSettings {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ImageDataSettings> for Any {
    fn from(s: ImageDataSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ImageDataSettings> for Any {
    fn from(s: &ImageDataSettings) -> Any {
        s.inner.clone()
    }
}

impl ImageDataSettings {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl ImageDataSettings {
    pub fn pixel_format(&self) -> ImageDataPixelFormat {
        self.inner.get("pixelFormat").as_::<ImageDataPixelFormat>()
    }

    pub fn set_pixel_format(&mut self, value: &ImageDataPixelFormat) {
        self.inner.set("pixelFormat", value);
    }
}
/// The CanvasRenderingContext2D class.
/// [`CanvasRenderingContext2D`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasRenderingContext2D {
    inner: Any,
}
impl FromVal for CanvasRenderingContext2D {
    fn from_val(v: &Any) -> Self {
        CanvasRenderingContext2D {
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
impl core::ops::Deref for CanvasRenderingContext2D {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasRenderingContext2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CanvasRenderingContext2D {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CanvasRenderingContext2D {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CanvasRenderingContext2D> for Any {
    fn from(s: CanvasRenderingContext2D) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CanvasRenderingContext2D> for Any {
    fn from(s: &CanvasRenderingContext2D) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CanvasRenderingContext2D);

impl CanvasRenderingContext2D {
    /// Getter of the `canvas` attribute.
    /// [`CanvasRenderingContext2D.canvas`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/canvas)
    pub fn canvas(&self) -> HTMLCanvasElement {
        self.inner.get("canvas").as_::<HTMLCanvasElement>()
    }
}
impl CanvasRenderingContext2D {
    /// The getContextAttributes method.
    /// [`CanvasRenderingContext2D.getContextAttributes`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getContextAttributes)
    pub fn get_context_attributes(&self) -> CanvasRenderingContext2DSettings {
        self.inner
            .call("getContextAttributes", &[])
            .as_::<CanvasRenderingContext2DSettings>()
    }
}
impl CanvasRenderingContext2D {
    /// The save method.
    /// [`CanvasRenderingContext2D.save`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)
    pub fn save(&self) -> Undefined {
        self.inner.call("save", &[]).as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The restore method.
    /// [`CanvasRenderingContext2D.restore`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)
    pub fn restore(&self) -> Undefined {
        self.inner.call("restore", &[]).as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The reset method.
    /// [`CanvasRenderingContext2D.reset`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The isContextLost method.
    /// [`CanvasRenderingContext2D.isContextLost`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isContextLost)
    pub fn is_context_lost(&self) -> bool {
        self.inner.call("isContextLost", &[]).as_::<bool>()
    }
}
impl CanvasRenderingContext2D {
    /// The scale method.
    /// [`CanvasRenderingContext2D.scale`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)
    pub fn scale(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scale", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The rotate method.
    /// [`CanvasRenderingContext2D.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)
    pub fn rotate(&self, angle: f64) -> Undefined {
        self.inner
            .call("rotate", &[angle.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The translate method.
    /// [`CanvasRenderingContext2D.translate`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)
    pub fn translate(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("translate", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The transform method.
    /// [`CanvasRenderingContext2D.transform`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Undefined {
        self.inner
            .call(
                "transform",
                &[a.into(), b.into(), c.into(), d.into(), e.into(), f.into()],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The getTransform method.
    /// [`CanvasRenderingContext2D.getTransform`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getTransform)
    pub fn get_transform(&self) -> DOMMatrix {
        self.inner.call("getTransform", &[]).as_::<DOMMatrix>()
    }
}
impl CanvasRenderingContext2D {
    /// The setTransform method.
    /// [`CanvasRenderingContext2D.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)
    pub fn set_transform0(&self) -> Undefined {
        self.inner.call("setTransform", &[]).as_::<Undefined>()
    }
    /// The setTransform method.
    /// [`CanvasRenderingContext2D.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)
    pub fn set_transform1(&self, transform: &DOMMatrix2DInit) -> Undefined {
        self.inner
            .call("setTransform", &[transform.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The resetTransform method.
    /// [`CanvasRenderingContext2D.resetTransform`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/resetTransform)
    pub fn reset_transform(&self) -> Undefined {
        self.inner.call("resetTransform", &[]).as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `globalAlpha` attribute.
    /// [`CanvasRenderingContext2D.globalAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)
    pub fn global_alpha(&self) -> f64 {
        self.inner.get("globalAlpha").as_::<f64>()
    }

    /// Setter of the `globalAlpha` attribute.
    /// [`CanvasRenderingContext2D.globalAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)
    pub fn set_global_alpha(&mut self, value: f64) {
        self.inner.set("globalAlpha", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `globalCompositeOperation` attribute.
    /// [`CanvasRenderingContext2D.globalCompositeOperation`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)
    pub fn global_composite_operation(&self) -> JsString {
        self.inner.get("globalCompositeOperation").as_::<JsString>()
    }

    /// Setter of the `globalCompositeOperation` attribute.
    /// [`CanvasRenderingContext2D.globalCompositeOperation`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)
    pub fn set_global_composite_operation(&mut self, value: &JsString) {
        self.inner.set("globalCompositeOperation", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `imageSmoothingEnabled` attribute.
    /// [`CanvasRenderingContext2D.imageSmoothingEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)
    pub fn image_smoothing_enabled(&self) -> bool {
        self.inner.get("imageSmoothingEnabled").as_::<bool>()
    }

    /// Setter of the `imageSmoothingEnabled` attribute.
    /// [`CanvasRenderingContext2D.imageSmoothingEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)
    pub fn set_image_smoothing_enabled(&mut self, value: bool) {
        self.inner.set("imageSmoothingEnabled", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `imageSmoothingQuality` attribute.
    /// [`CanvasRenderingContext2D.imageSmoothingQuality`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingQuality)
    pub fn image_smoothing_quality(&self) -> ImageSmoothingQuality {
        self.inner
            .get("imageSmoothingQuality")
            .as_::<ImageSmoothingQuality>()
    }

    /// Setter of the `imageSmoothingQuality` attribute.
    /// [`CanvasRenderingContext2D.imageSmoothingQuality`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingQuality)
    pub fn set_image_smoothing_quality(&mut self, value: &ImageSmoothingQuality) {
        self.inner.set("imageSmoothingQuality", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `strokeStyle` attribute.
    /// [`CanvasRenderingContext2D.strokeStyle`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    pub fn stroke_style(&self) -> Any {
        self.inner.get("strokeStyle").as_::<Any>()
    }

    /// Setter of the `strokeStyle` attribute.
    /// [`CanvasRenderingContext2D.strokeStyle`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    pub fn set_stroke_style(&mut self, value: &Any) {
        self.inner.set("strokeStyle", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `fillStyle` attribute.
    /// [`CanvasRenderingContext2D.fillStyle`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    pub fn fill_style(&self) -> Any {
        self.inner.get("fillStyle").as_::<Any>()
    }

    /// Setter of the `fillStyle` attribute.
    /// [`CanvasRenderingContext2D.fillStyle`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    pub fn set_fill_style(&mut self, value: &Any) {
        self.inner.set("fillStyle", value);
    }
}
impl CanvasRenderingContext2D {
    /// The createLinearGradient method.
    /// [`CanvasRenderingContext2D.createLinearGradient`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient {
        self.inner
            .call(
                "createLinearGradient",
                &[x0.into(), y0.into(), x1.into(), y1.into()],
            )
            .as_::<CanvasGradient>()
    }
}
impl CanvasRenderingContext2D {
    /// The createRadialGradient method.
    /// [`CanvasRenderingContext2D.createRadialGradient`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)
    pub fn create_radial_gradient(
        &self,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> CanvasGradient {
        self.inner
            .call(
                "createRadialGradient",
                &[
                    x0.into(),
                    y0.into(),
                    r0.into(),
                    x1.into(),
                    y1.into(),
                    r1.into(),
                ],
            )
            .as_::<CanvasGradient>()
    }
}
impl CanvasRenderingContext2D {
    /// The createConicGradient method.
    /// [`CanvasRenderingContext2D.createConicGradient`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createConicGradient)
    pub fn create_conic_gradient(&self, start_angle: f64, x: f64, y: f64) -> CanvasGradient {
        self.inner
            .call(
                "createConicGradient",
                &[start_angle.into(), x.into(), y.into()],
            )
            .as_::<CanvasGradient>()
    }
}
impl CanvasRenderingContext2D {
    /// The createPattern method.
    /// [`CanvasRenderingContext2D.createPattern`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    pub fn create_pattern(&self, image: &Any, repetition: &JsString) -> CanvasPattern {
        self.inner
            .call("createPattern", &[image.into(), repetition.into()])
            .as_::<CanvasPattern>()
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `shadowOffsetX` attribute.
    /// [`CanvasRenderingContext2D.shadowOffsetX`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)
    pub fn shadow_offset_x(&self) -> f64 {
        self.inner.get("shadowOffsetX").as_::<f64>()
    }

    /// Setter of the `shadowOffsetX` attribute.
    /// [`CanvasRenderingContext2D.shadowOffsetX`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)
    pub fn set_shadow_offset_x(&mut self, value: f64) {
        self.inner.set("shadowOffsetX", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `shadowOffsetY` attribute.
    /// [`CanvasRenderingContext2D.shadowOffsetY`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)
    pub fn shadow_offset_y(&self) -> f64 {
        self.inner.get("shadowOffsetY").as_::<f64>()
    }

    /// Setter of the `shadowOffsetY` attribute.
    /// [`CanvasRenderingContext2D.shadowOffsetY`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)
    pub fn set_shadow_offset_y(&mut self, value: f64) {
        self.inner.set("shadowOffsetY", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `shadowBlur` attribute.
    /// [`CanvasRenderingContext2D.shadowBlur`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)
    pub fn shadow_blur(&self) -> f64 {
        self.inner.get("shadowBlur").as_::<f64>()
    }

    /// Setter of the `shadowBlur` attribute.
    /// [`CanvasRenderingContext2D.shadowBlur`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)
    pub fn set_shadow_blur(&mut self, value: f64) {
        self.inner.set("shadowBlur", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `shadowColor` attribute.
    /// [`CanvasRenderingContext2D.shadowColor`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)
    pub fn shadow_color(&self) -> JsString {
        self.inner.get("shadowColor").as_::<JsString>()
    }

    /// Setter of the `shadowColor` attribute.
    /// [`CanvasRenderingContext2D.shadowColor`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)
    pub fn set_shadow_color(&mut self, value: &JsString) {
        self.inner.set("shadowColor", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `filter` attribute.
    /// [`CanvasRenderingContext2D.filter`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)
    pub fn filter(&self) -> JsString {
        self.inner.get("filter").as_::<JsString>()
    }

    /// Setter of the `filter` attribute.
    /// [`CanvasRenderingContext2D.filter`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)
    pub fn set_filter(&mut self, value: &JsString) {
        self.inner.set("filter", value);
    }
}
impl CanvasRenderingContext2D {
    /// The clearRect method.
    /// [`CanvasRenderingContext2D.clearRect`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)
    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("clearRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The fillRect method.
    /// [`CanvasRenderingContext2D.fillRect`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)
    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("fillRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The strokeRect method.
    /// [`CanvasRenderingContext2D.strokeRect`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)
    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("strokeRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The beginPath method.
    /// [`CanvasRenderingContext2D.beginPath`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)
    pub fn begin_path(&self) -> Undefined {
        self.inner.call("beginPath", &[]).as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The fill method.
    /// [`CanvasRenderingContext2D.fill`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    pub fn fill0(&self, path: &Path2D) -> Undefined {
        self.inner.call("fill", &[path.into()]).as_::<Undefined>()
    }
    /// The fill method.
    /// [`CanvasRenderingContext2D.fill`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    pub fn fill1(&self, path: &Path2D, fill_rule: &CanvasFillRule) -> Undefined {
        self.inner
            .call("fill", &[path.into(), fill_rule.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The stroke method.
    /// [`CanvasRenderingContext2D.stroke`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)
    pub fn stroke(&self, path: &Path2D) -> Undefined {
        self.inner.call("stroke", &[path.into()]).as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The clip method.
    /// [`CanvasRenderingContext2D.clip`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    pub fn clip0(&self, path: &Path2D) -> Undefined {
        self.inner.call("clip", &[path.into()]).as_::<Undefined>()
    }
    /// The clip method.
    /// [`CanvasRenderingContext2D.clip`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    pub fn clip1(&self, path: &Path2D, fill_rule: &CanvasFillRule) -> Undefined {
        self.inner
            .call("clip", &[path.into(), fill_rule.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The isPointInPath method.
    /// [`CanvasRenderingContext2D.isPointInPath`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    pub fn is_point_in_path0(&self, path: &Path2D, x: f64, y: f64) -> bool {
        self.inner
            .call("isPointInPath", &[path.into(), x.into(), y.into()])
            .as_::<bool>()
    }
    /// The isPointInPath method.
    /// [`CanvasRenderingContext2D.isPointInPath`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    pub fn is_point_in_path1(
        &self,
        path: &Path2D,
        x: f64,
        y: f64,
        fill_rule: &CanvasFillRule,
    ) -> bool {
        self.inner
            .call(
                "isPointInPath",
                &[path.into(), x.into(), y.into(), fill_rule.into()],
            )
            .as_::<bool>()
    }
}
impl CanvasRenderingContext2D {
    /// The isPointInStroke method.
    /// [`CanvasRenderingContext2D.isPointInStroke`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)
    pub fn is_point_in_stroke(&self, path: &Path2D, x: f64, y: f64) -> bool {
        self.inner
            .call("isPointInStroke", &[path.into(), x.into(), y.into()])
            .as_::<bool>()
    }
}
impl CanvasRenderingContext2D {
    /// The drawFocusIfNeeded method.
    /// [`CanvasRenderingContext2D.drawFocusIfNeeded`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)
    pub fn draw_focus_if_needed(&self, path: &Path2D, element: &Element) -> Undefined {
        self.inner
            .call("drawFocusIfNeeded", &[path.into(), element.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The fillText method.
    /// [`CanvasRenderingContext2D.fillText`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)
    pub fn fill_text0(&self, text: &JsString, x: f64, y: f64) -> Undefined {
        self.inner
            .call("fillText", &[text.into(), x.into(), y.into()])
            .as_::<Undefined>()
    }
    /// The fillText method.
    /// [`CanvasRenderingContext2D.fillText`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)
    pub fn fill_text1(&self, text: &JsString, x: f64, y: f64, max_width: f64) -> Undefined {
        self.inner
            .call(
                "fillText",
                &[text.into(), x.into(), y.into(), max_width.into()],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The strokeText method.
    /// [`CanvasRenderingContext2D.strokeText`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)
    pub fn stroke_text0(&self, text: &JsString, x: f64, y: f64) -> Undefined {
        self.inner
            .call("strokeText", &[text.into(), x.into(), y.into()])
            .as_::<Undefined>()
    }
    /// The strokeText method.
    /// [`CanvasRenderingContext2D.strokeText`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)
    pub fn stroke_text1(&self, text: &JsString, x: f64, y: f64, max_width: f64) -> Undefined {
        self.inner
            .call(
                "strokeText",
                &[text.into(), x.into(), y.into(), max_width.into()],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The measureText method.
    /// [`CanvasRenderingContext2D.measureText`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)
    pub fn measure_text(&self, text: &JsString) -> TextMetrics {
        self.inner
            .call("measureText", &[text.into()])
            .as_::<TextMetrics>()
    }
}
impl CanvasRenderingContext2D {
    /// The drawImage method.
    /// [`CanvasRenderingContext2D.drawImage`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    pub fn draw_image(
        &self,
        image: &Any,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Undefined {
        self.inner
            .call(
                "drawImage",
                &[
                    image.into(),
                    sx.into(),
                    sy.into(),
                    sw.into(),
                    sh.into(),
                    dx.into(),
                    dy.into(),
                    dw.into(),
                    dh.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The createImageData method.
    /// [`CanvasRenderingContext2D.createImageData`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)
    pub fn create_image_data(&self, image_data: &ImageData) -> ImageData {
        self.inner
            .call("createImageData", &[image_data.into()])
            .as_::<ImageData>()
    }
}
impl CanvasRenderingContext2D {
    /// The getImageData method.
    /// [`CanvasRenderingContext2D.getImageData`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)
    pub fn get_image_data0(&self, sx: i32, sy: i32, sw: i32, sh: i32) -> ImageData {
        self.inner
            .call(
                "getImageData",
                &[sx.into(), sy.into(), sw.into(), sh.into()],
            )
            .as_::<ImageData>()
    }
    /// The getImageData method.
    /// [`CanvasRenderingContext2D.getImageData`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)
    pub fn get_image_data1(
        &self,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
        settings: &ImageDataSettings,
    ) -> ImageData {
        self.inner
            .call(
                "getImageData",
                &[sx.into(), sy.into(), sw.into(), sh.into(), settings.into()],
            )
            .as_::<ImageData>()
    }
}
impl CanvasRenderingContext2D {
    /// The putImageData method.
    /// [`CanvasRenderingContext2D.putImageData`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)
    pub fn put_image_data(
        &self,
        image_data: &ImageData,
        dx: i32,
        dy: i32,
        dirty_x: i32,
        dirty_y: i32,
        dirty_width: i32,
        dirty_height: i32,
    ) -> Undefined {
        self.inner
            .call(
                "putImageData",
                &[
                    image_data.into(),
                    dx.into(),
                    dy.into(),
                    dirty_x.into(),
                    dirty_y.into(),
                    dirty_width.into(),
                    dirty_height.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `lineWidth` attribute.
    /// [`CanvasRenderingContext2D.lineWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    pub fn line_width(&self) -> f64 {
        self.inner.get("lineWidth").as_::<f64>()
    }

    /// Setter of the `lineWidth` attribute.
    /// [`CanvasRenderingContext2D.lineWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    pub fn set_line_width(&mut self, value: f64) {
        self.inner.set("lineWidth", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `lineCap` attribute.
    /// [`CanvasRenderingContext2D.lineCap`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    pub fn line_cap(&self) -> CanvasLineCap {
        self.inner.get("lineCap").as_::<CanvasLineCap>()
    }

    /// Setter of the `lineCap` attribute.
    /// [`CanvasRenderingContext2D.lineCap`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    pub fn set_line_cap(&mut self, value: &CanvasLineCap) {
        self.inner.set("lineCap", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `lineJoin` attribute.
    /// [`CanvasRenderingContext2D.lineJoin`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
    pub fn line_join(&self) -> CanvasLineJoin {
        self.inner.get("lineJoin").as_::<CanvasLineJoin>()
    }

    /// Setter of the `lineJoin` attribute.
    /// [`CanvasRenderingContext2D.lineJoin`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
    pub fn set_line_join(&mut self, value: &CanvasLineJoin) {
        self.inner.set("lineJoin", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `miterLimit` attribute.
    /// [`CanvasRenderingContext2D.miterLimit`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)
    pub fn miter_limit(&self) -> f64 {
        self.inner.get("miterLimit").as_::<f64>()
    }

    /// Setter of the `miterLimit` attribute.
    /// [`CanvasRenderingContext2D.miterLimit`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)
    pub fn set_miter_limit(&mut self, value: f64) {
        self.inner.set("miterLimit", value);
    }
}
impl CanvasRenderingContext2D {
    /// The setLineDash method.
    /// [`CanvasRenderingContext2D.setLineDash`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)
    pub fn set_line_dash(&self, segments: TypedArray<f64>) -> Undefined {
        self.inner
            .call("setLineDash", &[segments.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The getLineDash method.
    /// [`CanvasRenderingContext2D.getLineDash`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)
    pub fn get_line_dash(&self) -> TypedArray<f64> {
        self.inner.call("getLineDash", &[]).as_::<TypedArray<f64>>()
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `lineDashOffset` attribute.
    /// [`CanvasRenderingContext2D.lineDashOffset`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)
    pub fn line_dash_offset(&self) -> f64 {
        self.inner.get("lineDashOffset").as_::<f64>()
    }

    /// Setter of the `lineDashOffset` attribute.
    /// [`CanvasRenderingContext2D.lineDashOffset`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)
    pub fn set_line_dash_offset(&mut self, value: f64) {
        self.inner.set("lineDashOffset", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `lang` attribute.
    /// [`CanvasRenderingContext2D.lang`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lang)
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

    /// Setter of the `lang` attribute.
    /// [`CanvasRenderingContext2D.lang`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lang)
    pub fn set_lang(&mut self, value: &JsString) {
        self.inner.set("lang", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `font` attribute.
    /// [`CanvasRenderingContext2D.font`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)
    pub fn font(&self) -> JsString {
        self.inner.get("font").as_::<JsString>()
    }

    /// Setter of the `font` attribute.
    /// [`CanvasRenderingContext2D.font`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)
    pub fn set_font(&mut self, value: &JsString) {
        self.inner.set("font", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `textAlign` attribute.
    /// [`CanvasRenderingContext2D.textAlign`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)
    pub fn text_align(&self) -> CanvasTextAlign {
        self.inner.get("textAlign").as_::<CanvasTextAlign>()
    }

    /// Setter of the `textAlign` attribute.
    /// [`CanvasRenderingContext2D.textAlign`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)
    pub fn set_text_align(&mut self, value: &CanvasTextAlign) {
        self.inner.set("textAlign", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `textBaseline` attribute.
    /// [`CanvasRenderingContext2D.textBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)
    pub fn text_baseline(&self) -> CanvasTextBaseline {
        self.inner.get("textBaseline").as_::<CanvasTextBaseline>()
    }

    /// Setter of the `textBaseline` attribute.
    /// [`CanvasRenderingContext2D.textBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)
    pub fn set_text_baseline(&mut self, value: &CanvasTextBaseline) {
        self.inner.set("textBaseline", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `direction` attribute.
    /// [`CanvasRenderingContext2D.direction`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/direction)
    pub fn direction(&self) -> CanvasDirection {
        self.inner.get("direction").as_::<CanvasDirection>()
    }

    /// Setter of the `direction` attribute.
    /// [`CanvasRenderingContext2D.direction`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/direction)
    pub fn set_direction(&mut self, value: &CanvasDirection) {
        self.inner.set("direction", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `letterSpacing` attribute.
    /// [`CanvasRenderingContext2D.letterSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/letterSpacing)
    pub fn letter_spacing(&self) -> JsString {
        self.inner.get("letterSpacing").as_::<JsString>()
    }

    /// Setter of the `letterSpacing` attribute.
    /// [`CanvasRenderingContext2D.letterSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/letterSpacing)
    pub fn set_letter_spacing(&mut self, value: &JsString) {
        self.inner.set("letterSpacing", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `fontKerning` attribute.
    /// [`CanvasRenderingContext2D.fontKerning`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fontKerning)
    pub fn font_kerning(&self) -> CanvasFontKerning {
        self.inner.get("fontKerning").as_::<CanvasFontKerning>()
    }

    /// Setter of the `fontKerning` attribute.
    /// [`CanvasRenderingContext2D.fontKerning`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fontKerning)
    pub fn set_font_kerning(&mut self, value: &CanvasFontKerning) {
        self.inner.set("fontKerning", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `fontStretch` attribute.
    /// [`CanvasRenderingContext2D.fontStretch`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fontStretch)
    pub fn font_stretch(&self) -> CanvasFontStretch {
        self.inner.get("fontStretch").as_::<CanvasFontStretch>()
    }

    /// Setter of the `fontStretch` attribute.
    /// [`CanvasRenderingContext2D.fontStretch`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fontStretch)
    pub fn set_font_stretch(&mut self, value: &CanvasFontStretch) {
        self.inner.set("fontStretch", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `fontVariantCaps` attribute.
    /// [`CanvasRenderingContext2D.fontVariantCaps`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fontVariantCaps)
    pub fn font_variant_caps(&self) -> CanvasFontVariantCaps {
        self.inner
            .get("fontVariantCaps")
            .as_::<CanvasFontVariantCaps>()
    }

    /// Setter of the `fontVariantCaps` attribute.
    /// [`CanvasRenderingContext2D.fontVariantCaps`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fontVariantCaps)
    pub fn set_font_variant_caps(&mut self, value: &CanvasFontVariantCaps) {
        self.inner.set("fontVariantCaps", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `textRendering` attribute.
    /// [`CanvasRenderingContext2D.textRendering`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textRendering)
    pub fn text_rendering(&self) -> CanvasTextRendering {
        self.inner.get("textRendering").as_::<CanvasTextRendering>()
    }

    /// Setter of the `textRendering` attribute.
    /// [`CanvasRenderingContext2D.textRendering`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textRendering)
    pub fn set_text_rendering(&mut self, value: &CanvasTextRendering) {
        self.inner.set("textRendering", value);
    }
}
impl CanvasRenderingContext2D {
    /// Getter of the `wordSpacing` attribute.
    /// [`CanvasRenderingContext2D.wordSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/wordSpacing)
    pub fn word_spacing(&self) -> JsString {
        self.inner.get("wordSpacing").as_::<JsString>()
    }

    /// Setter of the `wordSpacing` attribute.
    /// [`CanvasRenderingContext2D.wordSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/wordSpacing)
    pub fn set_word_spacing(&mut self, value: &JsString) {
        self.inner.set("wordSpacing", value);
    }
}
impl CanvasRenderingContext2D {
    /// The closePath method.
    /// [`CanvasRenderingContext2D.closePath`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)
    pub fn close_path(&self) -> Undefined {
        self.inner.call("closePath", &[]).as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The moveTo method.
    /// [`CanvasRenderingContext2D.moveTo`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)
    pub fn move_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("moveTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The lineTo method.
    /// [`CanvasRenderingContext2D.lineTo`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)
    pub fn line_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("lineTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The quadraticCurveTo method.
    /// [`CanvasRenderingContext2D.quadraticCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) -> Undefined {
        self.inner
            .call(
                "quadraticCurveTo",
                &[cpx.into(), cpy.into(), x.into(), y.into()],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The bezierCurveTo method.
    /// [`CanvasRenderingContext2D.bezierCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)
    pub fn bezier_curve_to(
        &self,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) -> Undefined {
        self.inner
            .call(
                "bezierCurveTo",
                &[
                    cp1x.into(),
                    cp1y.into(),
                    cp2x.into(),
                    cp2y.into(),
                    x.into(),
                    y.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The arcTo method.
    /// [`CanvasRenderingContext2D.arcTo`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Undefined {
        self.inner
            .call(
                "arcTo",
                &[x1.into(), y1.into(), x2.into(), y2.into(), radius.into()],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The rect method.
    /// [`CanvasRenderingContext2D.rect`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("rect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The roundRect method.
    /// [`CanvasRenderingContext2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)
    pub fn round_rect0(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("roundRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
    /// The roundRect method.
    /// [`CanvasRenderingContext2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)
    pub fn round_rect1(&self, x: f64, y: f64, w: f64, h: f64, radii: &Any) -> Undefined {
        self.inner
            .call(
                "roundRect",
                &[x.into(), y.into(), w.into(), h.into(), radii.into()],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The arc method.
    /// [`CanvasRenderingContext2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)
    pub fn arc0(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Undefined {
        self.inner
            .call(
                "arc",
                &[
                    x.into(),
                    y.into(),
                    radius.into(),
                    start_angle.into(),
                    end_angle.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The arc method.
    /// [`CanvasRenderingContext2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)
    pub fn arc1(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        counterclockwise: bool,
    ) -> Undefined {
        self.inner
            .call(
                "arc",
                &[
                    x.into(),
                    y.into(),
                    radius.into(),
                    start_angle.into(),
                    end_angle.into(),
                    counterclockwise.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl CanvasRenderingContext2D {
    /// The ellipse method.
    /// [`CanvasRenderingContext2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)
    pub fn ellipse0(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Undefined {
        self.inner
            .call(
                "ellipse",
                &[
                    x.into(),
                    y.into(),
                    radius_x.into(),
                    radius_y.into(),
                    rotation.into(),
                    start_angle.into(),
                    end_angle.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The ellipse method.
    /// [`CanvasRenderingContext2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)
    pub fn ellipse1(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        counterclockwise: bool,
    ) -> Undefined {
        self.inner
            .call(
                "ellipse",
                &[
                    x.into(),
                    y.into(),
                    radius_x.into(),
                    radius_y.into(),
                    rotation.into(),
                    start_angle.into(),
                    end_angle.into(),
                    counterclockwise.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
