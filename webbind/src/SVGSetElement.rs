use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGSetElement {
    inner: SVGAnimationElement,
}
impl FromVal for SVGSetElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGSetElement {
            inner: SVGAnimationElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGSetElement {
    type Target = SVGAnimationElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGSetElement> for emlite::Val {
    fn from(s: SVGSetElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
