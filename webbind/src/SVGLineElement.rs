use super::*;

/// The SVGLineElement class.
/// [`SVGLineElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGLineElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGLineElement {
    fn from_val(v: &Any) -> Self {
        SVGLineElement {
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
impl core::ops::Deref for SVGLineElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGLineElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGLineElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGLineElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGLineElement> for Any {
    fn from(s: SVGLineElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGLineElement> for Any {
    fn from(s: &SVGLineElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGLineElement);

impl SVGLineElement {
    /// Getter of the `x1` attribute.
    /// [`SVGLineElement.x1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/x1)
    pub fn x1(&self) -> SVGAnimatedLength {
        self.inner.get("x1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    /// Getter of the `y1` attribute.
    /// [`SVGLineElement.y1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/y1)
    pub fn y1(&self) -> SVGAnimatedLength {
        self.inner.get("y1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    /// Getter of the `x2` attribute.
    /// [`SVGLineElement.x2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/x2)
    pub fn x2(&self) -> SVGAnimatedLength {
        self.inner.get("x2").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    /// Getter of the `y2` attribute.
    /// [`SVGLineElement.y2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/y2)
    pub fn y2(&self) -> SVGAnimatedLength {
        self.inner.get("y2").as_::<SVGAnimatedLength>()
    }
}
