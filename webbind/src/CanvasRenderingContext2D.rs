use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasRenderingContext2DSettings {
    inner: emlite::Val,
}
impl FromVal for CanvasRenderingContext2DSettings {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasRenderingContext2DSettings { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CanvasRenderingContext2DSettings {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasRenderingContext2DSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CanvasRenderingContext2DSettings {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CanvasRenderingContext2DSettings {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CanvasRenderingContext2DSettings> for emlite::Val {
    fn from(s: CanvasRenderingContext2DSettings) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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

    pub fn set_color_space(&mut self, value: PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }

}
impl CanvasRenderingContext2DSettings {
    pub fn color_type(&self) -> CanvasColorType {
        self.inner.get("colorType").as_::<CanvasColorType>()
    }

    pub fn set_color_type(&mut self, value: CanvasColorType) {
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
    inner: emlite::Val,
}
impl FromVal for ImageDataSettings {
    fn from_val(v: &emlite::Val) -> Self {
        ImageDataSettings { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ImageDataSettings {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageDataSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ImageDataSettings {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ImageDataSettings {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<ImageDataSettings> for emlite::Val {
    fn from(s: ImageDataSettings) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageDataSettings {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }

}
impl ImageDataSettings {
    pub fn pixel_format(&self) -> ImageDataPixelFormat {
        self.inner.get("pixelFormat").as_::<ImageDataPixelFormat>()
    }

    pub fn set_pixel_format(&mut self, value: ImageDataPixelFormat) {
        self.inner.set("pixelFormat", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasRenderingContext2D {
    inner: emlite::Val,
}
impl FromVal for CanvasRenderingContext2D {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasRenderingContext2D { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CanvasRenderingContext2D {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasRenderingContext2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CanvasRenderingContext2D {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CanvasRenderingContext2D {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CanvasRenderingContext2D> for emlite::Val {
    fn from(s: CanvasRenderingContext2D) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CanvasRenderingContext2D);


impl CanvasRenderingContext2D {
    pub fn canvas(&self) -> HTMLCanvasElement {
        self.inner.get("canvas").as_::<HTMLCanvasElement>()
    }

}
impl CanvasRenderingContext2D {
    pub fn get_context_attributes(&self, ) -> CanvasRenderingContext2DSettings {
        self.inner.call("getContextAttributes", &[]).as_::<CanvasRenderingContext2DSettings>()
    }

}
impl CanvasRenderingContext2D {
    pub fn save(&self, ) -> Undefined {
        self.inner.call("save", &[]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn restore(&self, ) -> Undefined {
        self.inner.call("restore", &[]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn reset(&self, ) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn is_context_lost(&self, ) -> bool {
        self.inner.call("isContextLost", &[]).as_::<bool>()
    }

}
impl CanvasRenderingContext2D {
    pub fn scale(&self, x: f64, y: f64) -> Undefined {
        self.inner.call("scale", &[x.into(), y.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn rotate(&self, angle: f64) -> Undefined {
        self.inner.call("rotate", &[angle.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn translate(&self, x: f64, y: f64) -> Undefined {
        self.inner.call("translate", &[x.into(), y.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Undefined {
        self.inner.call("transform", &[a.into(), b.into(), c.into(), d.into(), e.into(), f.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn get_transform(&self, ) -> DOMMatrix {
        self.inner.call("getTransform", &[]).as_::<DOMMatrix>()
    }

}
impl CanvasRenderingContext2D {
    pub fn set_transform0(&self, ) -> Undefined {
        self.inner.call("setTransform", &[]).as_::<Undefined>()
    }

    pub fn set_transform1(&self, transform: DOMMatrix2DInit) -> Undefined {
        self.inner.call("setTransform", &[transform.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn reset_transform(&self, ) -> Undefined {
        self.inner.call("resetTransform", &[]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn global_alpha(&self) -> f64 {
        self.inner.get("globalAlpha").as_::<f64>()
    }

    pub fn set_global_alpha(&mut self, value: f64) {
        self.inner.set("globalAlpha", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn global_composite_operation(&self) -> DOMString {
        self.inner.get("globalCompositeOperation").as_::<DOMString>()
    }

    pub fn set_global_composite_operation(&mut self, value: DOMString) {
        self.inner.set("globalCompositeOperation", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn image_smoothing_enabled(&self) -> bool {
        self.inner.get("imageSmoothingEnabled").as_::<bool>()
    }

    pub fn set_image_smoothing_enabled(&mut self, value: bool) {
        self.inner.set("imageSmoothingEnabled", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn image_smoothing_quality(&self) -> ImageSmoothingQuality {
        self.inner.get("imageSmoothingQuality").as_::<ImageSmoothingQuality>()
    }

    pub fn set_image_smoothing_quality(&mut self, value: ImageSmoothingQuality) {
        self.inner.set("imageSmoothingQuality", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn stroke_style(&self) -> Any {
        self.inner.get("strokeStyle").as_::<Any>()
    }

    pub fn set_stroke_style(&mut self, value: Any) {
        self.inner.set("strokeStyle", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn fill_style(&self) -> Any {
        self.inner.get("fillStyle").as_::<Any>()
    }

    pub fn set_fill_style(&mut self, value: Any) {
        self.inner.set("fillStyle", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient {
        self.inner.call("createLinearGradient", &[x0.into(), y0.into(), x1.into(), y1.into(), ]).as_::<CanvasGradient>()
    }

}
impl CanvasRenderingContext2D {
    pub fn create_radial_gradient(&self, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> CanvasGradient {
        self.inner.call("createRadialGradient", &[x0.into(), y0.into(), r0.into(), x1.into(), y1.into(), r1.into(), ]).as_::<CanvasGradient>()
    }

}
impl CanvasRenderingContext2D {
    pub fn create_conic_gradient(&self, start_angle: f64, x: f64, y: f64) -> CanvasGradient {
        self.inner.call("createConicGradient", &[start_angle.into(), x.into(), y.into(), ]).as_::<CanvasGradient>()
    }

}
impl CanvasRenderingContext2D {
    pub fn create_pattern(&self, image: Any, repetition: DOMString) -> CanvasPattern {
        self.inner.call("createPattern", &[image.into(), repetition.into(), ]).as_::<CanvasPattern>()
    }

}
impl CanvasRenderingContext2D {
    pub fn shadow_offset_x(&self) -> f64 {
        self.inner.get("shadowOffsetX").as_::<f64>()
    }

    pub fn set_shadow_offset_x(&mut self, value: f64) {
        self.inner.set("shadowOffsetX", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn shadow_offset_y(&self) -> f64 {
        self.inner.get("shadowOffsetY").as_::<f64>()
    }

    pub fn set_shadow_offset_y(&mut self, value: f64) {
        self.inner.set("shadowOffsetY", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn shadow_blur(&self) -> f64 {
        self.inner.get("shadowBlur").as_::<f64>()
    }

    pub fn set_shadow_blur(&mut self, value: f64) {
        self.inner.set("shadowBlur", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn shadow_color(&self) -> DOMString {
        self.inner.get("shadowColor").as_::<DOMString>()
    }

    pub fn set_shadow_color(&mut self, value: DOMString) {
        self.inner.set("shadowColor", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn filter(&self) -> DOMString {
        self.inner.get("filter").as_::<DOMString>()
    }

    pub fn set_filter(&mut self, value: DOMString) {
        self.inner.set("filter", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner.call("clearRect", &[x.into(), y.into(), w.into(), h.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner.call("fillRect", &[x.into(), y.into(), w.into(), h.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner.call("strokeRect", &[x.into(), y.into(), w.into(), h.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn begin_path(&self, ) -> Undefined {
        self.inner.call("beginPath", &[]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn fill0(&self, path: Path2D) -> Undefined {
        self.inner.call("fill", &[path.into(), ]).as_::<Undefined>()
    }

    pub fn fill1(&self, path: Path2D, fill_rule: CanvasFillRule) -> Undefined {
        self.inner.call("fill", &[path.into(), fill_rule.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn stroke(&self, path: Path2D) -> Undefined {
        self.inner.call("stroke", &[path.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn clip0(&self, path: Path2D) -> Undefined {
        self.inner.call("clip", &[path.into(), ]).as_::<Undefined>()
    }

    pub fn clip1(&self, path: Path2D, fill_rule: CanvasFillRule) -> Undefined {
        self.inner.call("clip", &[path.into(), fill_rule.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn is_point_in_path0(&self, path: Path2D, x: f64, y: f64) -> bool {
        self.inner.call("isPointInPath", &[path.into(), x.into(), y.into(), ]).as_::<bool>()
    }

    pub fn is_point_in_path1(&self, path: Path2D, x: f64, y: f64, fill_rule: CanvasFillRule) -> bool {
        self.inner.call("isPointInPath", &[path.into(), x.into(), y.into(), fill_rule.into(), ]).as_::<bool>()
    }

}
impl CanvasRenderingContext2D {
    pub fn is_point_in_stroke(&self, path: Path2D, x: f64, y: f64) -> bool {
        self.inner.call("isPointInStroke", &[path.into(), x.into(), y.into(), ]).as_::<bool>()
    }

}
impl CanvasRenderingContext2D {
    pub fn draw_focus_if_needed(&self, path: Path2D, element: Element) -> Undefined {
        self.inner.call("drawFocusIfNeeded", &[path.into(), element.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn fill_text0(&self, text: DOMString, x: f64, y: f64) -> Undefined {
        self.inner.call("fillText", &[text.into(), x.into(), y.into(), ]).as_::<Undefined>()
    }

    pub fn fill_text1(&self, text: DOMString, x: f64, y: f64, max_width: f64) -> Undefined {
        self.inner.call("fillText", &[text.into(), x.into(), y.into(), max_width.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn stroke_text0(&self, text: DOMString, x: f64, y: f64) -> Undefined {
        self.inner.call("strokeText", &[text.into(), x.into(), y.into(), ]).as_::<Undefined>()
    }

    pub fn stroke_text1(&self, text: DOMString, x: f64, y: f64, max_width: f64) -> Undefined {
        self.inner.call("strokeText", &[text.into(), x.into(), y.into(), max_width.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn measure_text(&self, text: DOMString) -> TextMetrics {
        self.inner.call("measureText", &[text.into(), ]).as_::<TextMetrics>()
    }

}
impl CanvasRenderingContext2D {
    pub fn draw_image(&self, image: Any, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Undefined {
        self.inner.call("drawImage", &[image.into(), sx.into(), sy.into(), sw.into(), sh.into(), dx.into(), dy.into(), dw.into(), dh.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn create_image_data(&self, image_data: ImageData) -> ImageData {
        self.inner.call("createImageData", &[image_data.into(), ]).as_::<ImageData>()
    }

}
impl CanvasRenderingContext2D {
    pub fn get_image_data0(&self, sx: i32, sy: i32, sw: i32, sh: i32) -> ImageData {
        self.inner.call("getImageData", &[sx.into(), sy.into(), sw.into(), sh.into(), ]).as_::<ImageData>()
    }

    pub fn get_image_data1(&self, sx: i32, sy: i32, sw: i32, sh: i32, settings: ImageDataSettings) -> ImageData {
        self.inner.call("getImageData", &[sx.into(), sy.into(), sw.into(), sh.into(), settings.into(), ]).as_::<ImageData>()
    }

}
impl CanvasRenderingContext2D {
    pub fn put_image_data(&self, image_data: ImageData, dx: i32, dy: i32, dirty_x: i32, dirty_y: i32, dirty_width: i32, dirty_height: i32) -> Undefined {
        self.inner.call("putImageData", &[image_data.into(), dx.into(), dy.into(), dirty_x.into(), dirty_y.into(), dirty_width.into(), dirty_height.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn line_width(&self) -> f64 {
        self.inner.get("lineWidth").as_::<f64>()
    }

    pub fn set_line_width(&mut self, value: f64) {
        self.inner.set("lineWidth", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn line_cap(&self) -> CanvasLineCap {
        self.inner.get("lineCap").as_::<CanvasLineCap>()
    }

    pub fn set_line_cap(&mut self, value: CanvasLineCap) {
        self.inner.set("lineCap", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn line_join(&self) -> CanvasLineJoin {
        self.inner.get("lineJoin").as_::<CanvasLineJoin>()
    }

    pub fn set_line_join(&mut self, value: CanvasLineJoin) {
        self.inner.set("lineJoin", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn miter_limit(&self) -> f64 {
        self.inner.get("miterLimit").as_::<f64>()
    }

    pub fn set_miter_limit(&mut self, value: f64) {
        self.inner.set("miterLimit", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn set_line_dash(&self, segments: Sequence<f64>) -> Undefined {
        self.inner.call("setLineDash", &[segments.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn get_line_dash(&self, ) -> Sequence<f64> {
        self.inner.call("getLineDash", &[]).as_::<Sequence<f64>>()
    }

}
impl CanvasRenderingContext2D {
    pub fn line_dash_offset(&self) -> f64 {
        self.inner.get("lineDashOffset").as_::<f64>()
    }

    pub fn set_line_dash_offset(&mut self, value: f64) {
        self.inner.set("lineDashOffset", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn lang(&self) -> DOMString {
        self.inner.get("lang").as_::<DOMString>()
    }

    pub fn set_lang(&mut self, value: DOMString) {
        self.inner.set("lang", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn font(&self) -> DOMString {
        self.inner.get("font").as_::<DOMString>()
    }

    pub fn set_font(&mut self, value: DOMString) {
        self.inner.set("font", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn text_align(&self) -> CanvasTextAlign {
        self.inner.get("textAlign").as_::<CanvasTextAlign>()
    }

    pub fn set_text_align(&mut self, value: CanvasTextAlign) {
        self.inner.set("textAlign", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn text_baseline(&self) -> CanvasTextBaseline {
        self.inner.get("textBaseline").as_::<CanvasTextBaseline>()
    }

    pub fn set_text_baseline(&mut self, value: CanvasTextBaseline) {
        self.inner.set("textBaseline", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn direction(&self) -> CanvasDirection {
        self.inner.get("direction").as_::<CanvasDirection>()
    }

    pub fn set_direction(&mut self, value: CanvasDirection) {
        self.inner.set("direction", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn letter_spacing(&self) -> DOMString {
        self.inner.get("letterSpacing").as_::<DOMString>()
    }

    pub fn set_letter_spacing(&mut self, value: DOMString) {
        self.inner.set("letterSpacing", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn font_kerning(&self) -> CanvasFontKerning {
        self.inner.get("fontKerning").as_::<CanvasFontKerning>()
    }

    pub fn set_font_kerning(&mut self, value: CanvasFontKerning) {
        self.inner.set("fontKerning", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn font_stretch(&self) -> CanvasFontStretch {
        self.inner.get("fontStretch").as_::<CanvasFontStretch>()
    }

    pub fn set_font_stretch(&mut self, value: CanvasFontStretch) {
        self.inner.set("fontStretch", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn font_variant_caps(&self) -> CanvasFontVariantCaps {
        self.inner.get("fontVariantCaps").as_::<CanvasFontVariantCaps>()
    }

    pub fn set_font_variant_caps(&mut self, value: CanvasFontVariantCaps) {
        self.inner.set("fontVariantCaps", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn text_rendering(&self) -> CanvasTextRendering {
        self.inner.get("textRendering").as_::<CanvasTextRendering>()
    }

    pub fn set_text_rendering(&mut self, value: CanvasTextRendering) {
        self.inner.set("textRendering", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn word_spacing(&self) -> DOMString {
        self.inner.get("wordSpacing").as_::<DOMString>()
    }

    pub fn set_word_spacing(&mut self, value: DOMString) {
        self.inner.set("wordSpacing", value);
    }

}
impl CanvasRenderingContext2D {
    pub fn close_path(&self, ) -> Undefined {
        self.inner.call("closePath", &[]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn move_to(&self, x: f64, y: f64) -> Undefined {
        self.inner.call("moveTo", &[x.into(), y.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn line_to(&self, x: f64, y: f64) -> Undefined {
        self.inner.call("lineTo", &[x.into(), y.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) -> Undefined {
        self.inner.call("quadraticCurveTo", &[cpx.into(), cpy.into(), x.into(), y.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) -> Undefined {
        self.inner.call("bezierCurveTo", &[cp1x.into(), cp1y.into(), cp2x.into(), cp2y.into(), x.into(), y.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Undefined {
        self.inner.call("arcTo", &[x1.into(), y1.into(), x2.into(), y2.into(), radius.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner.call("rect", &[x.into(), y.into(), w.into(), h.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn round_rect0(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner.call("roundRect", &[x.into(), y.into(), w.into(), h.into(), ]).as_::<Undefined>()
    }

    pub fn round_rect1(&self, x: f64, y: f64, w: f64, h: f64, radii: Any) -> Undefined {
        self.inner.call("roundRect", &[x.into(), y.into(), w.into(), h.into(), radii.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn arc0(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Undefined {
        self.inner.call("arc", &[x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), ]).as_::<Undefined>()
    }

    pub fn arc1(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, counterclockwise: bool) -> Undefined {
        self.inner.call("arc", &[x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), counterclockwise.into(), ]).as_::<Undefined>()
    }

}
impl CanvasRenderingContext2D {
    pub fn ellipse0(&self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64) -> Undefined {
        self.inner.call("ellipse", &[x.into(), y.into(), radius_x.into(), radius_y.into(), rotation.into(), start_angle.into(), end_angle.into(), ]).as_::<Undefined>()
    }

    pub fn ellipse1(&self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64, counterclockwise: bool) -> Undefined {
        self.inner.call("ellipse", &[x.into(), y.into(), radius_x.into(), radius_y.into(), rotation.into(), start_angle.into(), end_angle.into(), counterclockwise.into(), ]).as_::<Undefined>()
    }

}
