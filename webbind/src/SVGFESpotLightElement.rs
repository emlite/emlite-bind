use super::*;

/// The SVGFESpotLightElement class.
/// [`SVGFESpotLightElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFESpotLightElement {
    inner: SVGElement,
}
impl FromVal for SVGFESpotLightElement {
    fn from_val(v: &Any) -> Self {
        SVGFESpotLightElement {
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
impl core::ops::Deref for SVGFESpotLightElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFESpotLightElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFESpotLightElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFESpotLightElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFESpotLightElement> for Any {
    fn from(s: SVGFESpotLightElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFESpotLightElement> for Any {
    fn from(s: &SVGFESpotLightElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFESpotLightElement);

impl SVGFESpotLightElement {
    /// Getter of the `x` attribute.
    /// [`SVGFESpotLightElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/x)
    pub fn x(&self) -> SVGAnimatedNumber {
        self.inner.get("x").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    /// Getter of the `y` attribute.
    /// [`SVGFESpotLightElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/y)
    pub fn y(&self) -> SVGAnimatedNumber {
        self.inner.get("y").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    /// Getter of the `z` attribute.
    /// [`SVGFESpotLightElement.z`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/z)
    pub fn z(&self) -> SVGAnimatedNumber {
        self.inner.get("z").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    /// Getter of the `pointsAtX` attribute.
    /// [`SVGFESpotLightElement.pointsAtX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtX)
    pub fn points_at_x(&self) -> SVGAnimatedNumber {
        self.inner.get("pointsAtX").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    /// Getter of the `pointsAtY` attribute.
    /// [`SVGFESpotLightElement.pointsAtY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtY)
    pub fn points_at_y(&self) -> SVGAnimatedNumber {
        self.inner.get("pointsAtY").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    /// Getter of the `pointsAtZ` attribute.
    /// [`SVGFESpotLightElement.pointsAtZ`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtZ)
    pub fn points_at_z(&self) -> SVGAnimatedNumber {
        self.inner.get("pointsAtZ").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    /// Getter of the `specularExponent` attribute.
    /// [`SVGFESpotLightElement.specularExponent`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/specularExponent)
    pub fn specular_exponent(&self) -> SVGAnimatedNumber {
        self.inner
            .get("specularExponent")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    /// Getter of the `limitingConeAngle` attribute.
    /// [`SVGFESpotLightElement.limitingConeAngle`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/limitingConeAngle)
    pub fn limiting_cone_angle(&self) -> SVGAnimatedNumber {
        self.inner
            .get("limitingConeAngle")
            .as_::<SVGAnimatedNumber>()
    }
}
