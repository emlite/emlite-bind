use super::*;

#[derive(Clone, Debug)]
pub struct SVGLinearGradientElement {
    inner: SVGGradientElement,
}
impl FromVal for SVGLinearGradientElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGLinearGradientElement {
            inner: SVGGradientElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SVGLinearGradientElement {
    type Target = SVGGradientElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGLinearGradientElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGLinearGradientElement> for emlite::Val {
    fn from(s: SVGLinearGradientElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGLinearGradientElement {
    pub fn x1(&self) -> SVGAnimatedLength {
        self.inner.get("x1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLinearGradientElement {
    pub fn y1(&self) -> SVGAnimatedLength {
        self.inner.get("y1").as_::<SVGAnimatedLength>()
    }
}
impl SVGLinearGradientElement {
    pub fn x2(&self) -> SVGAnimatedLength {
        self.inner.get("x2").as_::<SVGAnimatedLength>()
    }
}
impl SVGLinearGradientElement {
    pub fn y2(&self) -> SVGAnimatedLength {
        self.inner.get("y2").as_::<SVGAnimatedLength>()
    }
}
