use super::*;

/// The SVGDiscardElement class.
/// [`SVGDiscardElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGDiscardElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGDiscardElement {
    inner: SVGAnimationElement,
}

impl FromVal for SVGDiscardElement {
    fn from_val(v: &Any) -> Self {
        SVGDiscardElement {
            inner: SVGAnimationElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGDiscardElement {
    type Target = SVGAnimationElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGDiscardElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGDiscardElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGDiscardElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGDiscardElement> for Any {
    fn from(s: SVGDiscardElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGDiscardElement> for Any {
    fn from(s: &SVGDiscardElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGDiscardElement);
