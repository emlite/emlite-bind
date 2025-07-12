use super::*;

#[derive(Clone, Debug)]
pub struct SVGLineElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGLineElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGLineElement {
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
impl std::ops::Deref for SVGLineElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGLineElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGLineElement> for emlite::Val {
    fn from(s: SVGLineElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGLineElement {
    pub fn x1(&self) -> SVGAnimatedLength {
        self.inner.get("x1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    pub fn y1(&self) -> SVGAnimatedLength {
        self.inner.get("y1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    pub fn x2(&self) -> SVGAnimatedLength {
        self.inner.get("x2").as_::<SVGAnimatedLength>()
    }
}
impl SVGLineElement {
    pub fn y2(&self) -> SVGAnimatedLength {
        self.inner.get("y2").as_::<SVGAnimatedLength>()
    }
}
