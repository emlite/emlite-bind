use super::*;

#[derive(Clone, Debug)]
pub struct SVGDiscardElement {
    inner: SVGAnimationElement,
}
impl FromVal for SVGDiscardElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGDiscardElement {
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
impl std::ops::Deref for SVGDiscardElement {
    type Target = SVGAnimationElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGDiscardElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGDiscardElement> for emlite::Val {
    fn from(s: SVGDiscardElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
