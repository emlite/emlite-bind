use super::*;

#[derive(Clone, Debug)]
pub struct SVGFilterElement {
    inner: SVGElement,
}
impl FromVal for SVGFilterElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFilterElement {
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
impl std::ops::Deref for SVGFilterElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGFilterElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFilterElement> for emlite::Val {
    fn from(s: SVGFilterElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGFilterElement {
    pub fn filter_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("filterUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFilterElement {
    pub fn primitive_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("primitiveUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFilterElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
