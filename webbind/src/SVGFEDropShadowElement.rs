use super::*;

/// The SVGFEDropShadowElement class.
/// [`SVGFEDropShadowElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEDropShadowElement {
    inner: SVGElement,
}

impl FromVal for SVGFEDropShadowElement {
    fn from_val(v: &Any) -> Self {
        SVGFEDropShadowElement {
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

impl core::ops::Deref for SVGFEDropShadowElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEDropShadowElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEDropShadowElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEDropShadowElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEDropShadowElement> for Any {
    fn from(s: SVGFEDropShadowElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEDropShadowElement> for Any {
    fn from(s: &SVGFEDropShadowElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEDropShadowElement);

impl SVGFEDropShadowElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEDropShadowElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `dx` attribute.
    /// [`SVGFEDropShadowElement.dx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dx)
    pub fn dx(&self) -> SVGAnimatedNumber {
        self.inner.get("dx").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `dy` attribute.
    /// [`SVGFEDropShadowElement.dy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dy)
    pub fn dy(&self) -> SVGAnimatedNumber {
        self.inner.get("dy").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `stdDeviationX` attribute.
    /// [`SVGFEDropShadowElement.stdDeviationX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationX)
    pub fn std_deviation_x(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationX").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `stdDeviationY` attribute.
    /// [`SVGFEDropShadowElement.stdDeviationY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationY)
    pub fn std_deviation_y(&self) -> SVGAnimatedNumber {
        self.inner.get("stdDeviationY").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEDropShadowElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEDropShadowElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEDropShadowElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEDropShadowElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDropShadowElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEDropShadowElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDropShadowElement {
    /// The setStdDeviation method.
    /// [`SVGFEDropShadowElement.setStdDeviation`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/setStdDeviation)
    pub fn set_std_deviation(&self, std_deviation_x: f32, std_deviation_y: f32) -> Undefined {
        self.inner
            .call(
                "setStdDeviation",
                &[std_deviation_x.into(), std_deviation_y.into()],
            )
            .as_::<Undefined>()
    }
}
