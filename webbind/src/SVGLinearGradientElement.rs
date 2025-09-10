use super::*;

/// The SVGLinearGradientElement class.
/// [`SVGLinearGradientElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGLinearGradientElement {
    inner: SVGGradientElement,
}

impl FromVal for SVGLinearGradientElement {
    fn from_val(v: &Any) -> Self {
        SVGLinearGradientElement {
            inner: SVGGradientElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGLinearGradientElement {
    type Target = SVGGradientElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGLinearGradientElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGLinearGradientElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGLinearGradientElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGLinearGradientElement> for Any {
    fn from(s: SVGLinearGradientElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGLinearGradientElement> for Any {
    fn from(s: &SVGLinearGradientElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGLinearGradientElement);

impl SVGLinearGradientElement {
    /// Getter of the `x1` attribute.
    /// [`SVGLinearGradientElement.x1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x1)
    pub fn x1(&self) -> SVGAnimatedLength {
        self.inner.get("x1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLinearGradientElement {
    /// Getter of the `y1` attribute.
    /// [`SVGLinearGradientElement.y1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y1)
    pub fn y1(&self) -> SVGAnimatedLength {
        self.inner.get("y1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLinearGradientElement {
    /// Getter of the `x2` attribute.
    /// [`SVGLinearGradientElement.x2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x2)
    pub fn x2(&self) -> SVGAnimatedLength {
        self.inner.get("x2").as_::<SVGAnimatedLength>()
    }
}
impl SVGLinearGradientElement {
    /// Getter of the `y2` attribute.
    /// [`SVGLinearGradientElement.y2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y2)
    pub fn y2(&self) -> SVGAnimatedLength {
        self.inner.get("y2").as_::<SVGAnimatedLength>()
    }
}
