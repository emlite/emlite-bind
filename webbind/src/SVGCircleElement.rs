use super::*;

/// The SVGCircleElement class.
/// [`SVGCircleElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGCircleElement {
    inner: SVGGeometryElement,
}

impl FromVal for SVGCircleElement {
    fn from_val(v: &Any) -> Self {
        SVGCircleElement {
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

impl core::ops::Deref for SVGCircleElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGCircleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGCircleElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGCircleElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGCircleElement> for Any {
    fn from(s: SVGCircleElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGCircleElement> for Any {
    fn from(s: &SVGCircleElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGCircleElement);

impl SVGCircleElement {
    /// Getter of the `cx` attribute.
    /// [`SVGCircleElement.cx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cx)
    pub fn cx(&self) -> SVGAnimatedLength {
        self.inner.get("cx").as_::<SVGAnimatedLength>()
    }
}
impl SVGCircleElement {
    /// Getter of the `cy` attribute.
    /// [`SVGCircleElement.cy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cy)
    pub fn cy(&self) -> SVGAnimatedLength {
        self.inner.get("cy").as_::<SVGAnimatedLength>()
    }
}
impl SVGCircleElement {
    /// Getter of the `r` attribute.
    /// [`SVGCircleElement.r`](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/r)
    pub fn r(&self) -> SVGAnimatedLength {
        self.inner.get("r").as_::<SVGAnimatedLength>()
    }
}
