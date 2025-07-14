use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGRadialGradientElement {
    inner: SVGGradientElement,
}
impl FromVal for SVGRadialGradientElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGRadialGradientElement {
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
impl core::ops::Deref for SVGRadialGradientElement {
    type Target = SVGGradientElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGRadialGradientElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGRadialGradientElement> for emlite::Val {
    fn from(s: SVGRadialGradientElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGRadialGradientElement {
    pub fn cx(&self) -> SVGAnimatedLength {
        self.inner.get("cx").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    pub fn cy(&self) -> SVGAnimatedLength {
        self.inner.get("cy").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    pub fn r(&self) -> SVGAnimatedLength {
        self.inner.get("r").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    pub fn fx(&self) -> SVGAnimatedLength {
        self.inner.get("fx").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    pub fn fy(&self) -> SVGAnimatedLength {
        self.inner.get("fy").as_::<SVGAnimatedLength>()
    }
}
impl SVGRadialGradientElement {
    pub fn fr(&self) -> SVGAnimatedLength {
        self.inner.get("fr").as_::<SVGAnimatedLength>()
    }
}
