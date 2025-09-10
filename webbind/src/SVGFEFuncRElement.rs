use super::*;

/// The SVGFEFuncRElement class.
/// [`SVGFEFuncRElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFuncRElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEFuncRElement {
    inner: SVGComponentTransferFunctionElement,
}

impl FromVal for SVGFEFuncRElement {
    fn from_val(v: &Any) -> Self {
        SVGFEFuncRElement {
            inner: SVGComponentTransferFunctionElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGFEFuncRElement {
    type Target = SVGComponentTransferFunctionElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEFuncRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEFuncRElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEFuncRElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEFuncRElement> for Any {
    fn from(s: SVGFEFuncRElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEFuncRElement> for Any {
    fn from(s: &SVGFEFuncRElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEFuncRElement);
