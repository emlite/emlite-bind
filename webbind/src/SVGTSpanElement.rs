use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGTSpanElement {
    inner: SVGTextPositioningElement,
}
impl FromVal for SVGTSpanElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGTSpanElement {
            inner: SVGTextPositioningElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGTSpanElement {
    type Target = SVGTextPositioningElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGTSpanElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGTSpanElement> for emlite::Val {
    fn from(s: SVGTSpanElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
