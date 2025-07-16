use super::*;

/// The SVGRectElement class.
/// [`SVGRectElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGRectElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGRectElement {
    fn from_val(v: &Any) -> Self {
        SVGRectElement {
            inner: SVGGeometryElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGRectElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGRectElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGRectElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGRectElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGRectElement> for Any {
    fn from(s: SVGRectElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGRectElement> for Any {
    fn from(s: &SVGRectElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGRectElement);

impl SVGRectElement {
    /// Getter of the `x` attribute.
    /// [`SVGRectElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGRectElement {
    /// Getter of the `y` attribute.
    /// [`SVGRectElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGRectElement {
    /// Getter of the `width` attribute.
    /// [`SVGRectElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGRectElement {
    /// Getter of the `height` attribute.
    /// [`SVGRectElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGRectElement {
    /// Getter of the `rx` attribute.
    /// [`SVGRectElement.rx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/rx)
    pub fn rx(&self) -> SVGAnimatedLength {
        self.inner.get("rx").as_::<SVGAnimatedLength>()
    }
}
impl SVGRectElement {
    /// Getter of the `ry` attribute.
    /// [`SVGRectElement.ry`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/ry)
    pub fn ry(&self) -> SVGAnimatedLength {
        self.inner.get("ry").as_::<SVGAnimatedLength>()
    }
}
