use super::*;

#[derive(Clone, Debug)]
pub struct SVGCircleElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGCircleElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGCircleElement {
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
impl std::ops::Deref for SVGCircleElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGCircleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGCircleElement> for emlite::Val {
    fn from(s: SVGCircleElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGCircleElement {
    pub fn cx(&self) -> SVGAnimatedLength {
        self.inner.get("cx").as_::<SVGAnimatedLength>()
    }
}
impl SVGCircleElement {
    pub fn cy(&self) -> SVGAnimatedLength {
        self.inner.get("cy").as_::<SVGAnimatedLength>()
    }
}
impl SVGCircleElement {
    pub fn r(&self) -> SVGAnimatedLength {
        self.inner.get("r").as_::<SVGAnimatedLength>()
    }
}
