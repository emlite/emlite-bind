use super::*;

/// The SVGFECompositeElement class.
/// [`SVGFECompositeElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFECompositeElement {
    inner: SVGElement,
}
impl FromVal for SVGFECompositeElement {
    fn from_val(v: &Any) -> Self {
        SVGFECompositeElement {
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
impl core::ops::Deref for SVGFECompositeElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFECompositeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFECompositeElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFECompositeElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFECompositeElement> for Any {
    fn from(s: SVGFECompositeElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFECompositeElement> for Any {
    fn from(s: &SVGFECompositeElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFECompositeElement);

impl SVGFECompositeElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFECompositeElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `in2` attribute.
    /// [`SVGFECompositeElement.in2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/in2)
    pub fn in2(&self) -> SVGAnimatedString {
        self.inner.get("in2").as_::<SVGAnimatedString>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `operator` attribute.
    /// [`SVGFECompositeElement.operator`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/operator)
    pub fn operator(&self) -> SVGAnimatedEnumeration {
        self.inner.get("operator").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `k1` attribute.
    /// [`SVGFECompositeElement.k1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k1)
    pub fn k1(&self) -> SVGAnimatedNumber {
        self.inner.get("k1").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `k2` attribute.
    /// [`SVGFECompositeElement.k2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k2)
    pub fn k2(&self) -> SVGAnimatedNumber {
        self.inner.get("k2").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `k3` attribute.
    /// [`SVGFECompositeElement.k3`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k3)
    pub fn k3(&self) -> SVGAnimatedNumber {
        self.inner.get("k3").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `k4` attribute.
    /// [`SVGFECompositeElement.k4`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k4)
    pub fn k4(&self) -> SVGAnimatedNumber {
        self.inner.get("k4").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `x` attribute.
    /// [`SVGFECompositeElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `y` attribute.
    /// [`SVGFECompositeElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `width` attribute.
    /// [`SVGFECompositeElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `height` attribute.
    /// [`SVGFECompositeElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    /// Getter of the `result` attribute.
    /// [`SVGFECompositeElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
