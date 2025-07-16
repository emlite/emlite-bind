use super::*;

/// The OffscreenCanvasRenderingContext2D class.
/// [`OffscreenCanvasRenderingContext2D`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OffscreenCanvasRenderingContext2D {
    inner: Any,
}
impl FromVal for OffscreenCanvasRenderingContext2D {
    fn from_val(v: &Any) -> Self {
        OffscreenCanvasRenderingContext2D {
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
impl core::ops::Deref for OffscreenCanvasRenderingContext2D {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OffscreenCanvasRenderingContext2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OffscreenCanvasRenderingContext2D {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OffscreenCanvasRenderingContext2D {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OffscreenCanvasRenderingContext2D> for Any {
    fn from(s: OffscreenCanvasRenderingContext2D) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OffscreenCanvasRenderingContext2D> for Any {
    fn from(s: &OffscreenCanvasRenderingContext2D) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OffscreenCanvasRenderingContext2D);

impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `canvas` attribute.
    /// [`OffscreenCanvasRenderingContext2D.canvas`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/canvas)
    pub fn canvas(&self) -> OffscreenCanvas {
        self.inner.get("canvas").as_::<OffscreenCanvas>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The getContextAttributes method.
    /// [`OffscreenCanvasRenderingContext2D.getContextAttributes`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getContextAttributes)
    pub fn get_context_attributes(&self) -> CanvasRenderingContext2DSettings {
        self.inner
            .call("getContextAttributes", &[])
            .as_::<CanvasRenderingContext2DSettings>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The save method.
    /// [`OffscreenCanvasRenderingContext2D.save`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/save)
    pub fn save(&self) -> Undefined {
        self.inner.call("save", &[]).as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The restore method.
    /// [`OffscreenCanvasRenderingContext2D.restore`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/restore)
    pub fn restore(&self) -> Undefined {
        self.inner.call("restore", &[]).as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The reset method.
    /// [`OffscreenCanvasRenderingContext2D.reset`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The isContextLost method.
    /// [`OffscreenCanvasRenderingContext2D.isContextLost`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isContextLost)
    pub fn is_context_lost(&self) -> bool {
        self.inner.call("isContextLost", &[]).as_::<bool>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The scale method.
    /// [`OffscreenCanvasRenderingContext2D.scale`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/scale)
    pub fn scale(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scale", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The rotate method.
    /// [`OffscreenCanvasRenderingContext2D.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/rotate)
    pub fn rotate(&self, angle: f64) -> Undefined {
        self.inner
            .call("rotate", &[angle.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The translate method.
    /// [`OffscreenCanvasRenderingContext2D.translate`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/translate)
    pub fn translate(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("translate", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The transform method.
    /// [`OffscreenCanvasRenderingContext2D.transform`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/transform)
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Undefined {
        self.inner
            .call(
                "transform",
                &[a.into(), b.into(), c.into(), d.into(), e.into(), f.into()],
            )
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The getTransform method.
    /// [`OffscreenCanvasRenderingContext2D.getTransform`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getTransform)
    pub fn get_transform(&self) -> DOMMatrix {
        self.inner.call("getTransform", &[]).as_::<DOMMatrix>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The setTransform method.
    /// [`OffscreenCanvasRenderingContext2D.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setTransform)
    pub fn set_transform0(&self) -> Undefined {
        self.inner.call("setTransform", &[]).as_::<Undefined>()
    }
    /// The setTransform method.
    /// [`OffscreenCanvasRenderingContext2D.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setTransform)
    pub fn set_transform1(&self, transform: &DOMMatrix2DInit) -> Undefined {
        self.inner
            .call("setTransform", &[transform.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The resetTransform method.
    /// [`OffscreenCanvasRenderingContext2D.resetTransform`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/resetTransform)
    pub fn reset_transform(&self) -> Undefined {
        self.inner.call("resetTransform", &[]).as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `globalAlpha` attribute.
    /// [`OffscreenCanvasRenderingContext2D.globalAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalAlpha)
    pub fn global_alpha(&self) -> f64 {
        self.inner.get("globalAlpha").as_::<f64>()
    }

    /// Setter of the `globalAlpha` attribute.
    /// [`OffscreenCanvasRenderingContext2D.globalAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalAlpha)
    pub fn set_global_alpha(&mut self, value: f64) {
        self.inner.set("globalAlpha", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `globalCompositeOperation` attribute.
    /// [`OffscreenCanvasRenderingContext2D.globalCompositeOperation`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalCompositeOperation)
    pub fn global_composite_operation(&self) -> String {
        self.inner.get("globalCompositeOperation").as_::<String>()
    }

    /// Setter of the `globalCompositeOperation` attribute.
    /// [`OffscreenCanvasRenderingContext2D.globalCompositeOperation`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalCompositeOperation)
    pub fn set_global_composite_operation(&mut self, value: &str) {
        self.inner.set("globalCompositeOperation", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `imageSmoothingEnabled` attribute.
    /// [`OffscreenCanvasRenderingContext2D.imageSmoothingEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingEnabled)
    pub fn image_smoothing_enabled(&self) -> bool {
        self.inner.get("imageSmoothingEnabled").as_::<bool>()
    }

    /// Setter of the `imageSmoothingEnabled` attribute.
    /// [`OffscreenCanvasRenderingContext2D.imageSmoothingEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingEnabled)
    pub fn set_image_smoothing_enabled(&mut self, value: bool) {
        self.inner.set("imageSmoothingEnabled", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `imageSmoothingQuality` attribute.
    /// [`OffscreenCanvasRenderingContext2D.imageSmoothingQuality`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingQuality)
    pub fn image_smoothing_quality(&self) -> ImageSmoothingQuality {
        self.inner
            .get("imageSmoothingQuality")
            .as_::<ImageSmoothingQuality>()
    }

    /// Setter of the `imageSmoothingQuality` attribute.
    /// [`OffscreenCanvasRenderingContext2D.imageSmoothingQuality`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingQuality)
    pub fn set_image_smoothing_quality(&mut self, value: &ImageSmoothingQuality) {
        self.inner.set("imageSmoothingQuality", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `strokeStyle` attribute.
    /// [`OffscreenCanvasRenderingContext2D.strokeStyle`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)
    pub fn stroke_style(&self) -> Any {
        self.inner.get("strokeStyle").as_::<Any>()
    }

    /// Setter of the `strokeStyle` attribute.
    /// [`OffscreenCanvasRenderingContext2D.strokeStyle`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)
    pub fn set_stroke_style(&mut self, value: &Any) {
        self.inner.set("strokeStyle", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `fillStyle` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fillStyle`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)
    pub fn fill_style(&self) -> Any {
        self.inner.get("fillStyle").as_::<Any>()
    }

    /// Setter of the `fillStyle` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fillStyle`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)
    pub fn set_fill_style(&mut self, value: &Any) {
        self.inner.set("fillStyle", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The createLinearGradient method.
    /// [`OffscreenCanvasRenderingContext2D.createLinearGradient`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createLinearGradient)
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient {
        self.inner
            .call(
                "createLinearGradient",
                &[x0.into(), y0.into(), x1.into(), y1.into()],
            )
            .as_::<CanvasGradient>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The createRadialGradient method.
    /// [`OffscreenCanvasRenderingContext2D.createRadialGradient`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createRadialGradient)
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
impl OffscreenCanvasRenderingContext2D {
    /// The createConicGradient method.
    /// [`OffscreenCanvasRenderingContext2D.createConicGradient`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createConicGradient)
    pub fn create_conic_gradient(&self, start_angle: f64, x: f64, y: f64) -> CanvasGradient {
        self.inner
            .call(
                "createConicGradient",
                &[start_angle.into(), x.into(), y.into()],
            )
            .as_::<CanvasGradient>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The createPattern method.
    /// [`OffscreenCanvasRenderingContext2D.createPattern`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)
    pub fn create_pattern(&self, image: &Any, repetition: &str) -> CanvasPattern {
        self.inner
            .call("createPattern", &[image.into(), repetition.into()])
            .as_::<CanvasPattern>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `shadowOffsetX` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowOffsetX`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetX)
    pub fn shadow_offset_x(&self) -> f64 {
        self.inner.get("shadowOffsetX").as_::<f64>()
    }

    /// Setter of the `shadowOffsetX` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowOffsetX`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetX)
    pub fn set_shadow_offset_x(&mut self, value: f64) {
        self.inner.set("shadowOffsetX", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `shadowOffsetY` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowOffsetY`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetY)
    pub fn shadow_offset_y(&self) -> f64 {
        self.inner.get("shadowOffsetY").as_::<f64>()
    }

    /// Setter of the `shadowOffsetY` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowOffsetY`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetY)
    pub fn set_shadow_offset_y(&mut self, value: f64) {
        self.inner.set("shadowOffsetY", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `shadowBlur` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowBlur`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowBlur)
    pub fn shadow_blur(&self) -> f64 {
        self.inner.get("shadowBlur").as_::<f64>()
    }

    /// Setter of the `shadowBlur` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowBlur`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowBlur)
    pub fn set_shadow_blur(&mut self, value: f64) {
        self.inner.set("shadowBlur", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `shadowColor` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowColor`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowColor)
    pub fn shadow_color(&self) -> String {
        self.inner.get("shadowColor").as_::<String>()
    }

    /// Setter of the `shadowColor` attribute.
    /// [`OffscreenCanvasRenderingContext2D.shadowColor`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowColor)
    pub fn set_shadow_color(&mut self, value: &str) {
        self.inner.set("shadowColor", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `filter` attribute.
    /// [`OffscreenCanvasRenderingContext2D.filter`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/filter)
    pub fn filter(&self) -> String {
        self.inner.get("filter").as_::<String>()
    }

    /// Setter of the `filter` attribute.
    /// [`OffscreenCanvasRenderingContext2D.filter`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/filter)
    pub fn set_filter(&mut self, value: &str) {
        self.inner.set("filter", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The clearRect method.
    /// [`OffscreenCanvasRenderingContext2D.clearRect`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clearRect)
    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("clearRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The fillRect method.
    /// [`OffscreenCanvasRenderingContext2D.fillRect`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillRect)
    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("fillRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The strokeRect method.
    /// [`OffscreenCanvasRenderingContext2D.strokeRect`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeRect)
    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("strokeRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The beginPath method.
    /// [`OffscreenCanvasRenderingContext2D.beginPath`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/beginPath)
    pub fn begin_path(&self) -> Undefined {
        self.inner.call("beginPath", &[]).as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The fill method.
    /// [`OffscreenCanvasRenderingContext2D.fill`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)
    pub fn fill0(&self, path: &Path2D) -> Undefined {
        self.inner.call("fill", &[path.into()]).as_::<Undefined>()
    }
    /// The fill method.
    /// [`OffscreenCanvasRenderingContext2D.fill`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)
    pub fn fill1(&self, path: &Path2D, fill_rule: &CanvasFillRule) -> Undefined {
        self.inner
            .call("fill", &[path.into(), fill_rule.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The stroke method.
    /// [`OffscreenCanvasRenderingContext2D.stroke`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/stroke)
    pub fn stroke(&self, path: &Path2D) -> Undefined {
        self.inner.call("stroke", &[path.into()]).as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The clip method.
    /// [`OffscreenCanvasRenderingContext2D.clip`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)
    pub fn clip0(&self, path: &Path2D) -> Undefined {
        self.inner.call("clip", &[path.into()]).as_::<Undefined>()
    }
    /// The clip method.
    /// [`OffscreenCanvasRenderingContext2D.clip`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)
    pub fn clip1(&self, path: &Path2D, fill_rule: &CanvasFillRule) -> Undefined {
        self.inner
            .call("clip", &[path.into(), fill_rule.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The isPointInPath method.
    /// [`OffscreenCanvasRenderingContext2D.isPointInPath`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)
    pub fn is_point_in_path0(&self, path: &Path2D, x: f64, y: f64) -> bool {
        self.inner
            .call("isPointInPath", &[path.into(), x.into(), y.into()])
            .as_::<bool>()
    }
    /// The isPointInPath method.
    /// [`OffscreenCanvasRenderingContext2D.isPointInPath`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)
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
impl OffscreenCanvasRenderingContext2D {
    /// The isPointInStroke method.
    /// [`OffscreenCanvasRenderingContext2D.isPointInStroke`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInStroke)
    pub fn is_point_in_stroke(&self, path: &Path2D, x: f64, y: f64) -> bool {
        self.inner
            .call("isPointInStroke", &[path.into(), x.into(), y.into()])
            .as_::<bool>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The fillText method.
    /// [`OffscreenCanvasRenderingContext2D.fillText`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillText)
    pub fn fill_text0(&self, text: &str, x: f64, y: f64) -> Undefined {
        self.inner
            .call("fillText", &[text.into(), x.into(), y.into()])
            .as_::<Undefined>()
    }
    /// The fillText method.
    /// [`OffscreenCanvasRenderingContext2D.fillText`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillText)
    pub fn fill_text1(&self, text: &str, x: f64, y: f64, max_width: f64) -> Undefined {
        self.inner
            .call(
                "fillText",
                &[text.into(), x.into(), y.into(), max_width.into()],
            )
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The strokeText method.
    /// [`OffscreenCanvasRenderingContext2D.strokeText`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeText)
    pub fn stroke_text0(&self, text: &str, x: f64, y: f64) -> Undefined {
        self.inner
            .call("strokeText", &[text.into(), x.into(), y.into()])
            .as_::<Undefined>()
    }
    /// The strokeText method.
    /// [`OffscreenCanvasRenderingContext2D.strokeText`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeText)
    pub fn stroke_text1(&self, text: &str, x: f64, y: f64, max_width: f64) -> Undefined {
        self.inner
            .call(
                "strokeText",
                &[text.into(), x.into(), y.into(), max_width.into()],
            )
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The measureText method.
    /// [`OffscreenCanvasRenderingContext2D.measureText`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/measureText)
    pub fn measure_text(&self, text: &str) -> TextMetrics {
        self.inner
            .call("measureText", &[text.into()])
            .as_::<TextMetrics>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The drawImage method.
    /// [`OffscreenCanvasRenderingContext2D.drawImage`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)
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
impl OffscreenCanvasRenderingContext2D {
    /// The createImageData method.
    /// [`OffscreenCanvasRenderingContext2D.createImageData`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createImageData)
    pub fn create_image_data(&self, image_data: &ImageData) -> ImageData {
        self.inner
            .call("createImageData", &[image_data.into()])
            .as_::<ImageData>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The getImageData method.
    /// [`OffscreenCanvasRenderingContext2D.getImageData`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getImageData)
    pub fn get_image_data0(&self, sx: i32, sy: i32, sw: i32, sh: i32) -> ImageData {
        self.inner
            .call(
                "getImageData",
                &[sx.into(), sy.into(), sw.into(), sh.into()],
            )
            .as_::<ImageData>()
    }
    /// The getImageData method.
    /// [`OffscreenCanvasRenderingContext2D.getImageData`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getImageData)
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
impl OffscreenCanvasRenderingContext2D {
    /// The putImageData method.
    /// [`OffscreenCanvasRenderingContext2D.putImageData`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/putImageData)
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
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `lineWidth` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineWidth`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineWidth)
    pub fn line_width(&self) -> f64 {
        self.inner.get("lineWidth").as_::<f64>()
    }

    /// Setter of the `lineWidth` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineWidth`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineWidth)
    pub fn set_line_width(&mut self, value: f64) {
        self.inner.set("lineWidth", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `lineCap` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineCap`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineCap)
    pub fn line_cap(&self) -> CanvasLineCap {
        self.inner.get("lineCap").as_::<CanvasLineCap>()
    }

    /// Setter of the `lineCap` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineCap`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineCap)
    pub fn set_line_cap(&mut self, value: &CanvasLineCap) {
        self.inner.set("lineCap", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `lineJoin` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineJoin`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineJoin)
    pub fn line_join(&self) -> CanvasLineJoin {
        self.inner.get("lineJoin").as_::<CanvasLineJoin>()
    }

    /// Setter of the `lineJoin` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineJoin`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineJoin)
    pub fn set_line_join(&mut self, value: &CanvasLineJoin) {
        self.inner.set("lineJoin", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `miterLimit` attribute.
    /// [`OffscreenCanvasRenderingContext2D.miterLimit`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/miterLimit)
    pub fn miter_limit(&self) -> f64 {
        self.inner.get("miterLimit").as_::<f64>()
    }

    /// Setter of the `miterLimit` attribute.
    /// [`OffscreenCanvasRenderingContext2D.miterLimit`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/miterLimit)
    pub fn set_miter_limit(&mut self, value: f64) {
        self.inner.set("miterLimit", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The setLineDash method.
    /// [`OffscreenCanvasRenderingContext2D.setLineDash`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setLineDash)
    pub fn set_line_dash(&self, segments: Sequence<f64>) -> Undefined {
        self.inner
            .call("setLineDash", &[segments.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The getLineDash method.
    /// [`OffscreenCanvasRenderingContext2D.getLineDash`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getLineDash)
    pub fn get_line_dash(&self) -> Sequence<f64> {
        self.inner.call("getLineDash", &[]).as_::<Sequence<f64>>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `lineDashOffset` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineDashOffset`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineDashOffset)
    pub fn line_dash_offset(&self) -> f64 {
        self.inner.get("lineDashOffset").as_::<f64>()
    }

    /// Setter of the `lineDashOffset` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lineDashOffset`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineDashOffset)
    pub fn set_line_dash_offset(&mut self, value: f64) {
        self.inner.set("lineDashOffset", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `lang` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lang`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lang)
    pub fn lang(&self) -> String {
        self.inner.get("lang").as_::<String>()
    }

    /// Setter of the `lang` attribute.
    /// [`OffscreenCanvasRenderingContext2D.lang`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lang)
    pub fn set_lang(&mut self, value: &str) {
        self.inner.set("lang", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `font` attribute.
    /// [`OffscreenCanvasRenderingContext2D.font`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/font)
    pub fn font(&self) -> String {
        self.inner.get("font").as_::<String>()
    }

    /// Setter of the `font` attribute.
    /// [`OffscreenCanvasRenderingContext2D.font`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/font)
    pub fn set_font(&mut self, value: &str) {
        self.inner.set("font", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `textAlign` attribute.
    /// [`OffscreenCanvasRenderingContext2D.textAlign`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textAlign)
    pub fn text_align(&self) -> CanvasTextAlign {
        self.inner.get("textAlign").as_::<CanvasTextAlign>()
    }

    /// Setter of the `textAlign` attribute.
    /// [`OffscreenCanvasRenderingContext2D.textAlign`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textAlign)
    pub fn set_text_align(&mut self, value: &CanvasTextAlign) {
        self.inner.set("textAlign", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `textBaseline` attribute.
    /// [`OffscreenCanvasRenderingContext2D.textBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textBaseline)
    pub fn text_baseline(&self) -> CanvasTextBaseline {
        self.inner.get("textBaseline").as_::<CanvasTextBaseline>()
    }

    /// Setter of the `textBaseline` attribute.
    /// [`OffscreenCanvasRenderingContext2D.textBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textBaseline)
    pub fn set_text_baseline(&mut self, value: &CanvasTextBaseline) {
        self.inner.set("textBaseline", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `direction` attribute.
    /// [`OffscreenCanvasRenderingContext2D.direction`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/direction)
    pub fn direction(&self) -> CanvasDirection {
        self.inner.get("direction").as_::<CanvasDirection>()
    }

    /// Setter of the `direction` attribute.
    /// [`OffscreenCanvasRenderingContext2D.direction`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/direction)
    pub fn set_direction(&mut self, value: &CanvasDirection) {
        self.inner.set("direction", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `letterSpacing` attribute.
    /// [`OffscreenCanvasRenderingContext2D.letterSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/letterSpacing)
    pub fn letter_spacing(&self) -> String {
        self.inner.get("letterSpacing").as_::<String>()
    }

    /// Setter of the `letterSpacing` attribute.
    /// [`OffscreenCanvasRenderingContext2D.letterSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/letterSpacing)
    pub fn set_letter_spacing(&mut self, value: &str) {
        self.inner.set("letterSpacing", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `fontKerning` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fontKerning`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fontKerning)
    pub fn font_kerning(&self) -> CanvasFontKerning {
        self.inner.get("fontKerning").as_::<CanvasFontKerning>()
    }

    /// Setter of the `fontKerning` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fontKerning`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fontKerning)
    pub fn set_font_kerning(&mut self, value: &CanvasFontKerning) {
        self.inner.set("fontKerning", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `fontStretch` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fontStretch`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fontStretch)
    pub fn font_stretch(&self) -> CanvasFontStretch {
        self.inner.get("fontStretch").as_::<CanvasFontStretch>()
    }

    /// Setter of the `fontStretch` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fontStretch`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fontStretch)
    pub fn set_font_stretch(&mut self, value: &CanvasFontStretch) {
        self.inner.set("fontStretch", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `fontVariantCaps` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fontVariantCaps`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fontVariantCaps)
    pub fn font_variant_caps(&self) -> CanvasFontVariantCaps {
        self.inner
            .get("fontVariantCaps")
            .as_::<CanvasFontVariantCaps>()
    }

    /// Setter of the `fontVariantCaps` attribute.
    /// [`OffscreenCanvasRenderingContext2D.fontVariantCaps`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fontVariantCaps)
    pub fn set_font_variant_caps(&mut self, value: &CanvasFontVariantCaps) {
        self.inner.set("fontVariantCaps", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `textRendering` attribute.
    /// [`OffscreenCanvasRenderingContext2D.textRendering`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textRendering)
    pub fn text_rendering(&self) -> CanvasTextRendering {
        self.inner.get("textRendering").as_::<CanvasTextRendering>()
    }

    /// Setter of the `textRendering` attribute.
    /// [`OffscreenCanvasRenderingContext2D.textRendering`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textRendering)
    pub fn set_text_rendering(&mut self, value: &CanvasTextRendering) {
        self.inner.set("textRendering", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// Getter of the `wordSpacing` attribute.
    /// [`OffscreenCanvasRenderingContext2D.wordSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/wordSpacing)
    pub fn word_spacing(&self) -> String {
        self.inner.get("wordSpacing").as_::<String>()
    }

    /// Setter of the `wordSpacing` attribute.
    /// [`OffscreenCanvasRenderingContext2D.wordSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/wordSpacing)
    pub fn set_word_spacing(&mut self, value: &str) {
        self.inner.set("wordSpacing", value);
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The closePath method.
    /// [`OffscreenCanvasRenderingContext2D.closePath`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/closePath)
    pub fn close_path(&self) -> Undefined {
        self.inner.call("closePath", &[]).as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The moveTo method.
    /// [`OffscreenCanvasRenderingContext2D.moveTo`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/moveTo)
    pub fn move_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("moveTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The lineTo method.
    /// [`OffscreenCanvasRenderingContext2D.lineTo`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineTo)
    pub fn line_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("lineTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The quadraticCurveTo method.
    /// [`OffscreenCanvasRenderingContext2D.quadraticCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/quadraticCurveTo)
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) -> Undefined {
        self.inner
            .call(
                "quadraticCurveTo",
                &[cpx.into(), cpy.into(), x.into(), y.into()],
            )
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The bezierCurveTo method.
    /// [`OffscreenCanvasRenderingContext2D.bezierCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/bezierCurveTo)
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
impl OffscreenCanvasRenderingContext2D {
    /// The arcTo method.
    /// [`OffscreenCanvasRenderingContext2D.arcTo`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arcTo)
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Undefined {
        self.inner
            .call(
                "arcTo",
                &[x1.into(), y1.into(), x2.into(), y2.into(), radius.into()],
            )
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The rect method.
    /// [`OffscreenCanvasRenderingContext2D.rect`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/rect)
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("rect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The roundRect method.
    /// [`OffscreenCanvasRenderingContext2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/roundRect)
    pub fn round_rect0(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("roundRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
    /// The roundRect method.
    /// [`OffscreenCanvasRenderingContext2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/roundRect)
    pub fn round_rect1(&self, x: f64, y: f64, w: f64, h: f64, radii: &Any) -> Undefined {
        self.inner
            .call(
                "roundRect",
                &[x.into(), y.into(), w.into(), h.into(), radii.into()],
            )
            .as_::<Undefined>()
    }
}
impl OffscreenCanvasRenderingContext2D {
    /// The arc method.
    /// [`OffscreenCanvasRenderingContext2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arc)
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
    /// [`OffscreenCanvasRenderingContext2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arc)
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
impl OffscreenCanvasRenderingContext2D {
    /// The ellipse method.
    /// [`OffscreenCanvasRenderingContext2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/ellipse)
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
    /// [`OffscreenCanvasRenderingContext2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/ellipse)
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
