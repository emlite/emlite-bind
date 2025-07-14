use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGFEColorMatrixElement {
    inner: SVGElement,
}
impl FromVal for SVGFEColorMatrixElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEColorMatrixElement {
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
impl core::ops::Deref for SVGFEColorMatrixElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEColorMatrixElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFEColorMatrixElement> for emlite::Val {
    fn from(s: SVGFEColorMatrixElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGFEColorMatrixElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEColorMatrixElement {
    pub fn type_(&self) -> SVGAnimatedEnumeration {
        self.inner.get("type").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEColorMatrixElement {
    pub fn values(&self) -> SVGAnimatedNumberList {
        self.inner.get("values").as_::<SVGAnimatedNumberList>()
    }
}
impl SVGFEColorMatrixElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEColorMatrixElement {
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
