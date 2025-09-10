use super::*;

/// The SVGFEDiffuseLightingElement class.
/// [`SVGFEDiffuseLightingElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEDiffuseLightingElement {
    inner: SVGElement,
}

impl FromVal for SVGFEDiffuseLightingElement {
    fn from_val(v: &Any) -> Self {
        SVGFEDiffuseLightingElement {
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

impl core::ops::Deref for SVGFEDiffuseLightingElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEDiffuseLightingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEDiffuseLightingElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEDiffuseLightingElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEDiffuseLightingElement> for Any {
    fn from(s: SVGFEDiffuseLightingElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEDiffuseLightingElement> for Any {
    fn from(s: &SVGFEDiffuseLightingElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEDiffuseLightingElement);

impl SVGFEDiffuseLightingElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEDiffuseLightingElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `surfaceScale` attribute.
    /// [`SVGFEDiffuseLightingElement.surfaceScale`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/surfaceScale)
    pub fn surface_scale(&self) -> SVGAnimatedNumber {
        self.inner.get("surfaceScale").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `diffuseConstant` attribute.
    /// [`SVGFEDiffuseLightingElement.diffuseConstant`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/diffuseConstant)
    pub fn diffuse_constant(&self) -> SVGAnimatedNumber {
        self.inner.get("diffuseConstant").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `kernelUnitLengthX` attribute.
    /// [`SVGFEDiffuseLightingElement.kernelUnitLengthX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthX)
    pub fn kernel_unit_length_x(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthX")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `kernelUnitLengthY` attribute.
    /// [`SVGFEDiffuseLightingElement.kernelUnitLengthY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthY)
    pub fn kernel_unit_length_y(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthY")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEDiffuseLightingElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEDiffuseLightingElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEDiffuseLightingElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEDiffuseLightingElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEDiffuseLightingElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
