use super::*;

/// The SVGFEPointLightElement class.
/// [`SVGFEPointLightElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEPointLightElement {
    inner: SVGElement,
}
impl FromVal for SVGFEPointLightElement {
    fn from_val(v: &Any) -> Self {
        SVGFEPointLightElement {
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
impl core::ops::Deref for SVGFEPointLightElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEPointLightElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFEPointLightElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFEPointLightElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFEPointLightElement> for Any {
    fn from(s: SVGFEPointLightElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFEPointLightElement> for Any {
    fn from(s: &SVGFEPointLightElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEPointLightElement);

impl SVGFEPointLightElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEPointLightElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/x)
    pub fn x(&self) -> SVGAnimatedNumber {
        self.inner.get("x").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEPointLightElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEPointLightElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/y)
    pub fn y(&self) -> SVGAnimatedNumber {
        self.inner.get("y").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEPointLightElement {
    /// Getter of the `z` attribute.
    /// [`SVGFEPointLightElement.z`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/z)
    pub fn z(&self) -> SVGAnimatedNumber {
        self.inner.get("z").as_::<SVGAnimatedNumber>()
    }
}
