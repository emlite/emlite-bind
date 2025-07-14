use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGFESpecularLightingElement {
    inner: SVGElement,
}
impl FromVal for SVGFESpecularLightingElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFESpecularLightingElement {
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
impl core::ops::Deref for SVGFESpecularLightingElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFESpecularLightingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFESpecularLightingElement> for emlite::Val {
    fn from(s: SVGFESpecularLightingElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGFESpecularLightingElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn surface_scale(&self) -> SVGAnimatedNumber {
        self.inner.get("surfaceScale").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn specular_constant(&self) -> SVGAnimatedNumber {
        self.inner
            .get("specularConstant")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn specular_exponent(&self) -> SVGAnimatedNumber {
        self.inner
            .get("specularExponent")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn kernel_unit_length_x(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthX")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn kernel_unit_length_y(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthY")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFESpecularLightingElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
