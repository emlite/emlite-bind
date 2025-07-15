use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEMorphologyElement {
    inner: SVGElement,
}
impl FromVal for SVGFEMorphologyElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEMorphologyElement {
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
impl core::ops::Deref for SVGFEMorphologyElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEMorphologyElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFEMorphologyElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFEMorphologyElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGFEMorphologyElement> for emlite::Val {
    fn from(s: SVGFEMorphologyElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEMorphologyElement);

impl SVGFEMorphologyElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEMorphologyElement {
    pub fn operator(&self) -> SVGAnimatedEnumeration {
        self.inner.get("operator").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEMorphologyElement {
    pub fn radius_x(&self) -> SVGAnimatedNumber {
        self.inner.get("radiusX").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEMorphologyElement {
    pub fn radius_y(&self) -> SVGAnimatedNumber {
        self.inner.get("radiusY").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEMorphologyElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMorphologyElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMorphologyElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMorphologyElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMorphologyElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
