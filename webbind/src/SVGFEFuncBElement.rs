use super::*;

#[derive(Clone, Debug)]
pub struct SVGFEFuncBElement {
    inner: SVGComponentTransferFunctionElement,
}
impl FromVal for SVGFEFuncBElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEFuncBElement {
            inner: SVGComponentTransferFunctionElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SVGFEFuncBElement {
    type Target = SVGComponentTransferFunctionElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGFEFuncBElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFEFuncBElement> for emlite::Val {
    fn from(s: SVGFEFuncBElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
