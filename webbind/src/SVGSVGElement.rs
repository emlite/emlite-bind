use super::*;

/// The SVGSVGElement class.
/// [`SVGSVGElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGSVGElement {
    inner: SVGGraphicsElement,
}

impl FromVal for SVGSVGElement {
    fn from_val(v: &Any) -> Self {
        SVGSVGElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SVGSVGElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGSVGElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGSVGElement> for Any {
    fn from(s: SVGSVGElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGSVGElement> for Any {
    fn from(s: &SVGSVGElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGSVGElement);

impl SVGSVGElement {
    /// Getter of the `x` attribute.
    /// [`SVGSVGElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    /// Getter of the `y` attribute.
    /// [`SVGSVGElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    /// Getter of the `width` attribute.
    /// [`SVGSVGElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    /// Getter of the `height` attribute.
    /// [`SVGSVGElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGSVGElement {
    /// Getter of the `currentScale` attribute.
    /// [`SVGSVGElement.currentScale`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)
    pub fn current_scale(&self) -> f32 {
        self.inner.get("currentScale").as_::<f32>()
    }

    /// Setter of the `currentScale` attribute.
    /// [`SVGSVGElement.currentScale`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)
    pub fn set_current_scale(&mut self, value: f32) {
        self.inner.set("currentScale", value);
    }
}
impl SVGSVGElement {
    /// Getter of the `currentTranslate` attribute.
    /// [`SVGSVGElement.currentTranslate`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentTranslate)
    pub fn current_translate(&self) -> DOMPointReadOnly {
        self.inner.get("currentTranslate").as_::<DOMPointReadOnly>()
    }
}
impl SVGSVGElement {
    /// Getter of the `viewBox` attribute.
    /// [`SVGSVGElement.viewBox`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/viewBox)
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }
}
impl SVGSVGElement {
    /// Getter of the `preserveAspectRatio` attribute.
    /// [`SVGSVGElement.preserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/preserveAspectRatio)
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
impl SVGSVGElement {
    /// Getter of the `onportalactivate` attribute.
    /// [`SVGSVGElement.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/onportalactivate)
    pub fn onportalactivate(&self) -> Any {
        self.inner.get("onportalactivate").as_::<Any>()
    }

    /// Setter of the `onportalactivate` attribute.
    /// [`SVGSVGElement.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/onportalactivate)
    pub fn set_onportalactivate(&mut self, value: &Any) {
        self.inner.set("onportalactivate", value);
    }
}
impl SVGSVGElement {
    /// The getIntersectionList method.
    /// [`SVGSVGElement.getIntersectionList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getIntersectionList)
    pub fn get_intersection_list(
        &self,
        rect: &DOMRectReadOnly,
        reference_element: &SVGElement,
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
    /// The getEnclosureList method.
    /// [`SVGSVGElement.getEnclosureList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getEnclosureList)
    pub fn get_enclosure_list(
        &self,
        rect: &DOMRectReadOnly,
        reference_element: &SVGElement,
    ) -> NodeList {
        self.inner
            .call("getEnclosureList", &[rect.into(), reference_element.into()])
            .as_::<NodeList>()
    }
}
impl SVGSVGElement {
    /// The checkIntersection method.
    /// [`SVGSVGElement.checkIntersection`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/checkIntersection)
    pub fn check_intersection(&self, element: &SVGElement, rect: &DOMRectReadOnly) -> bool {
        self.inner
            .call("checkIntersection", &[element.into(), rect.into()])
            .as_::<bool>()
    }
}
impl SVGSVGElement {
    /// The checkEnclosure method.
    /// [`SVGSVGElement.checkEnclosure`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/checkEnclosure)
    pub fn check_enclosure(&self, element: &SVGElement, rect: &DOMRectReadOnly) -> bool {
        self.inner
            .call("checkEnclosure", &[element.into(), rect.into()])
            .as_::<bool>()
    }
}
impl SVGSVGElement {
    /// The deselectAll method.
    /// [`SVGSVGElement.deselectAll`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/deselectAll)
    pub fn deselect_all(&self) -> Undefined {
        self.inner.call("deselectAll", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    /// The createSVGNumber method.
    /// [`SVGSVGElement.createSVGNumber`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGNumber)
    pub fn create_svg_number(&self) -> SVGNumber {
        self.inner.call("createSVGNumber", &[]).as_::<SVGNumber>()
    }
}
impl SVGSVGElement {
    /// The createSVGLength method.
    /// [`SVGSVGElement.createSVGLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGLength)
    pub fn create_svg_length(&self) -> SVGLength {
        self.inner.call("createSVGLength", &[]).as_::<SVGLength>()
    }
}
impl SVGSVGElement {
    /// The createSVGAngle method.
    /// [`SVGSVGElement.createSVGAngle`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGAngle)
    pub fn create_svg_angle(&self) -> SVGAngle {
        self.inner.call("createSVGAngle", &[]).as_::<SVGAngle>()
    }
}
impl SVGSVGElement {
    /// The createSVGPoint method.
    /// [`SVGSVGElement.createSVGPoint`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGPoint)
    pub fn create_svg_point(&self) -> DOMPoint {
        self.inner.call("createSVGPoint", &[]).as_::<DOMPoint>()
    }
}
impl SVGSVGElement {
    /// The createSVGMatrix method.
    /// [`SVGSVGElement.createSVGMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGMatrix)
    pub fn create_svg_matrix(&self) -> DOMMatrix {
        self.inner.call("createSVGMatrix", &[]).as_::<DOMMatrix>()
    }
}
impl SVGSVGElement {
    /// The createSVGRect method.
    /// [`SVGSVGElement.createSVGRect`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGRect)
    pub fn create_svg_rect(&self) -> DOMRect {
        self.inner.call("createSVGRect", &[]).as_::<DOMRect>()
    }
}
impl SVGSVGElement {
    /// The createSVGTransform method.
    /// [`SVGSVGElement.createSVGTransform`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransform)
    pub fn create_svg_transform(&self) -> SVGTransform {
        self.inner
            .call("createSVGTransform", &[])
            .as_::<SVGTransform>()
    }
}
impl SVGSVGElement {
    /// The createSVGTransformFromMatrix method.
    /// [`SVGSVGElement.createSVGTransformFromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransformFromMatrix)
    pub fn create_svg_transform_from_matrix(&self) -> SVGTransform {
        self.inner
            .call("createSVGTransformFromMatrix", &[])
            .as_::<SVGTransform>()
    }
}
impl SVGSVGElement {
    /// The createSVGTransformFromMatrix method.
    /// [`SVGSVGElement.createSVGTransformFromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransformFromMatrix)
    pub fn create_svg_transform_from_matrix_with_matrix(
        &self,
        matrix: &DOMMatrix2DInit,
    ) -> SVGTransform {
        self.inner
            .call("createSVGTransformFromMatrix", &[matrix.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGSVGElement {
    /// The getElementById method.
    /// [`SVGSVGElement.getElementById`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getElementById)
    pub fn get_element_by_id(&self, element_id: &JsString) -> Element {
        self.inner
            .call("getElementById", &[element_id.into()])
            .as_::<Element>()
    }
}
impl SVGSVGElement {
    /// The suspendRedraw method.
    /// [`SVGSVGElement.suspendRedraw`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/suspendRedraw)
    pub fn suspend_redraw(&self, max_wait_milliseconds: u32) -> u32 {
        self.inner
            .call("suspendRedraw", &[max_wait_milliseconds.into()])
            .as_::<u32>()
    }
}
impl SVGSVGElement {
    /// The unsuspendRedraw method.
    /// [`SVGSVGElement.unsuspendRedraw`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedraw)
    pub fn unsuspend_redraw(&self, suspend_handle_id: u32) -> Undefined {
        self.inner
            .call("unsuspendRedraw", &[suspend_handle_id.into()])
            .as_::<Undefined>()
    }
}
impl SVGSVGElement {
    /// The unsuspendRedrawAll method.
    /// [`SVGSVGElement.unsuspendRedrawAll`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedrawAll)
    pub fn unsuspend_redraw_all(&self) -> Undefined {
        self.inner
            .call("unsuspendRedrawAll", &[])
            .as_::<Undefined>()
    }
}
impl SVGSVGElement {
    /// The forceRedraw method.
    /// [`SVGSVGElement.forceRedraw`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/forceRedraw)
    pub fn force_redraw(&self) -> Undefined {
        self.inner.call("forceRedraw", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    /// The pauseAnimations method.
    /// [`SVGSVGElement.pauseAnimations`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/pauseAnimations)
    pub fn pause_animations(&self) -> Undefined {
        self.inner.call("pauseAnimations", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    /// The unpauseAnimations method.
    /// [`SVGSVGElement.unpauseAnimations`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unpauseAnimations)
    pub fn unpause_animations(&self) -> Undefined {
        self.inner.call("unpauseAnimations", &[]).as_::<Undefined>()
    }
}
impl SVGSVGElement {
    /// The animationsPaused method.
    /// [`SVGSVGElement.animationsPaused`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/animationsPaused)
    pub fn animations_paused(&self) -> bool {
        self.inner.call("animationsPaused", &[]).as_::<bool>()
    }
}
impl SVGSVGElement {
    /// The getCurrentTime method.
    /// [`SVGSVGElement.getCurrentTime`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getCurrentTime)
    pub fn get_current_time(&self) -> f32 {
        self.inner.call("getCurrentTime", &[]).as_::<f32>()
    }
}
impl SVGSVGElement {
    /// The setCurrentTime method.
    /// [`SVGSVGElement.setCurrentTime`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/setCurrentTime)
    pub fn set_current_time(&self, seconds: f32) -> Undefined {
        self.inner
            .call("setCurrentTime", &[seconds.into()])
            .as_::<Undefined>()
    }
}
