use super::*;

#[derive(Clone, Debug)]
pub struct SVGAnimateTransformElement {
    inner: SVGAnimationElement,
}
impl FromVal for SVGAnimateTransformElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimateTransformElement {
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
impl std::ops::Deref for SVGAnimateTransformElement {
    type Target = SVGAnimationElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAnimateTransformElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimateTransformElement> for emlite::Val {
    fn from(s: SVGAnimateTransformElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
