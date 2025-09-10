use super::*;

/// The SVGFESpecularLightingElement class.
/// [`SVGFESpecularLightingElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFESpecularLightingElement {
    inner: SVGElement,
}

impl FromVal for SVGFESpecularLightingElement {
    fn from_val(v: &Any) -> Self {
        SVGFESpecularLightingElement {
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

impl core::ops::Deref for SVGFESpecularLightingElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFESpecularLightingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFESpecularLightingElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFESpecularLightingElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFESpecularLightingElement> for Any {
    fn from(s: SVGFESpecularLightingElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFESpecularLightingElement> for Any {
    fn from(s: &SVGFESpecularLightingElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFESpecularLightingElement);

impl SVGFESpecularLightingElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFESpecularLightingElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `surfaceScale` attribute.
    /// [`SVGFESpecularLightingElement.surfaceScale`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/surfaceScale)
    pub fn surface_scale(&self) -> SVGAnimatedNumber {
        self.inner.get("surfaceScale").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `specularConstant` attribute.
    /// [`SVGFESpecularLightingElement.specularConstant`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularConstant)
    pub fn specular_constant(&self) -> SVGAnimatedNumber {
        self.inner
            .get("specularConstant")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `specularExponent` attribute.
    /// [`SVGFESpecularLightingElement.specularExponent`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularExponent)
    pub fn specular_exponent(&self) -> SVGAnimatedNumber {
        self.inner
            .get("specularExponent")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `kernelUnitLengthX` attribute.
    /// [`SVGFESpecularLightingElement.kernelUnitLengthX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthX)
    pub fn kernel_unit_length_x(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthX")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `kernelUnitLengthY` attribute.
    /// [`SVGFESpecularLightingElement.kernelUnitLengthY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthY)
    pub fn kernel_unit_length_y(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthY")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `x` attribute.
    /// [`SVGFESpecularLightingElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `y` attribute.
    /// [`SVGFESpecularLightingElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `width` attribute.
    /// [`SVGFESpecularLightingElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `height` attribute.
    /// [`SVGFESpecularLightingElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    /// Getter of the `result` attribute.
    /// [`SVGFESpecularLightingElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
