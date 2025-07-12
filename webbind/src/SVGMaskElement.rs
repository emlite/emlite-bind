use super::*;

#[derive(Clone, Debug)]
pub struct SVGMaskElement {
    inner: SVGElement,
}
impl FromVal for SVGMaskElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGMaskElement {
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
impl std::ops::Deref for SVGMaskElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGMaskElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGMaskElement> for emlite::Val {
    fn from(s: SVGMaskElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGMaskElement {
    pub fn mask_units(&self) -> SVGAnimatedEnumeration {
        self.inner.get("maskUnits").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGMaskElement {
    pub fn mask_content_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("maskContentUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGMaskElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGMaskElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGMaskElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGMaskElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
