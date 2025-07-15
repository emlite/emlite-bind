use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFESpotLightElement {
    inner: SVGElement,
}
impl FromVal for SVGFESpotLightElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFESpotLightElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for SVGFESpotLightElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFESpotLightElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGFESpotLightElement> for emlite::Val {
    fn from(s: SVGFESpotLightElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFESpotLightElement);

impl SVGFESpotLightElement {
    pub fn x(&self) -> SVGAnimatedNumber {
        self.inner.get("x").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    pub fn y(&self) -> SVGAnimatedNumber {
        self.inner.get("y").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    pub fn z(&self) -> SVGAnimatedNumber {
        self.inner.get("z").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    pub fn points_at_x(&self) -> SVGAnimatedNumber {
        self.inner.get("pointsAtX").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    pub fn points_at_y(&self) -> SVGAnimatedNumber {
        self.inner.get("pointsAtY").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    pub fn points_at_z(&self) -> SVGAnimatedNumber {
        self.inner.get("pointsAtZ").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    pub fn specular_exponent(&self) -> SVGAnimatedNumber {
        self.inner
            .get("specularExponent")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpotLightElement {
    pub fn limiting_cone_angle(&self) -> SVGAnimatedNumber {
        self.inner
            .get("limitingConeAngle")
            .as_::<SVGAnimatedNumber>()
    }
}
