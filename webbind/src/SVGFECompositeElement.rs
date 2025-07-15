use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFECompositeElement {
    inner: SVGElement,
}
impl FromVal for SVGFECompositeElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFECompositeElement {
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
impl core::ops::Deref for SVGFECompositeElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFECompositeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFECompositeElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFECompositeElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGFECompositeElement> for emlite::Val {
    fn from(s: SVGFECompositeElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGFECompositeElement> for emlite::Val {
    fn from(s: &SVGFECompositeElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFECompositeElement);

impl SVGFECompositeElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFECompositeElement {
    pub fn in2(&self) -> SVGAnimatedString {
        self.inner.get("in2").as_::<SVGAnimatedString>()
    }
}
impl SVGFECompositeElement {
    pub fn operator(&self) -> SVGAnimatedEnumeration {
        self.inner.get("operator").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFECompositeElement {
    pub fn k1(&self) -> SVGAnimatedNumber {
        self.inner.get("k1").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    pub fn k2(&self) -> SVGAnimatedNumber {
        self.inner.get("k2").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    pub fn k3(&self) -> SVGAnimatedNumber {
        self.inner.get("k3").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    pub fn k4(&self) -> SVGAnimatedNumber {
        self.inner.get("k4").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFECompositeElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFECompositeElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
