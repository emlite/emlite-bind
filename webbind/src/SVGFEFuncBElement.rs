use super::*;

/// The SVGFEFuncBElement class.
/// [`SVGFEFuncBElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFuncBElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEFuncBElement {
    inner: SVGComponentTransferFunctionElement,
}

impl FromVal for SVGFEFuncBElement {
    fn from_val(v: &Any) -> Self {
        SVGFEFuncBElement {
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

impl core::ops::Deref for SVGFEFuncBElement {
    type Target = SVGComponentTransferFunctionElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEFuncBElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEFuncBElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEFuncBElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEFuncBElement> for Any {
    fn from(s: SVGFEFuncBElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEFuncBElement> for Any {
    fn from(s: &SVGFEFuncBElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEFuncBElement);
