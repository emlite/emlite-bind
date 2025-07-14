use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGFEDistantLightElement {
    inner: SVGElement,
}
impl FromVal for SVGFEDistantLightElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEDistantLightElement {
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
impl core::ops::Deref for SVGFEDistantLightElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEDistantLightElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFEDistantLightElement> for emlite::Val {
    fn from(s: SVGFEDistantLightElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGFEDistantLightElement {
    pub fn azimuth(&self) -> SVGAnimatedNumber {
        self.inner.get("azimuth").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDistantLightElement {
    pub fn elevation(&self) -> SVGAnimatedNumber {
        self.inner.get("elevation").as_::<SVGAnimatedNumber>()
    }
}
