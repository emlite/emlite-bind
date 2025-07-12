use super::*;

#[derive(Clone, Debug)]
pub struct SVGFEFuncRElement {
    inner: SVGComponentTransferFunctionElement,
}
impl FromVal for SVGFEFuncRElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEFuncRElement {
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
impl std::ops::Deref for SVGFEFuncRElement {
    type Target = SVGComponentTransferFunctionElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGFEFuncRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFEFuncRElement> for emlite::Val {
    fn from(s: SVGFEFuncRElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
