use super::*;

/// The SVGEllipseElement class.
/// [`SVGEllipseElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGEllipseElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGEllipseElement {
    fn from_val(v: &Any) -> Self {
        SVGEllipseElement {
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
impl core::ops::Deref for SVGEllipseElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGEllipseElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGEllipseElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGEllipseElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGEllipseElement> for Any {
    fn from(s: SVGEllipseElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGEllipseElement> for Any {
    fn from(s: &SVGEllipseElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGEllipseElement);

impl SVGEllipseElement {
    /// Getter of the `cx` attribute.
    /// [`SVGEllipseElement.cx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cx)
    pub fn cx(&self) -> SVGAnimatedLength {
        self.inner.get("cx").as_::<SVGAnimatedLength>()
    }
}
impl SVGEllipseElement {
    /// Getter of the `cy` attribute.
    /// [`SVGEllipseElement.cy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cy)
    pub fn cy(&self) -> SVGAnimatedLength {
        self.inner.get("cy").as_::<SVGAnimatedLength>()
    }
}
impl SVGEllipseElement {
    /// Getter of the `rx` attribute.
    /// [`SVGEllipseElement.rx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/rx)
    pub fn rx(&self) -> SVGAnimatedLength {
        self.inner.get("rx").as_::<SVGAnimatedLength>()
    }
}
impl SVGEllipseElement {
    /// Getter of the `ry` attribute.
    /// [`SVGEllipseElement.ry`](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/ry)
    pub fn ry(&self) -> SVGAnimatedLength {
        self.inner.get("ry").as_::<SVGAnimatedLength>()
    }
}
