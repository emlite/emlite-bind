use super::*;

/// The SVGFEColorMatrixElement class.
/// [`SVGFEColorMatrixElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEColorMatrixElement {
    inner: SVGElement,
}

impl FromVal for SVGFEColorMatrixElement {
    fn from_val(v: &Any) -> Self {
        SVGFEColorMatrixElement {
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

impl core::ops::Deref for SVGFEColorMatrixElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEColorMatrixElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEColorMatrixElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEColorMatrixElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEColorMatrixElement> for Any {
    fn from(s: SVGFEColorMatrixElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEColorMatrixElement> for Any {
    fn from(s: &SVGFEColorMatrixElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEColorMatrixElement);

impl SVGFEColorMatrixElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEColorMatrixElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEColorMatrixElement {
    /// Getter of the `type` attribute.
    /// [`SVGFEColorMatrixElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/type)
    pub fn type_(&self) -> SVGAnimatedEnumeration {
        self.inner.get("type").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEColorMatrixElement {
    /// Getter of the `values` attribute.
    /// [`SVGFEColorMatrixElement.values`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/values)
    pub fn values(&self) -> SVGAnimatedNumberList {
        self.inner.get("values").as_::<SVGAnimatedNumberList>()
    }
}
impl SVGFEColorMatrixElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEColorMatrixElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEColorMatrixElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEColorMatrixElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEColorMatrixElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEColorMatrixElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
