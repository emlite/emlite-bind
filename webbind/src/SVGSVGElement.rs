use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMMatrix2DInit {
    inner: emlite::Val,
}
impl FromVal for DOMMatrix2DInit {
    fn from_val(v: &emlite::Val) -> Self {
        DOMMatrix2DInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMMatrix2DInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMMatrix2DInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMMatrix2DInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMMatrix2DInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMMatrix2DInit> for emlite::Val {
    fn from(s: DOMMatrix2DInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMMatrix2DInit {
    pub fn a(&self) -> f64 {
        self.inner.get("a").as_::<f64>()
    }

    pub fn set_a(&mut self, value: f64) {
        self.inner.set("a", value);
    }
}
impl DOMMatrix2DInit {
    pub fn b(&self) -> f64 {
        self.inner.get("b").as_::<f64>()
    }

    pub fn set_b(&mut self, value: f64) {
        self.inner.set("b", value);
    }
}
impl DOMMatrix2DInit {
    pub fn c(&self) -> f64 {
        self.inner.get("c").as_::<f64>()
    }

    pub fn set_c(&mut self, value: f64) {
        self.inner.set("c", value);
    }
}
impl DOMMatrix2DInit {
    pub fn d(&self) -> f64 {
        self.inner.get("d").as_::<f64>()
    }

    pub fn set_d(&mut self, value: f64) {
        self.inner.set("d", value);
    }
}
impl DOMMatrix2DInit {
    pub fn e(&self) -> f64 {
        self.inner.get("e").as_::<f64>()
    }

    pub fn set_e(&mut self, value: f64) {
        self.inner.set("e", value);
    }
}
impl DOMMatrix2DInit {
    pub fn f(&self) -> f64 {
        self.inner.get("f").as_::<f64>()
    }

    pub fn set_f(&mut self, value: f64) {
        self.inner.set("f", value);
    }
}
impl DOMMatrix2DInit {
    pub fn m11(&self) -> f64 {
        self.inner.get("m11").as_::<f64>()
    }

    pub fn set_m11(&mut self, value: f64) {
        self.inner.set("m11", value);
    }
}
impl DOMMatrix2DInit {
    pub fn m12(&self) -> f64 {
        self.inner.get("m12").as_::<f64>()
    }

    pub fn set_m12(&mut self, value: f64) {
        self.inner.set("m12", value);
    }
}
impl DOMMatrix2DInit {
    pub fn m21(&self) -> f64 {
        self.inner.get("m21").as_::<f64>()
    }

    pub fn set_m21(&mut self, value: f64) {
        self.inner.set("m21", value);
    }
}
impl DOMMatrix2DInit {
    pub fn m22(&self) -> f64 {
        self.inner.get("m22").as_::<f64>()
    }

    pub fn set_m22(&mut self, value: f64) {
        self.inner.set("m22", value);
    }
}
impl DOMMatrix2DInit {
    pub fn m41(&self) -> f64 {
        self.inner.get("m41").as_::<f64>()
    }

    pub fn set_m41(&mut self, value: f64) {
        self.inner.set("m41", value);
    }
}
impl DOMMatrix2DInit {
    pub fn m42(&self) -> f64 {
        self.inner.get("m42").as_::<f64>()
    }

    pub fn set_m42(&mut self, value: f64) {
        self.inner.set("m42", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGSVGElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGSVGElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGSVGElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGSVGElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGSVGElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGSVGElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGSVGElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGSVGElement> for emlite::Val {
    fn from(s: SVGSVGElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGSVGElement);

impl SVGSVGElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    pub fn current_scale(&self) -> f32 {
        self.inner.get("currentScale").as_::<f32>()
    }

    pub fn set_current_scale(&mut self, value: f32) {
        self.inner.set("currentScale", value);
    }
}
impl SVGSVGElement {
    pub fn current_translate(&self) -> DOMPointReadOnly {
        self.inner.get("currentTranslate").as_::<DOMPointReadOnly>()
    }
}
impl SVGSVGElement {
    pub fn get_intersection_list(
        &self,
        rect: DOMRectReadOnly,
        reference_element: SVGElement,
    ) -> NodeList {
        self.inner
            .call(
                "getIntersectionList",
                &[rect.into(), reference_element.into()],
            )
            .as_::<NodeList>()
    }
}
impl SVGSVGElement {
    pub fn get_enclosure_list(
        &self,
        rect: DOMRectReadOnly,
        reference_element: SVGElement,
    ) -> NodeList {
        self.inner
            .call("getEnclosureList", &[rect.into(), reference_element.into()])
            .as_::<NodeList>()
    }
}
impl SVGSVGElement {
    pub fn check_intersection(&self, element: SVGElement, rect: DOMRectReadOnly) -> bool {
        self.inner
            .call("checkIntersection", &[element.into(), rect.into()])
            .as_::<bool>()
    }
}
impl SVGSVGElement {
    pub fn check_enclosure(&self, element: SVGElement, rect: DOMRectReadOnly) -> bool {
        self.inner
            .call("checkEnclosure", &[element.into(), rect.into()])
            .as_::<bool>()
    }
}
impl SVGSVGElement {
    pub fn deselect_all(&self) -> Undefined {
        self.inner.call("deselectAll", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_number(&self) -> SVGNumber {
        self.inner.call("createSVGNumber", &[]).as_::<SVGNumber>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_length(&self) -> SVGLength {
        self.inner.call("createSVGLength", &[]).as_::<SVGLength>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_angle(&self) -> SVGAngle {
        self.inner.call("createSVGAngle", &[]).as_::<SVGAngle>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_point(&self) -> DOMPoint {
        self.inner.call("createSVGPoint", &[]).as_::<DOMPoint>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_matrix(&self) -> DOMMatrix {
        self.inner.call("createSVGMatrix", &[]).as_::<DOMMatrix>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_rect(&self) -> DOMRect {
        self.inner.call("createSVGRect", &[]).as_::<DOMRect>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_transform(&self) -> SVGTransform {
        self.inner
            .call("createSVGTransform", &[])
            .as_::<SVGTransform>()
    }
}
impl SVGSVGElement {
    pub fn create_svg_transform_from_matrix0(&self) -> SVGTransform {
        self.inner
            .call("createSVGTransformFromMatrix", &[])
            .as_::<SVGTransform>()
    }

    pub fn create_svg_transform_from_matrix1(&self, matrix: DOMMatrix2DInit) -> SVGTransform {
        self.inner
            .call("createSVGTransformFromMatrix", &[matrix.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGSVGElement {
    pub fn get_element_by_id(&self, element_id: DOMString) -> Element {
        self.inner
            .call("getElementById", &[element_id.into()])
            .as_::<Element>()
    }
}
impl SVGSVGElement {
    pub fn suspend_redraw(&self, max_wait_milliseconds: u32) -> u32 {
        self.inner
            .call("suspendRedraw", &[max_wait_milliseconds.into()])
            .as_::<u32>()
    }
}
impl SVGSVGElement {
    pub fn unsuspend_redraw(&self, suspend_handle_id: u32) -> Undefined {
        self.inner
            .call("unsuspendRedraw", &[suspend_handle_id.into()])
            .as_::<Undefined>()
    }
}
impl SVGSVGElement {
    pub fn unsuspend_redraw_all(&self) -> Undefined {
        self.inner
            .call("unsuspendRedrawAll", &[])
            .as_::<Undefined>()
    }
}
impl SVGSVGElement {
    pub fn force_redraw(&self) -> Undefined {
        self.inner.call("forceRedraw", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    pub fn pause_animations(&self) -> Undefined {
        self.inner.call("pauseAnimations", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    pub fn unpause_animations(&self) -> Undefined {
        self.inner.call("unpauseAnimations", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    pub fn animations_paused(&self) -> bool {
        self.inner.call("animationsPaused", &[]).as_::<bool>()
    }
}
impl SVGSVGElement {
    pub fn get_current_time(&self) -> f32 {
        self.inner.call("getCurrentTime", &[]).as_::<f32>()
    }
}
impl SVGSVGElement {
    pub fn set_current_time(&self, seconds: f32) -> Undefined {
        self.inner
            .call("setCurrentTime", &[seconds.into()])
            .as_::<Undefined>()
    }
}
impl SVGSVGElement {
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }
}
impl SVGSVGElement {
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
impl SVGSVGElement {
    pub fn onportalactivate(&self) -> Any {
        self.inner.get("onportalactivate").as_::<Any>()
    }

    pub fn set_onportalactivate(&mut self, value: Any) {
        self.inner.set("onportalactivate", value);
    }
}
