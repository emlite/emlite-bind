use super::*;

#[derive(Clone, Debug)]
pub struct SVGTextPathElement {
    inner: SVGTextContentElement,
}
impl FromVal for SVGTextPathElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGTextPathElement {
            inner: SVGTextContentElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SVGTextPathElement {
    type Target = SVGTextContentElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGTextPathElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGTextPathElement> for emlite::Val {
    fn from(s: SVGTextPathElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGTextPathElement {
    pub fn start_offset(&self) -> SVGAnimatedLength {
        self.inner.get("startOffset").as_::<SVGAnimatedLength>()
    }
}
impl SVGTextPathElement {
    pub fn method(&self) -> SVGAnimatedEnumeration {
        self.inner.get("method").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGTextPathElement {
    pub fn spacing(&self) -> SVGAnimatedEnumeration {
        self.inner.get("spacing").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGTextPathElement {
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
