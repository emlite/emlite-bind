use super::*;

/// The SVGFEOffsetElement class.
/// [`SVGFEOffsetElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEOffsetElement {
    inner: SVGElement,
}
impl FromVal for SVGFEOffsetElement {
    fn from_val(v: &Any) -> Self {
        SVGFEOffsetElement {
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
impl core::ops::Deref for SVGFEOffsetElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEOffsetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFEOffsetElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFEOffsetElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFEOffsetElement> for Any {
    fn from(s: SVGFEOffsetElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFEOffsetElement> for Any {
    fn from(s: &SVGFEOffsetElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEOffsetElement);

impl SVGFEOffsetElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEOffsetElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEOffsetElement {
    /// Getter of the `dx` attribute.
    /// [`SVGFEOffsetElement.dx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dx)
    pub fn dx(&self) -> SVGAnimatedNumber {
        self.inner.get("dx").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEOffsetElement {
    /// Getter of the `dy` attribute.
    /// [`SVGFEOffsetElement.dy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dy)
    pub fn dy(&self) -> SVGAnimatedNumber {
        self.inner.get("dy").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEOffsetElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEOffsetElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEOffsetElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEOffsetElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEOffsetElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEOffsetElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEOffsetElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEOffsetElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEOffsetElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEOffsetElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
