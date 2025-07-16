use super::*;

/// The SVGFEFuncAElement class.
/// [`SVGFEFuncAElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFuncAElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEFuncAElement {
    inner: SVGComponentTransferFunctionElement,
}
impl FromVal for SVGFEFuncAElement {
    fn from_val(v: &Any) -> Self {
        SVGFEFuncAElement {
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
impl core::ops::Deref for SVGFEFuncAElement {
    type Target = SVGComponentTransferFunctionElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEFuncAElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFEFuncAElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFEFuncAElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFEFuncAElement> for Any {
    fn from(s: SVGFEFuncAElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFEFuncAElement> for Any {
    fn from(s: &SVGFEFuncAElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEFuncAElement);
