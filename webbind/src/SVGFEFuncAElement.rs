use super::*;

#[derive(Clone, Debug)]
pub struct SVGFEFuncAElement {
    inner: SVGComponentTransferFunctionElement,
}
impl FromVal for SVGFEFuncAElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEFuncAElement {
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
impl std::ops::Deref for SVGFEFuncAElement {
    type Target = SVGComponentTransferFunctionElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGFEFuncAElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFEFuncAElement> for emlite::Val {
    fn from(s: SVGFEFuncAElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
