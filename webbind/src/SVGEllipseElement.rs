use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGEllipseElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGEllipseElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGEllipseElement {
            inner: SVGGeometryElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGEllipseElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGEllipseElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGEllipseElement> for emlite::Val {
    fn from(s: SVGEllipseElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGEllipseElement {
    pub fn cx(&self) -> SVGAnimatedLength {
        self.inner.get("cx").as_::<SVGAnimatedLength>()
    }
}
impl SVGEllipseElement {
    pub fn cy(&self) -> SVGAnimatedLength {
        self.inner.get("cy").as_::<SVGAnimatedLength>()
    }
}
impl SVGEllipseElement {
    pub fn rx(&self) -> SVGAnimatedLength {
        self.inner.get("rx").as_::<SVGAnimatedLength>()
    }
}
impl SVGEllipseElement {
    pub fn ry(&self) -> SVGAnimatedLength {
        self.inner.get("ry").as_::<SVGAnimatedLength>()
    }
}
