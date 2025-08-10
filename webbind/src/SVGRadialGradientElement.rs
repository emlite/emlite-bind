use super::*;

/// The SVGRadialGradientElement class.
/// [`SVGRadialGradientElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGRadialGradientElement {
    inner: SVGGradientElement,
}

impl FromVal for SVGRadialGradientElement {
    fn from_val(v: &Any) -> Self {
        SVGRadialGradientElement {
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

impl core::ops::Deref for SVGRadialGradientElement {
    type Target = SVGGradientElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGRadialGradientElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGRadialGradientElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGRadialGradientElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGRadialGradientElement> for Any {
    fn from(s: SVGRadialGradientElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGRadialGradientElement> for Any {
    fn from(s: &SVGRadialGradientElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGRadialGradientElement);

impl SVGRadialGradientElement {
    /// Getter of the `cx` attribute.
    /// [`SVGRadialGradientElement.cx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cx)
    pub fn cx(&self) -> SVGAnimatedLength {
        self.inner.get("cx").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    /// Getter of the `cy` attribute.
    /// [`SVGRadialGradientElement.cy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cy)
    pub fn cy(&self) -> SVGAnimatedLength {
        self.inner.get("cy").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    /// Getter of the `r` attribute.
    /// [`SVGRadialGradientElement.r`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/r)
    pub fn r(&self) -> SVGAnimatedLength {
        self.inner.get("r").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    /// Getter of the `fx` attribute.
    /// [`SVGRadialGradientElement.fx`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fx)
    pub fn fx(&self) -> SVGAnimatedLength {
        self.inner.get("fx").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    /// Getter of the `fy` attribute.
    /// [`SVGRadialGradientElement.fy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fy)
    pub fn fy(&self) -> SVGAnimatedLength {
        self.inner.get("fy").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    /// Getter of the `fr` attribute.
    /// [`SVGRadialGradientElement.fr`](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fr)
    pub fn fr(&self) -> SVGAnimatedLength {
        self.inner.get("fr").as_::<SVGAnimatedLength>()
    }
}
