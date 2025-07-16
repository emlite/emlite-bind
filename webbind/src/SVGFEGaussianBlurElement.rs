use super::*;

/// The SVGFEGaussianBlurElement class.
/// [`SVGFEGaussianBlurElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEGaussianBlurElement {
    inner: SVGElement,
}
impl FromVal for SVGFEGaussianBlurElement {
    fn from_val(v: &Any) -> Self {
        SVGFEGaussianBlurElement {
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
impl core::ops::Deref for SVGFEGaussianBlurElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEGaussianBlurElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFEGaussianBlurElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFEGaussianBlurElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFEGaussianBlurElement> for Any {
    fn from(s: SVGFEGaussianBlurElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFEGaussianBlurElement> for Any {
    fn from(s: &SVGFEGaussianBlurElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEGaussianBlurElement);

impl SVGFEGaussianBlurElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEGaussianBlurElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `stdDeviationX` attribute.
    /// [`SVGFEGaussianBlurElement.stdDeviationX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationX)
    pub fn std_deviation_x(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationX").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `stdDeviationY` attribute.
    /// [`SVGFEGaussianBlurElement.stdDeviationY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationY)
    pub fn std_deviation_y(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationY").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `edgeMode` attribute.
    /// [`SVGFEGaussianBlurElement.edgeMode`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/edgeMode)
    pub fn edge_mode(&self) -> SVGAnimatedEnumeration {
        self.inner.get("edgeMode").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEGaussianBlurElement {
    /// The setStdDeviation method.
    /// [`SVGFEGaussianBlurElement.setStdDeviation`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/setStdDeviation)
    pub fn set_std_deviation(&self, std_deviation_x: f32, std_deviation_y: f32) -> Undefined {
        self.inner
            .call(
                "setStdDeviation",
                &[std_deviation_x.into(), std_deviation_y.into()],
            )
            .as_::<Undefined>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEGaussianBlurElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEGaussianBlurElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEGaussianBlurElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEGaussianBlurElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEGaussianBlurElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEGaussianBlurElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
