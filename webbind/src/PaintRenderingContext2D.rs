use super::*;

/// The PaintRenderingContext2D class.
/// [`PaintRenderingContext2D`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaintRenderingContext2D {
    inner: Any,
}
impl FromVal for PaintRenderingContext2D {
    fn from_val(v: &Any) -> Self {
        PaintRenderingContext2D {
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
impl core::ops::Deref for PaintRenderingContext2D {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaintRenderingContext2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaintRenderingContext2D {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaintRenderingContext2D {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaintRenderingContext2D> for Any {
    fn from(s: PaintRenderingContext2D) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaintRenderingContext2D> for Any {
    fn from(s: &PaintRenderingContext2D) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaintRenderingContext2D);

impl PaintRenderingContext2D {
    /// The save method.
    /// [`PaintRenderingContext2D.save`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/save)
    pub fn save(&self) -> Undefined {
        self.inner.call("save", &[]).as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The restore method.
    /// [`PaintRenderingContext2D.restore`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/restore)
    pub fn restore(&self) -> Undefined {
        self.inner.call("restore", &[]).as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The reset method.
    /// [`PaintRenderingContext2D.reset`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The isContextLost method.
    /// [`PaintRenderingContext2D.isContextLost`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/isContextLost)
    pub fn is_context_lost(&self) -> bool {
        self.inner.call("isContextLost", &[]).as_::<bool>()
    }
}
impl PaintRenderingContext2D {
    /// The scale method.
    /// [`PaintRenderingContext2D.scale`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/scale)
    pub fn scale(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scale", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The rotate method.
    /// [`PaintRenderingContext2D.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/rotate)
    pub fn rotate(&self, angle: f64) -> Undefined {
        self.inner
            .call("rotate", &[angle.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The translate method.
    /// [`PaintRenderingContext2D.translate`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/translate)
    pub fn translate(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("translate", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The transform method.
    /// [`PaintRenderingContext2D.transform`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/transform)
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Undefined {
        self.inner
            .call(
                "transform",
                &[a.into(), b.into(), c.into(), d.into(), e.into(), f.into()],
            )
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The getTransform method.
    /// [`PaintRenderingContext2D.getTransform`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/getTransform)
    pub fn get_transform(&self) -> DOMMatrix {
        self.inner.call("getTransform", &[]).as_::<DOMMatrix>()
    }
}
impl PaintRenderingContext2D {
    /// The setTransform method.
    /// [`PaintRenderingContext2D.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/setTransform)
    pub fn set_transform0(&self) -> Undefined {
        self.inner.call("setTransform", &[]).as_::<Undefined>()
    }
    /// The setTransform method.
    /// [`PaintRenderingContext2D.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/setTransform)
    pub fn set_transform1(&self, transform: &DOMMatrix2DInit) -> Undefined {
        self.inner
            .call("setTransform", &[transform.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The resetTransform method.
    /// [`PaintRenderingContext2D.resetTransform`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/resetTransform)
    pub fn reset_transform(&self) -> Undefined {
        self.inner.call("resetTransform", &[]).as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `globalAlpha` attribute.
    /// [`PaintRenderingContext2D.globalAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/globalAlpha)
    pub fn global_alpha(&self) -> f64 {
        self.inner.get("globalAlpha").as_::<f64>()
    }

    /// Setter of the `globalAlpha` attribute.
    /// [`PaintRenderingContext2D.globalAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/globalAlpha)
    pub fn set_global_alpha(&mut self, value: f64) {
        self.inner.set("globalAlpha", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `globalCompositeOperation` attribute.
    /// [`PaintRenderingContext2D.globalCompositeOperation`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/globalCompositeOperation)
    pub fn global_composite_operation(&self) -> String {
        self.inner.get("globalCompositeOperation").as_::<String>()
    }

    /// Setter of the `globalCompositeOperation` attribute.
    /// [`PaintRenderingContext2D.globalCompositeOperation`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/globalCompositeOperation)
    pub fn set_global_composite_operation(&mut self, value: &str) {
        self.inner.set("globalCompositeOperation", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `imageSmoothingEnabled` attribute.
    /// [`PaintRenderingContext2D.imageSmoothingEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/imageSmoothingEnabled)
    pub fn image_smoothing_enabled(&self) -> bool {
        self.inner.get("imageSmoothingEnabled").as_::<bool>()
    }

    /// Setter of the `imageSmoothingEnabled` attribute.
    /// [`PaintRenderingContext2D.imageSmoothingEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/imageSmoothingEnabled)
    pub fn set_image_smoothing_enabled(&mut self, value: bool) {
        self.inner.set("imageSmoothingEnabled", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `imageSmoothingQuality` attribute.
    /// [`PaintRenderingContext2D.imageSmoothingQuality`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/imageSmoothingQuality)
    pub fn image_smoothing_quality(&self) -> ImageSmoothingQuality {
        self.inner
            .get("imageSmoothingQuality")
            .as_::<ImageSmoothingQuality>()
    }

    /// Setter of the `imageSmoothingQuality` attribute.
    /// [`PaintRenderingContext2D.imageSmoothingQuality`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/imageSmoothingQuality)
    pub fn set_image_smoothing_quality(&mut self, value: &ImageSmoothingQuality) {
        self.inner.set("imageSmoothingQuality", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `strokeStyle` attribute.
    /// [`PaintRenderingContext2D.strokeStyle`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/strokeStyle)
    pub fn stroke_style(&self) -> Any {
        self.inner.get("strokeStyle").as_::<Any>()
    }

    /// Setter of the `strokeStyle` attribute.
    /// [`PaintRenderingContext2D.strokeStyle`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/strokeStyle)
    pub fn set_stroke_style(&mut self, value: &Any) {
        self.inner.set("strokeStyle", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `fillStyle` attribute.
    /// [`PaintRenderingContext2D.fillStyle`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/fillStyle)
    pub fn fill_style(&self) -> Any {
        self.inner.get("fillStyle").as_::<Any>()
    }

    /// Setter of the `fillStyle` attribute.
    /// [`PaintRenderingContext2D.fillStyle`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/fillStyle)
    pub fn set_fill_style(&mut self, value: &Any) {
        self.inner.set("fillStyle", value);
    }
}
impl PaintRenderingContext2D {
    /// The createLinearGradient method.
    /// [`PaintRenderingContext2D.createLinearGradient`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/createLinearGradient)
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient {
        self.inner
            .call(
                "createLinearGradient",
                &[x0.into(), y0.into(), x1.into(), y1.into()],
            )
            .as_::<CanvasGradient>()
    }
}
impl PaintRenderingContext2D {
    /// The createRadialGradient method.
    /// [`PaintRenderingContext2D.createRadialGradient`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/createRadialGradient)
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
impl PaintRenderingContext2D {
    /// The createConicGradient method.
    /// [`PaintRenderingContext2D.createConicGradient`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/createConicGradient)
    pub fn create_conic_gradient(&self, start_angle: f64, x: f64, y: f64) -> CanvasGradient {
        self.inner
            .call(
                "createConicGradient",
                &[start_angle.into(), x.into(), y.into()],
            )
            .as_::<CanvasGradient>()
    }
}
impl PaintRenderingContext2D {
    /// The createPattern method.
    /// [`PaintRenderingContext2D.createPattern`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/createPattern)
    pub fn create_pattern(&self, image: &Any, repetition: &str) -> CanvasPattern {
        self.inner
            .call("createPattern", &[image.into(), repetition.into()])
            .as_::<CanvasPattern>()
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `shadowOffsetX` attribute.
    /// [`PaintRenderingContext2D.shadowOffsetX`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowOffsetX)
    pub fn shadow_offset_x(&self) -> f64 {
        self.inner.get("shadowOffsetX").as_::<f64>()
    }

    /// Setter of the `shadowOffsetX` attribute.
    /// [`PaintRenderingContext2D.shadowOffsetX`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowOffsetX)
    pub fn set_shadow_offset_x(&mut self, value: f64) {
        self.inner.set("shadowOffsetX", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `shadowOffsetY` attribute.
    /// [`PaintRenderingContext2D.shadowOffsetY`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowOffsetY)
    pub fn shadow_offset_y(&self) -> f64 {
        self.inner.get("shadowOffsetY").as_::<f64>()
    }

    /// Setter of the `shadowOffsetY` attribute.
    /// [`PaintRenderingContext2D.shadowOffsetY`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowOffsetY)
    pub fn set_shadow_offset_y(&mut self, value: f64) {
        self.inner.set("shadowOffsetY", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `shadowBlur` attribute.
    /// [`PaintRenderingContext2D.shadowBlur`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowBlur)
    pub fn shadow_blur(&self) -> f64 {
        self.inner.get("shadowBlur").as_::<f64>()
    }

    /// Setter of the `shadowBlur` attribute.
    /// [`PaintRenderingContext2D.shadowBlur`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowBlur)
    pub fn set_shadow_blur(&mut self, value: f64) {
        self.inner.set("shadowBlur", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `shadowColor` attribute.
    /// [`PaintRenderingContext2D.shadowColor`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowColor)
    pub fn shadow_color(&self) -> String {
        self.inner.get("shadowColor").as_::<String>()
    }

    /// Setter of the `shadowColor` attribute.
    /// [`PaintRenderingContext2D.shadowColor`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/shadowColor)
    pub fn set_shadow_color(&mut self, value: &str) {
        self.inner.set("shadowColor", value);
    }
}
impl PaintRenderingContext2D {
    /// The clearRect method.
    /// [`PaintRenderingContext2D.clearRect`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/clearRect)
    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("clearRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The fillRect method.
    /// [`PaintRenderingContext2D.fillRect`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/fillRect)
    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("fillRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The strokeRect method.
    /// [`PaintRenderingContext2D.strokeRect`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/strokeRect)
    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("strokeRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The beginPath method.
    /// [`PaintRenderingContext2D.beginPath`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/beginPath)
    pub fn begin_path(&self) -> Undefined {
        self.inner.call("beginPath", &[]).as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The fill method.
    /// [`PaintRenderingContext2D.fill`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/fill)
    pub fn fill0(&self, path: &Path2D) -> Undefined {
        self.inner.call("fill", &[path.into()]).as_::<Undefined>()
    }
    /// The fill method.
    /// [`PaintRenderingContext2D.fill`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/fill)
    pub fn fill1(&self, path: &Path2D, fill_rule: &CanvasFillRule) -> Undefined {
        self.inner
            .call("fill", &[path.into(), fill_rule.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The stroke method.
    /// [`PaintRenderingContext2D.stroke`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/stroke)
    pub fn stroke(&self, path: &Path2D) -> Undefined {
        self.inner.call("stroke", &[path.into()]).as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The clip method.
    /// [`PaintRenderingContext2D.clip`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/clip)
    pub fn clip0(&self, path: &Path2D) -> Undefined {
        self.inner.call("clip", &[path.into()]).as_::<Undefined>()
    }
    /// The clip method.
    /// [`PaintRenderingContext2D.clip`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/clip)
    pub fn clip1(&self, path: &Path2D, fill_rule: &CanvasFillRule) -> Undefined {
        self.inner
            .call("clip", &[path.into(), fill_rule.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The isPointInPath method.
    /// [`PaintRenderingContext2D.isPointInPath`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/isPointInPath)
    pub fn is_point_in_path0(&self, path: &Path2D, x: f64, y: f64) -> bool {
        self.inner
            .call("isPointInPath", &[path.into(), x.into(), y.into()])
            .as_::<bool>()
    }
    /// The isPointInPath method.
    /// [`PaintRenderingContext2D.isPointInPath`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/isPointInPath)
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
impl PaintRenderingContext2D {
    /// The isPointInStroke method.
    /// [`PaintRenderingContext2D.isPointInStroke`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/isPointInStroke)
    pub fn is_point_in_stroke(&self, path: &Path2D, x: f64, y: f64) -> bool {
        self.inner
            .call("isPointInStroke", &[path.into(), x.into(), y.into()])
            .as_::<bool>()
    }
}
impl PaintRenderingContext2D {
    /// The drawImage method.
    /// [`PaintRenderingContext2D.drawImage`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/drawImage)
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
impl PaintRenderingContext2D {
    /// Getter of the `lineWidth` attribute.
    /// [`PaintRenderingContext2D.lineWidth`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineWidth)
    pub fn line_width(&self) -> f64 {
        self.inner.get("lineWidth").as_::<f64>()
    }

    /// Setter of the `lineWidth` attribute.
    /// [`PaintRenderingContext2D.lineWidth`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineWidth)
    pub fn set_line_width(&mut self, value: f64) {
        self.inner.set("lineWidth", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `lineCap` attribute.
    /// [`PaintRenderingContext2D.lineCap`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineCap)
    pub fn line_cap(&self) -> CanvasLineCap {
        self.inner.get("lineCap").as_::<CanvasLineCap>()
    }

    /// Setter of the `lineCap` attribute.
    /// [`PaintRenderingContext2D.lineCap`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineCap)
    pub fn set_line_cap(&mut self, value: &CanvasLineCap) {
        self.inner.set("lineCap", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `lineJoin` attribute.
    /// [`PaintRenderingContext2D.lineJoin`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineJoin)
    pub fn line_join(&self) -> CanvasLineJoin {
        self.inner.get("lineJoin").as_::<CanvasLineJoin>()
    }

    /// Setter of the `lineJoin` attribute.
    /// [`PaintRenderingContext2D.lineJoin`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineJoin)
    pub fn set_line_join(&mut self, value: &CanvasLineJoin) {
        self.inner.set("lineJoin", value);
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `miterLimit` attribute.
    /// [`PaintRenderingContext2D.miterLimit`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/miterLimit)
    pub fn miter_limit(&self) -> f64 {
        self.inner.get("miterLimit").as_::<f64>()
    }

    /// Setter of the `miterLimit` attribute.
    /// [`PaintRenderingContext2D.miterLimit`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/miterLimit)
    pub fn set_miter_limit(&mut self, value: f64) {
        self.inner.set("miterLimit", value);
    }
}
impl PaintRenderingContext2D {
    /// The setLineDash method.
    /// [`PaintRenderingContext2D.setLineDash`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/setLineDash)
    pub fn set_line_dash(&self, segments: Sequence<f64>) -> Undefined {
        self.inner
            .call("setLineDash", &[segments.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The getLineDash method.
    /// [`PaintRenderingContext2D.getLineDash`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/getLineDash)
    pub fn get_line_dash(&self) -> Sequence<f64> {
        self.inner.call("getLineDash", &[]).as_::<Sequence<f64>>()
    }
}
impl PaintRenderingContext2D {
    /// Getter of the `lineDashOffset` attribute.
    /// [`PaintRenderingContext2D.lineDashOffset`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineDashOffset)
    pub fn line_dash_offset(&self) -> f64 {
        self.inner.get("lineDashOffset").as_::<f64>()
    }

    /// Setter of the `lineDashOffset` attribute.
    /// [`PaintRenderingContext2D.lineDashOffset`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineDashOffset)
    pub fn set_line_dash_offset(&mut self, value: f64) {
        self.inner.set("lineDashOffset", value);
    }
}
impl PaintRenderingContext2D {
    /// The closePath method.
    /// [`PaintRenderingContext2D.closePath`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/closePath)
    pub fn close_path(&self) -> Undefined {
        self.inner.call("closePath", &[]).as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The moveTo method.
    /// [`PaintRenderingContext2D.moveTo`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/moveTo)
    pub fn move_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("moveTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The lineTo method.
    /// [`PaintRenderingContext2D.lineTo`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/lineTo)
    pub fn line_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("lineTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The quadraticCurveTo method.
    /// [`PaintRenderingContext2D.quadraticCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/quadraticCurveTo)
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) -> Undefined {
        self.inner
            .call(
                "quadraticCurveTo",
                &[cpx.into(), cpy.into(), x.into(), y.into()],
            )
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The bezierCurveTo method.
    /// [`PaintRenderingContext2D.bezierCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/bezierCurveTo)
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
impl PaintRenderingContext2D {
    /// The arcTo method.
    /// [`PaintRenderingContext2D.arcTo`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/arcTo)
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Undefined {
        self.inner
            .call(
                "arcTo",
                &[x1.into(), y1.into(), x2.into(), y2.into(), radius.into()],
            )
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The rect method.
    /// [`PaintRenderingContext2D.rect`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/rect)
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("rect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The roundRect method.
    /// [`PaintRenderingContext2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/roundRect)
    pub fn round_rect0(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("roundRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
    /// The roundRect method.
    /// [`PaintRenderingContext2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/roundRect)
    pub fn round_rect1(&self, x: f64, y: f64, w: f64, h: f64, radii: &Any) -> Undefined {
        self.inner
            .call(
                "roundRect",
                &[x.into(), y.into(), w.into(), h.into(), radii.into()],
            )
            .as_::<Undefined>()
    }
}
impl PaintRenderingContext2D {
    /// The arc method.
    /// [`PaintRenderingContext2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/arc)
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
    /// [`PaintRenderingContext2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/arc)
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
impl PaintRenderingContext2D {
    /// The ellipse method.
    /// [`PaintRenderingContext2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/ellipse)
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
    /// [`PaintRenderingContext2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/PaintRenderingContext2D/ellipse)
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
