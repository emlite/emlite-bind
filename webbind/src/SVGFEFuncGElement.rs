use super::*;

/// The SVGFEFuncGElement class.
/// [`SVGFEFuncGElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFuncGElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEFuncGElement {
    inner: SVGComponentTransferFunctionElement,
}

impl FromVal for SVGFEFuncGElement {
    fn from_val(v: &Any) -> Self {
        SVGFEFuncGElement {
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

impl core::ops::Deref for SVGFEFuncGElement {
    type Target = SVGComponentTransferFunctionElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEFuncGElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEFuncGElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEFuncGElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEFuncGElement> for Any {
    fn from(s: SVGFEFuncGElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEFuncGElement> for Any {
    fn from(s: &SVGFEFuncGElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEFuncGElement);
