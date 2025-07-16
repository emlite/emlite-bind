use super::*;

/// The SVGAnimateTransformElement class.
/// [`SVGAnimateTransformElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimateTransformElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimateTransformElement {
    inner: SVGAnimationElement,
}
impl FromVal for SVGAnimateTransformElement {
    fn from_val(v: &Any) -> Self {
        SVGAnimateTransformElement {
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
impl core::ops::Deref for SVGAnimateTransformElement {
    type Target = SVGAnimationElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimateTransformElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAnimateTransformElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAnimateTransformElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAnimateTransformElement> for Any {
    fn from(s: SVGAnimateTransformElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAnimateTransformElement> for Any {
    fn from(s: &SVGAnimateTransformElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimateTransformElement);
