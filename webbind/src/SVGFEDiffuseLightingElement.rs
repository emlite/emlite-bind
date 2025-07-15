use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEDiffuseLightingElement {
    inner: SVGElement,
}
impl FromVal for SVGFEDiffuseLightingElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEDiffuseLightingElement {
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
impl core::ops::Deref for SVGFEDiffuseLightingElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEDiffuseLightingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFEDiffuseLightingElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFEDiffuseLightingElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGFEDiffuseLightingElement> for emlite::Val {
    fn from(s: SVGFEDiffuseLightingElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEDiffuseLightingElement);

impl SVGFEDiffuseLightingElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn surface_scale(&self) -> SVGAnimatedNumber {
        self.inner.get("surfaceScale").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn diffuse_constant(&self) -> SVGAnimatedNumber {
        self.inner.get("diffuseConstant").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn kernel_unit_length_x(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthX")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn kernel_unit_length_y(&self) -> SVGAnimatedNumber {
        self.inner
            .get("kernelUnitLengthY")
            .as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDiffuseLightingElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
