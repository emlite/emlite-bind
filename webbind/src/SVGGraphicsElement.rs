use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGBoundingBoxOptions {
    inner: Any,
}
impl FromVal for SVGBoundingBoxOptions {
    fn from_val(v: &Any) -> Self {
        SVGBoundingBoxOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGBoundingBoxOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGBoundingBoxOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGBoundingBoxOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGBoundingBoxOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGBoundingBoxOptions> for Any {
    fn from(s: SVGBoundingBoxOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGBoundingBoxOptions> for Any {
    fn from(s: &SVGBoundingBoxOptions) -> Any {
        s.inner.clone()
    }
}

impl SVGBoundingBoxOptions {
    pub fn fill(&self) -> bool {
        self.inner.get("fill").as_::<bool>()
    }

    pub fn set_fill(&mut self, value: bool) {
        self.inner.set("fill", value);
    }
}
impl SVGBoundingBoxOptions {
    pub fn stroke(&self) -> bool {
        self.inner.get("stroke").as_::<bool>()
    }

    pub fn set_stroke(&mut self, value: bool) {
        self.inner.set("stroke", value);
    }
}
impl SVGBoundingBoxOptions {
    pub fn markers(&self) -> bool {
        self.inner.get("markers").as_::<bool>()
    }

    pub fn set_markers(&mut self, value: bool) {
        self.inner.set("markers", value);
    }
}
impl SVGBoundingBoxOptions {
    pub fn clipped(&self) -> bool {
        self.inner.get("clipped").as_::<bool>()
    }

    pub fn set_clipped(&mut self, value: bool) {
        self.inner.set("clipped", value);
    }
}
/// The SVGGraphicsElement class.
/// [`SVGGraphicsElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGGraphicsElement {
    inner: SVGElement,
}
impl FromVal for SVGGraphicsElement {
    fn from_val(v: &Any) -> Self {
        SVGGraphicsElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGGraphicsElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGGraphicsElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGGraphicsElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGGraphicsElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGGraphicsElement> for Any {
    fn from(s: SVGGraphicsElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGGraphicsElement> for Any {
    fn from(s: &SVGGraphicsElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGGraphicsElement);

impl SVGGraphicsElement {
    /// Getter of the `transform` attribute.
    /// [`SVGGraphicsElement.transform`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/transform)
    pub fn transform(&self) -> SVGAnimatedTransformList {
        self.inner
            .get("transform")
            .as_::<SVGAnimatedTransformList>()
    }
}
impl SVGGraphicsElement {
    /// The getBBox method.
    /// [`SVGGraphicsElement.getBBox`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)
    pub fn get_b_box0(&self) -> DOMRect {
        self.inner.call("getBBox", &[]).as_::<DOMRect>()
    }
    /// The getBBox method.
    /// [`SVGGraphicsElement.getBBox`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)
    pub fn get_b_box1(&self, options: &SVGBoundingBoxOptions) -> DOMRect {
        self.inner
            .call("getBBox", &[options.into()])
            .as_::<DOMRect>()
    }
}
impl SVGGraphicsElement {
    /// The getCTM method.
    /// [`SVGGraphicsElement.getCTM`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getCTM)
    pub fn get_ctm(&self) -> DOMMatrix {
        self.inner.call("getCTM", &[]).as_::<DOMMatrix>()
    }
}
impl SVGGraphicsElement {
    /// The getScreenCTM method.
    /// [`SVGGraphicsElement.getScreenCTM`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getScreenCTM)
    pub fn get_screen_ctm(&self) -> DOMMatrix {
        self.inner.call("getScreenCTM", &[]).as_::<DOMMatrix>()
    }
}
impl SVGGraphicsElement {
    /// Getter of the `requiredExtensions` attribute.
    /// [`SVGGraphicsElement.requiredExtensions`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredExtensions)
    pub fn required_extensions(&self) -> SVGStringList {
        self.inner.get("requiredExtensions").as_::<SVGStringList>()
    }
}
impl SVGGraphicsElement {
    /// Getter of the `systemLanguage` attribute.
    /// [`SVGGraphicsElement.systemLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/systemLanguage)
    pub fn system_language(&self) -> SVGStringList {
        self.inner.get("systemLanguage").as_::<SVGStringList>()
    }
}
