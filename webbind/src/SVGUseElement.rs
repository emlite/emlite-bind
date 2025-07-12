use super::*;

#[derive(Clone, Debug)]
pub struct SVGUseElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGUseElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGUseElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SVGUseElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGUseElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGUseElement> for emlite::Val {
    fn from(s: SVGUseElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGUseElement {
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    pub fn instance_root(&self) -> SVGElement {
        self.inner.get("instanceRoot").as_::<SVGElement>()
    }
}
impl SVGUseElement {
    pub fn animated_instance_root(&self) -> SVGElement {
        self.inner.get("animatedInstanceRoot").as_::<SVGElement>()
    }
}
impl SVGUseElement {
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
