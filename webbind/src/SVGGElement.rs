use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGGElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGGElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGGElement {
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
impl core::ops::Deref for SVGGElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGGElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGGElement> for emlite::Val {
    fn from(s: SVGGElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
