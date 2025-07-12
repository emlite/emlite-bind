use super::*;

#[derive(Clone, Debug)]
pub struct SVGAnimateMotionElement {
    inner: SVGAnimationElement,
}
impl FromVal for SVGAnimateMotionElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimateMotionElement {
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
impl std::ops::Deref for SVGAnimateMotionElement {
    type Target = SVGAnimationElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAnimateMotionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimateMotionElement> for emlite::Val {
    fn from(s: SVGAnimateMotionElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
